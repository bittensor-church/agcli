use agcli::admin;
use agcli::chain::subxt::dynamic::Value;
use agcli::chain::subxt::ext::sp_core::sr25519;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3_async_runtimes::tokio::future_into_py;

use crate::client::PyClient;
use crate::errors::{map_error, ValidationError};
use crate::types::netuid_from_py;
use crate::wallet::PyWallet;

fn validation_error(message: impl Into<String>) -> PyErr {
    let message = message.into();
    Python::with_gil(|py| {
        let py_err = ValidationError::new_err(message);
        let value = py_err.value(py);
        let _ = value.setattr("code", agcli::error::exit_code::VALIDATION);
        py_err
    })
}

fn wallet_coldkey_pair(wallet: &agcli::Wallet) -> PyResult<sr25519::Pair> {
    wallet.coldkey().map(|p| p.clone()).map_err(map_error)
}

fn parse_netuid_u16(value: &Bound<'_, PyAny>, field: &str) -> PyResult<u16> {
    let netuid = netuid_from_py(value)
        .map_err(|_| validation_error(format!("{field} must be an integer or NetUid instance")))?;
    Ok(netuid.as_u16())
}

macro_rules! admin_call_subnet {
    ($fn_name:ident, $rust_fn:ident, $value_ty:ty) => {
        #[pyfunction]
        #[allow(clippy::too_many_arguments)]
        fn $fn_name<'py>(
            py: Python<'py>,
            client: PyRef<'_, PyClient>,
            wallet: PyRef<'_, PyWallet>,
            netuid: Bound<'_, PyAny>,
            value: $value_ty,
        ) -> PyResult<Bound<'py, PyAny>> {
            let netuid = parse_netuid_u16(&netuid, "netuid")?;
            let client = client.shared_client();
            let wallet = wallet.shared_wallet();
            future_into_py(py, async move {
                let client = client.lock().await;
                let wallet = wallet.lock().await;
                let pair = wallet_coldkey_pair(&wallet)?;
                admin::$rust_fn(&client, &pair, netuid, value)
                    .await
                    .map_err(map_error)
            })
        }
    };
}

macro_rules! admin_call_global {
    ($fn_name:ident, $rust_fn:ident, $value_ty:ty) => {
        #[pyfunction]
        fn $fn_name<'py>(
            py: Python<'py>,
            client: PyRef<'_, PyClient>,
            wallet: PyRef<'_, PyWallet>,
            value: $value_ty,
        ) -> PyResult<Bound<'py, PyAny>> {
            let client = client.shared_client();
            let wallet = wallet.shared_wallet();
            future_into_py(py, async move {
                let client = client.lock().await;
                let wallet = wallet.lock().await;
                let pair = wallet_coldkey_pair(&wallet)?;
                admin::$rust_fn(&client, &pair, value)
                    .await
                    .map_err(map_error)
            })
        }
    };
}

admin_call_subnet!(set_tempo, set_tempo, u16);
admin_call_subnet!(set_max_allowed_validators, set_max_allowed_validators, u16);
admin_call_subnet!(set_max_allowed_uids, set_max_allowed_uids, u16);
admin_call_subnet!(set_immunity_period, set_immunity_period, u16);
admin_call_subnet!(set_min_allowed_weights, set_min_allowed_weights, u16);
admin_call_subnet!(set_max_weight_limit, set_max_weight_limit, u16);
admin_call_subnet!(set_weights_set_rate_limit, set_weights_set_rate_limit, u64);
admin_call_subnet!(
    set_commit_reveal_weights_enabled,
    set_commit_reveal_weights_enabled,
    bool
);
admin_call_subnet!(set_difficulty, set_difficulty, u64);
admin_call_subnet!(set_bonds_moving_average, set_bonds_moving_average, u64);
admin_call_subnet!(
    set_target_registrations_per_interval,
    set_target_registrations_per_interval,
    u16
);
admin_call_subnet!(set_activity_cutoff, set_activity_cutoff, u16);
admin_call_subnet!(set_serving_rate_limit, set_serving_rate_limit, u64);
admin_call_subnet!(set_min_difficulty, set_min_difficulty, u64);
admin_call_subnet!(set_max_difficulty, set_max_difficulty, u64);
admin_call_subnet!(set_adjustment_interval, set_adjustment_interval, u16);
admin_call_subnet!(set_adjustment_alpha, set_adjustment_alpha, u64);
admin_call_subnet!(set_kappa, set_kappa, u16);
admin_call_subnet!(set_rho, set_rho, u16);
admin_call_subnet!(set_min_burn, set_min_burn, u64);
admin_call_subnet!(set_max_burn, set_max_burn, u64);
admin_call_subnet!(set_liquid_alpha_enabled, set_liquid_alpha_enabled, bool);
admin_call_subnet!(set_yuma3_enabled, set_yuma3_enabled, bool);
admin_call_subnet!(set_bonds_penalty, set_bonds_penalty, u16);
admin_call_subnet!(set_mechanism_count, set_mechanism_count, u16);
admin_call_subnet!(
    set_network_registration_allowed,
    set_network_registration_allowed,
    bool
);
admin_call_subnet!(
    set_network_pow_registration_allowed,
    set_network_pow_registration_allowed,
    bool
);

