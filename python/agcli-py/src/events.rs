use std::str::FromStr;
use std::sync::Arc;

use agcli::chain::subxt::events::Phase;
use agcli::chain::subxt::ext::scale_value::{Composite, Primitive, Value, ValueDef};
use agcli::chain::subxt::ext::sp_core::crypto::Ss58Codec;
use agcli::chain::subxt::ext::sp_core::sr25519;
use agcli::chain::subxt::OnlineClient;
use agcli::events::EventFilter as RustEventFilter;
use agcli::SubtensorConfig;
use pyo3::exceptions::PyStopAsyncIteration;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use pyo3_async_runtimes::tokio::{future_into_py, get_runtime};
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;

use crate::client::PyClient;
use crate::errors::{map_error, ValidationError};

#[derive(Clone, Debug)]
struct ParsedFilter {
    category: RustEventFilter,
    netuid: Option<u16>,
    account: Option<String>,
}

impl ParsedFilter {
    fn matches(&self, pallet: &str, variant: &str) -> bool {
        filter_matches(&self.category, pallet, variant)
    }
}

fn filter_matches(filter: &RustEventFilter, pallet: &str, variant: &str) -> bool {
    use RustEventFilter::*;
    match filter {
        All => true,
        Staking => pallet == "SubtensorModule" && STAKING_VARIANTS.iter().any(|v| *v == variant),
        Registration => {
            pallet == "SubtensorModule" && REGISTRATION_VARIANTS.iter().any(|v| *v == variant)
        }
        Transfer => pallet == "Balances",
        Weights => pallet == "SubtensorModule" && WEIGHT_VARIANTS.iter().any(|v| *v == variant),
        Subnet => pallet == "SubtensorModule" && SUBNET_VARIANTS.iter().any(|v| *v == variant),
        Delegation => {
            pallet == "SubtensorModule" && DELEGATION_VARIANTS.iter().any(|v| *v == variant)
        }
        Keys => pallet == "SubtensorModule" && KEY_VARIANTS.iter().any(|v| *v == variant),
        Swap => pallet == "Swap" && SWAP_VARIANTS.iter().any(|v| *v == variant),
        Governance => GOVERNANCE_VARIANTS
            .iter()
            .any(|(p, v)| *p == pallet && *v == variant),
        Crowdloan => pallet == "Crowdloan" && CROWDLOAN_VARIANTS.iter().any(|v| *v == variant),
    }
}

// Kept in sync with `agcli::events` (the upstream lists are private). The
// binding boundary is the right layer to mirror them so we don't import
// internal modules.
const STAKING_VARIANTS: &[&str] = &[
    "StakeAdded",
    "StakeRemoved",
    "StakeMoved",
    "StakeSwapped",
    "AllStakeRemoved",
    "StakeTransferred",
    "AlphaRecycled",
    "AlphaBurned",
    "RootClaimed",
    "AutoStakeAdded",
    "AutoStakeDestinationSet",
];

const REGISTRATION_VARIANTS: &[&str] = &[
    "NeuronRegistered",
    "BurnedRegister",
    "SubnetRegistered",
    "PowRegistered",
    "BulkNeuronsRegistered",
];

const WEIGHT_VARIANTS: &[&str] = &[
    "WeightsSet",
    "WeightsCommitted",
    "WeightsRevealed",
    "WeightsBatchRevealed",
    "CRV3WeightsCommitted",
    "CRV3WeightsRevealed",
    "TimelockedWeightsCommitted",
    "TimelockedWeightsRevealed",
    "BatchWeightsCompleted",
    "BatchCompletedWithErrors",
    "BatchWeightItemFailed",
    "CommitRevealEnabled",
    "CommitRevealPeriodsSet",
];

const SUBNET_VARIANTS: &[&str] = &[
    "SubnetHyperparamsSet",
    "SubnetIdentitySet",
    "SubnetIdentityRemoved",
    "NetworkAdded",
    "NetworkRemoved",
    "TempoSet",
    "DissolveNetworkScheduled",
    "SubnetLeaseCreated",
    "SubnetLeaseTerminated",
    "SubnetLeaseDividendsDistributed",
    "SymbolUpdated",
    "FirstEmissionBlockNumberSet",
    "TransferToggle",
    "SubnetOwnerHotkeySet",
];

const DELEGATION_VARIANTS: &[&str] = &[
    "DelegateAdded",
    "TakeDecreased",
    "TakeIncreased",
    "ChildKeyTakeSet",
    "SetChildren",
    "SetChildrenScheduled",
];

const KEY_VARIANTS: &[&str] = &[
    "HotkeySwapped",
    "HotkeySwappedOnSubnet",
    "ColdkeySwapped",
    "ColdkeySwapScheduled",
    "EvmKeyAssociated",
    "ChainIdentitySet",
];

const SWAP_VARIANTS: &[&str] = &[
    "SwapExecuted",
    "LiquidityAdded",
    "LiquidityRemoved",
    "PositionCreated",
    "PositionClosed",
    "FeesCollected",
];

const GOVERNANCE_VARIANTS: &[(&str, &str)] = &[
    ("SafeMode", "Entered"),
    ("SafeMode", "Exited"),
    ("SafeMode", "DepositPlaced"),
    ("SafeMode", "DepositReleased"),
    ("Sudo", "Sudid"),
    ("Sudo", "KeyChanged"),
    ("Sudo", "KeyRotated"),
    ("Sudo", "SudoAsDone"),
    ("Scheduler", "Scheduled"),
    ("Scheduler", "Canceled"),
    ("Scheduler", "Dispatched"),
    ("Proxy", "ProxyExecuted"),
    ("Proxy", "PureCreated"),
    ("Proxy", "Announced"),
    ("Proxy", "ProxyAdded"),
    ("Proxy", "ProxyRemoved"),
    ("Multisig", "NewMultisig"),
    ("Multisig", "MultisigApproval"),
    ("Multisig", "MultisigExecuted"),
    ("Multisig", "MultisigCancelled"),
];

const CROWDLOAN_VARIANTS: &[&str] = &[
    "Created",
    "Contributed",
    "Withdrew",
    "PartiallyRefunded",
    "AllRefunded",
    "Dissolved",
    "Edited",
];

fn validation_error(message: impl Into<String>) -> PyErr {
    let message = message.into();
    Python::with_gil(|py| {
        let py_err = ValidationError::new_err(message);
        let value = py_err.value(py);
        let _ = value.setattr("code", agcli::error::exit_code::VALIDATION);
        py_err
    })
}

fn category_from_str(s: &str) -> RustEventFilter {
    RustEventFilter::from_str(s).unwrap_or(RustEventFilter::All)
}

fn extract_str_attr(value: &Bound<'_, PyAny>, name: &str) -> PyResult<Option<String>> {
    if value.hasattr(name)? {
        let attr = value.getattr(name)?;
        if attr.is_none() {
            return Ok(None);
        }
        return Ok(Some(attr.extract::<String>().map_err(|_| {
            validation_error(format!("filter.{name} must be a string"))
        })?));
    }
    if let Ok(item) = value.get_item(name) {
        if item.is_none() {
            return Ok(None);
        }
        return Ok(Some(item.extract::<String>().map_err(|_| {
            validation_error(format!("filter['{name}'] must be a string"))
        })?));
    }
    Ok(None)
}

fn extract_u16_attr(value: &Bound<'_, PyAny>, name: &str) -> PyResult<Option<u16>> {
    if value.hasattr(name)? {
        let attr = value.getattr(name)?;
        if attr.is_none() {
            return Ok(None);
        }
        return Ok(Some(attr.extract::<u16>().map_err(|_| {
            validation_error(format!("filter.{name} must fit in u16"))
        })?));
    }
    if let Ok(item) = value.get_item(name) {
        if item.is_none() {
            return Ok(None);
        }
        return Ok(Some(item.extract::<u16>().map_err(|_| {
            validation_error(format!("filter['{name}'] must fit in u16"))
        })?));
    }
    Ok(None)
}