admin_call_global!(set_default_take, set_default_take, u16);
admin_call_global!(set_tx_rate_limit, set_tx_rate_limit, u64);
admin_call_global!(set_subnet_moving_alpha, set_subnet_moving_alpha, u64);
admin_call_global!(set_stake_threshold, set_stake_threshold, u64);
admin_call_global!(
    set_nominator_min_required_stake,
    set_nominator_min_required_stake,
    u64
);

#[pyfunction]
fn set_alpha_values<'py>(
    py: Python<'py>,
    client: PyRef<'_, PyClient>,
    wallet: PyRef<'_, PyWallet>,
    netuid: Bound<'_, PyAny>,
    alpha_low: u16,
    alpha_high: u16,
) -> PyResult<Bound<'py, PyAny>> {
    let netuid = parse_netuid_u16(&netuid, "netuid")?;
    let client = client.shared_client();
    let wallet = wallet.shared_wallet();
    future_into_py(py, async move {
        let client = client.lock().await;
        let wallet = wallet.lock().await;
        let pair = wallet_coldkey_pair(&wallet)?;
        admin::set_alpha_values(&client, &pair, netuid, alpha_low, alpha_high)
            .await
            .map_err(map_error)
    })
}

#[pyfunction]
fn set_mechanism_emission_split<'py>(
    py: Python<'py>,
    client: PyRef<'_, PyClient>,
    wallet: PyRef<'_, PyWallet>,
    netuid: Bound<'_, PyAny>,
    split: Vec<u64>,
) -> PyResult<Bound<'py, PyAny>> {
    let netuid = parse_netuid_u16(&netuid, "netuid")?;
    let client = client.shared_client();
    let wallet = wallet.shared_wallet();
    future_into_py(py, async move {
        let client = client.lock().await;
        let wallet = wallet.lock().await;
        let pair = wallet_coldkey_pair(&wallet)?;
        admin::set_mechanism_emission_split(&client, &pair, netuid, split)
            .await
            .map_err(map_error)
    })
}

fn parse_admin_args(values: Bound<'_, PyAny>) -> PyResult<Vec<Value>> {
    let list: Vec<Bound<'_, PyAny>> = values.extract().map_err(|_| {
        validation_error("args must be a sequence of integers, booleans, or strings")
    })?;
    let mut out = Vec::with_capacity(list.len());
    for (idx, item) in list.into_iter().enumerate() {
        if let Ok(b) = item.extract::<bool>() {
            out.push(Value::bool(b));
            continue;
        }
        if let Ok(n) = item.extract::<i128>() {
            if n < 0 {
                return Err(validation_error(format!(
                    "args[{idx}]={n} is negative; only non-negative values are supported"
                )));
            }
            out.push(Value::u128(n as u128));
            continue;
        }
        if let Ok(n) = item.extract::<u128>() {
            out.push(Value::u128(n));
            continue;
        }
        if let Ok(s) = item.extract::<String>() {
            out.push(Value::string(s));
            continue;
        }
        return Err(validation_error(format!(
            "args[{idx}] has unsupported type; pass int, bool, or str"
        )));
    }
    Ok(out)
}