fn parse_filter(value: Option<&Bound<'_, PyAny>>) -> PyResult<ParsedFilter> {
    let Some(value) = value else {
        return Ok(ParsedFilter {
            category: RustEventFilter::All,
            netuid: None,
            account: None,
        });
    };
    if value.is_none() {
        return Ok(ParsedFilter {
            category: RustEventFilter::All,
            netuid: None,
            account: None,
        });
    }
    if let Ok(s) = value.extract::<String>() {
        return Ok(ParsedFilter {
            category: category_from_str(&s),
            netuid: None,
            account: None,
        });
    }
    let category_name = extract_str_attr(value, "category")?.unwrap_or_else(|| "all".to_string());
    let netuid = extract_u16_attr(value, "netuid")?;
    let account = extract_str_attr(value, "account")?;
    if let Some(acc) = account.as_deref() {
        if agcli::Client::ss58_to_account_id_pub(acc).is_err() {
            return Err(validation_error(format!(
                "filter.account is not a valid SS58 address: {acc}"
            )));
        }
    }
    Ok(ParsedFilter {
        category: category_from_str(&category_name),
        netuid,
        account,
    })
}

fn extract_netuid(composite: &Composite<u32>) -> Option<u16> {
    if let Composite::Named(fields) = composite {
        for (name, val) in fields {
            if name == "netuid" {
                if let ValueDef::Primitive(Primitive::U128(n)) = &val.value {
                    if *n <= u16::MAX as u128 {
                        return Some(*n as u16);
                    }
                    return None;
                }
            }
        }
    }
    None
}

fn try_composite_as_ss58(composite: &Composite<u32>) -> Option<String> {
    let fields = match composite {
        Composite::Unnamed(fields) if fields.len() == 32 => fields,
        _ => return None,
    };
    let mut bytes = [0u8; 32];
    for (i, field) in fields.iter().enumerate() {
        match &field.value {
            ValueDef::Primitive(Primitive::U128(n)) if *n <= 255 => {
                bytes[i] = *n as u8;
            }
            _ => return None,
        }
    }
    let public = sr25519::Public::from_raw(bytes);
    Some(public.to_ss58check())
}

fn extract_accounts_from_value(val: &ValueDef<u32>, out: &mut Vec<String>) {
    match val {
        ValueDef::Composite(inner) => {
            if let Some(ss58) = try_composite_as_ss58(inner) {
                out.push(ss58);
            } else {
                match inner {
                    Composite::Named(fields) => {
                        for (_, v) in fields {
                            extract_accounts_from_value(&v.value, out);
                        }
                    }
                    Composite::Unnamed(fields) => {
                        for v in fields {
                            extract_accounts_from_value(&v.value, out);
                        }
                    }
                }
            }
        }
        ValueDef::Variant(variant) => match &variant.values {
            Composite::Named(fields) => {
                for (_, v) in fields {
                    extract_accounts_from_value(&v.value, out);
                }
            }
            Composite::Unnamed(fields) => {
                for v in fields {
                    extract_accounts_from_value(&v.value, out);
                }
            }
        },
        _ => {}
    }
}

fn extract_accounts(composite: &Composite<u32>) -> Vec<String> {
    let mut accounts = Vec::new();
    match composite {
        Composite::Named(fields) => {
            for (_, val) in fields {
                extract_accounts_from_value(&val.value, &mut accounts);
            }
        }
        Composite::Unnamed(fields) => {
            for val in fields {
                extract_accounts_from_value(&val.value, &mut accounts);
            }
        }
    }
    accounts
}

fn value_to_json(val: &Value<u32>) -> serde_json::Value {
    match &val.value {
        ValueDef::Primitive(p) => match p {
            Primitive::Bool(b) => serde_json::Value::Bool(*b),
            Primitive::Char(c) => serde_json::Value::String(c.to_string()),
            Primitive::U128(n) => {
                if *n <= u64::MAX as u128 {
                    serde_json::json!(*n as u64)
                } else {
                    serde_json::Value::String(n.to_string())
                }
            }
            Primitive::I128(n) => {
                if *n >= i64::MIN as i128 && *n <= i64::MAX as i128 {
                    serde_json::json!(*n as i64)
                } else {
                    serde_json::Value::String(n.to_string())
                }
            }
            Primitive::U256(n) => serde_json::Value::String(format!("{:?}", n)),
            Primitive::I256(n) => serde_json::Value::String(format!("{:?}", n)),
            Primitive::String(s) => serde_json::Value::String(s.clone()),
        },
        ValueDef::Composite(composite) => composite_to_json(composite),
        ValueDef::Variant(variant) => {
            let inner = composite_to_json(&variant.values);
            serde_json::json!({ &variant.name: inner })
        }
        ValueDef::BitSequence(bits) => serde_json::Value::String(format!("bits({})", bits.len())),
    }
}