#[pyfunction]
fn raw_admin_call<'py>(
    py: Python<'py>,
    client: PyRef<'_, PyClient>,
    wallet: PyRef<'_, PyWallet>,
    call_name: String,
    args: Bound<'_, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let parsed = parse_admin_args(args)?;
    let client = client.shared_client();
    let wallet = wallet.shared_wallet();
    future_into_py(py, async move {
        let client = client.lock().await;
        let wallet = wallet.lock().await;
        let pair = wallet_coldkey_pair(&wallet)?;
        admin::raw_admin_call(&client, &pair, &call_name, parsed)
            .await
            .map_err(map_error)
    })
}

#[pyfunction]
fn known_params() -> Vec<(String, String, Vec<String>)> {
    admin::known_params()
        .into_iter()
        .map(|(name, desc, args)| {
            (
                name.to_string(),
                desc.to_string(),
                args.iter().map(|s| s.to_string()).collect(),
            )
        })
        .collect()
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_tempo, m)?)?;
    m.add_function(wrap_pyfunction!(set_max_allowed_validators, m)?)?;
    m.add_function(wrap_pyfunction!(set_max_allowed_uids, m)?)?;
    m.add_function(wrap_pyfunction!(set_immunity_period, m)?)?;
    m.add_function(wrap_pyfunction!(set_min_allowed_weights, m)?)?;
    m.add_function(wrap_pyfunction!(set_max_weight_limit, m)?)?;
    m.add_function(wrap_pyfunction!(set_weights_set_rate_limit, m)?)?;
    m.add_function(wrap_pyfunction!(set_commit_reveal_weights_enabled, m)?)?;
    m.add_function(wrap_pyfunction!(set_difficulty, m)?)?;
    m.add_function(wrap_pyfunction!(set_bonds_moving_average, m)?)?;
    m.add_function(wrap_pyfunction!(set_target_registrations_per_interval, m)?)?;
    m.add_function(wrap_pyfunction!(set_activity_cutoff, m)?)?;
    m.add_function(wrap_pyfunction!(set_serving_rate_limit, m)?)?;
    m.add_function(wrap_pyfunction!(set_default_take, m)?)?;
    m.add_function(wrap_pyfunction!(set_tx_rate_limit, m)?)?;
    m.add_function(wrap_pyfunction!(set_min_difficulty, m)?)?;
    m.add_function(wrap_pyfunction!(set_max_difficulty, m)?)?;
    m.add_function(wrap_pyfunction!(set_adjustment_interval, m)?)?;
    m.add_function(wrap_pyfunction!(set_adjustment_alpha, m)?)?;
    m.add_function(wrap_pyfunction!(set_kappa, m)?)?;
    m.add_function(wrap_pyfunction!(set_rho, m)?)?;
    m.add_function(wrap_pyfunction!(set_min_burn, m)?)?;
    m.add_function(wrap_pyfunction!(set_max_burn, m)?)?;
    m.add_function(wrap_pyfunction!(set_liquid_alpha_enabled, m)?)?;
    m.add_function(wrap_pyfunction!(set_alpha_values, m)?)?;
    m.add_function(wrap_pyfunction!(set_yuma3_enabled, m)?)?;
    m.add_function(wrap_pyfunction!(set_bonds_penalty, m)?)?;
    m.add_function(wrap_pyfunction!(set_subnet_moving_alpha, m)?)?;
    m.add_function(wrap_pyfunction!(set_mechanism_count, m)?)?;
    m.add_function(wrap_pyfunction!(set_mechanism_emission_split, m)?)?;
    m.add_function(wrap_pyfunction!(set_stake_threshold, m)?)?;
    m.add_function(wrap_pyfunction!(set_nominator_min_required_stake, m)?)?;
    m.add_function(wrap_pyfunction!(set_network_registration_allowed, m)?)?;
    m.add_function(wrap_pyfunction!(set_network_pow_registration_allowed, m)?)?;
    m.add_function(wrap_pyfunction!(raw_admin_call, m)?)?;
    m.add_function(wrap_pyfunction!(known_params, m)?)?;
    Ok(())
}