fn composite_to_json(composite: &Composite<u32>) -> serde_json::Value {
    match composite {
        Composite::Named(fields) => {
            let map: serde_json::Map<String, serde_json::Value> = fields
                .iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect();
            serde_json::Value::Object(map)
        }
        Composite::Unnamed(fields) => {
            if fields.len() == 32 || fields.len() == 64 {
                let bytes: Vec<u8> = fields
                    .iter()
                    .filter_map(|v| match &v.value {
                        ValueDef::Primitive(Primitive::U128(n)) if *n <= 255 => Some(*n as u8),
                        _ => None,
                    })
                    .collect();
                if bytes.len() == fields.len() {
                    return serde_json::Value::String(format!("0x{}", hex::encode(&bytes)));
                }
            }
            let arr: Vec<serde_json::Value> = fields.iter().map(value_to_json).collect();
            serde_json::Value::Array(arr)
        }
    }
}

fn json_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    pythonize::pythonize(py, value)
        .map(|b| b.unbind())
        .map_err(|e| map_error(anyhow::anyhow!("{e}")))
}

fn build_event_dict(
    py: Python<'_>,
    block_number: u64,
    pallet: &str,
    variant: &str,
    extrinsic_index: Option<u32>,
    fields: &serde_json::Value,
) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("block_number", block_number)?;
    dict.set_item("pallet", pallet)?;
    dict.set_item("variant", variant)?;
    dict.set_item("extrinsic_index", extrinsic_index)?;
    dict.set_item("fields", json_to_py(py, fields)?)?;
    Ok(dict.unbind().into())
}

fn build_block_dict(
    py: Python<'_>,
    block_number: u64,
    block_hash: &str,
    extrinsic_count: usize,
) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    dict.set_item("block_number", block_number)?;
    dict.set_item("hash", block_hash)?;
    dict.set_item("extrinsics", extrinsic_count)?;
    Ok(dict.unbind().into())
}

const STREAM_CHANNEL_CAPACITY: usize = 64;

#[pyclass(name = "EventStream", module = "agcli._agcli")]
pub struct PyEventStream {
    receiver: Arc<Mutex<Option<mpsc::Receiver<PyObject>>>>,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl Drop for PyEventStream {
    fn drop(&mut self) {
        let handle = Arc::clone(&self.handle);
        get_runtime().spawn(async move {
            let mut guard = handle.lock().await;
            if let Some(h) = guard.take() {
                h.abort();
            }
        });
    }
}

#[pymethods]
impl PyEventStream {
    fn __aiter__<'py>(slf: PyRef<'py, Self>) -> PyRef<'py, Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let rx = Arc::clone(&self.receiver);
        future_into_py(py, async move {
            let mut guard = rx.lock().await;
            let Some(receiver) = guard.as_mut() else {
                return Err(PyStopAsyncIteration::new_err("stream is closed"));
            };
            match receiver.recv().await {
                Some(obj) => Ok(obj),
                None => Err(PyStopAsyncIteration::new_err("stream closed")),
            }
        })
    }

    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let rx = Arc::clone(&self.receiver);
        let handle = Arc::clone(&self.handle);
        future_into_py(py, async move {
            let mut rx = rx.lock().await;
            *rx = None;
            let mut handle = handle.lock().await;
            if let Some(h) = handle.take() {
                h.abort();
            }
            Ok(())
        })
    }
}

fn spawn_event_subscription(
    subxt_client: OnlineClient<SubtensorConfig>,
    filter: ParsedFilter,
) -> (mpsc::Receiver<PyObject>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<PyObject>(STREAM_CHANNEL_CAPACITY);
    let handle = get_runtime().spawn(async move {
        let mut sub = match subxt_client.blocks().subscribe_finalized().await {
            Ok(s) => s,
            Err(_) => return,
        };
        while let Some(block_result) = sub.next().await {
            let block = match block_result {
                Ok(b) => b,
                Err(_) => break,
            };
            let block_number = block.number() as u64;
            let events = match block.events().await {
                Ok(ev) => ev,
                Err(_) => continue,
            };
            for event in events.iter() {
                let event = match event {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let pallet = event.pallet_name().to_string();
                let variant = event.variant_name().to_string();
                if !filter.matches(&pallet, &variant) {
                    continue;
                }
                let field_values = match event.field_values() {
                    Ok(fv) => fv,
                    Err(_) => continue,
                };
                if let Some(target_netuid) = filter.netuid {
                    match extract_netuid(&field_values) {
                        Some(found) if found == target_netuid => {}
                        _ => continue,
                    }
                }
                if let Some(target_account) = filter.account.as_deref() {
                    let accounts = extract_accounts(&field_values);
                    if !accounts.iter().any(|a| a == target_account) {
                        continue;
                    }
                }
                let extrinsic_index = match event.phase() {
                    Phase::ApplyExtrinsic(i) => Some(i),
                    _ => None,
                };
                let json_fields = composite_to_json(&field_values);
                let dict = match Python::with_gil(|py| {
                    build_event_dict(
                        py,
                        block_number,
                        &pallet,
                        &variant,
                        extrinsic_index,
                        &json_fields,
                    )
                }) {
                    Ok(d) => d,
                    Err(_) => continue,
                };
                if tx.send(dict).await.is_err() {
                    return;
                }
            }
        }
    });
    (rx, handle)
}

fn spawn_block_subscription(
    subxt_client: OnlineClient<SubtensorConfig>,
) -> (mpsc::Receiver<PyObject>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<PyObject>(STREAM_CHANNEL_CAPACITY);
    let handle = get_runtime().spawn(async move {
        let mut sub = match subxt_client.blocks().subscribe_finalized().await {
            Ok(s) => s,
            Err(_) => return,
        };
        while let Some(block_result) = sub.next().await {
            let block = match block_result {
                Ok(b) => b,
                Err(_) => break,
            };
            let block_number = block.number() as u64;
            let block_hash = format!("{:?}", block.hash());
            let extrinsic_count = block.extrinsics().await.map(|e| e.len()).unwrap_or(0);
            let dict = match Python::with_gil(|py| {
                build_block_dict(py, block_number, &block_hash, extrinsic_count)
            }) {
                Ok(d) => d,
                Err(_) => continue,
            };
            if tx.send(dict).await.is_err() {
                return;
            }
        }
    });
    (rx, handle)
}

#[pymethods]
impl PyClient {
    #[pyo3(signature = (filter=None))]
    fn subscribe_events(&self, filter: Option<Bound<'_, PyAny>>) -> PyResult<PyEventStream> {
        let parsed = parse_filter(filter.as_ref())?;
        let shared = self.shared_client();
        let subxt_client = get_runtime().block_on(async move {
            let client = shared.lock().await;
            client.subxt().clone()
        });
        let (rx, handle) = spawn_event_subscription(subxt_client, parsed);
        Ok(PyEventStream {
            receiver: Arc::new(Mutex::new(Some(rx))),
            handle: Arc::new(Mutex::new(Some(handle))),
        })
    }

    fn subscribe_events_filtered(&self, filter: Bound<'_, PyAny>) -> PyResult<PyEventStream> {
        let parsed = parse_filter(Some(&filter))?;
        let shared = self.shared_client();
        let subxt_client = get_runtime().block_on(async move {
            let client = shared.lock().await;
            client.subxt().clone()
        });
        let (rx, handle) = spawn_event_subscription(subxt_client, parsed);
        Ok(PyEventStream {
            receiver: Arc::new(Mutex::new(Some(rx))),
            handle: Arc::new(Mutex::new(Some(handle))),
        })
    }

    fn subscribe_blocks(&self) -> PyResult<PyEventStream> {
        let shared = self.shared_client();
        let subxt_client = get_runtime().block_on(async move {
            let client = shared.lock().await;
            client.subxt().clone()
        });
        let (rx, handle) = spawn_block_subscription(subxt_client);
        Ok(PyEventStream {
            receiver: Arc::new(Mutex::new(Some(rx))),
            handle: Arc::new(Mutex::new(Some(handle))),
        })
    }
}
