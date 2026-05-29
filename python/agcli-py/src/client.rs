use std::sync::Arc;

use agcli::{Client, Config};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3_async_runtimes::tokio::future_into_py;
use tokio::sync::Mutex;

use crate::errors::map_error;
use crate::runtime::runtime;
use crate::types::{
    hash_to_hex, netuid_from_py, parse_hash, parse_network, to_pyobject_unbound, u64_to_u32,
    PyBalance, PyNetwork,
};

pub(crate) type SharedClient = Arc<Mutex<Client>>;

#[pyclass(name = "Client", module = "agcli._agcli")]
pub struct PyClient {
    inner: SharedClient,
}

#[pyclass(name = "ClientSync", module = "agcli._agcli")]
pub struct PyClientSync {
    inner: SharedClient,
}

fn wrap_client(client: Client) -> PyClient {
    PyClient {
        inner: Arc::new(Mutex::new(client)),
    }
}

impl PyClient {
    pub(crate) fn shared_client(&self) -> SharedClient {
        Arc::clone(&self.inner)
    }
}

fn wrap_client_sync(client: Client) -> PyClientSync {
    PyClientSync {
        inner: Arc::new(Mutex::new(client)),
    }
}

async fn block_hash_from_number(
    client: &mut Client,
    block_number: u64,
) -> anyhow::Result<agcli::Hash> {
    let block_number = u32::try_from(block_number)
        .map_err(|_| anyhow::anyhow!("block number {block_number} exceeds u32 range"))?;
    client.get_block_hash(block_number).await
}

fn serialize_weight_commits(
    commits: Option<Vec<(agcli::Hash, u64, u64, u64)>>,
) -> Option<Vec<(String, u64, u64, u64)>> {
    commits.map(|rows| {
        rows.into_iter()
            .map(|(hash, commit_block, first_reveal, last_reveal)| {
                (hash_to_hex(hash), commit_block, first_reveal, last_reveal)
            })
            .collect()
    })
}

#[pymethods]
impl PyClient {
    #[staticmethod]
    fn connect<'py>(py: Python<'py>, url: String) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, async move {
            let client = Client::connect(&url).await.map_err(map_error)?;
            Ok(wrap_client(client))
        })
    }

    #[staticmethod]
    fn connect_with_retry<'py>(py: Python<'py>, urls: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, async move {
            let refs: Vec<&str> = urls.iter().map(String::as_str).collect();
            let client = Client::connect_with_retry(&refs).await.map_err(map_error)?;
            Ok(wrap_client(client))
        })
    }

    #[staticmethod]
    fn best_connection<'py>(py: Python<'py>, urls: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, async move {
            let refs: Vec<&str> = urls.iter().map(String::as_str).collect();
            let client = Client::best_connection(&refs).await.map_err(map_error)?;
            Ok(wrap_client(client))
        })
    }

    #[staticmethod]
    #[pyo3(signature = (network=None))]
    fn connect_network<'py>(
        py: Python<'py>,
        network: Option<PyRef<'_, PyNetwork>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let network = network
            .map(|n| n.inner.clone())
            .unwrap_or(agcli::types::Network::Finney);
        future_into_py(py, async move {
            let client = Client::connect_network(&network).await.map_err(map_error)?;
            Ok(wrap_client(client))
        })
    }

    #[staticmethod]
    fn from_config<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, async move {
            let config = Config::load();
            let client = client_from_config(&config).await.map_err(map_error)?;
            Ok(wrap_client(client))
        })
    }

    #[getter]
    fn endpoint(&self) -> PyResult<String> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(client.url().to_string())
            })
            .map_err(map_error)
    }

    #[getter]
    fn network(&self) -> PyResult<PyNetwork> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(PyNetwork {
                    inner: client.network().clone(),
                })
            })
            .map_err(map_error)
    }

    fn set_dry_run(&self, enabled: bool) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let mut client = client.lock().await;
                client.set_dry_run(enabled);
                Ok(())
            })
            .map_err(map_error)
    }

    fn is_dry_run(&self) -> PyResult<bool> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(client.is_dry_run())
            })
            .map_err(map_error)
    }

    fn set_finalization_timeout(&self, timeout: u64) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let mut client = client.lock().await;
                client.set_finalization_timeout(timeout);
                Ok(())
            })
            .map_err(map_error)
    }

    fn finalization_timeout(&self) -> PyResult<u64> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(client.finalization_timeout())
            })
            .map_err(map_error)
    }

    fn set_mortality_blocks(&self, blocks: u64) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let mut client = client.lock().await;
                client.set_mortality_blocks(blocks);
                Ok(())
            })
            .map_err(map_error)
    }

    fn mortality_blocks(&self) -> PyResult<u64> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(client.mortality_blocks())
            })
            .map_err(map_error)
    }

    fn reconnect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            client.reconnect().await.map_err(map_error)?;
            Ok(())
        })
    }

    fn invalidate_cache<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client.invalidate_cache().await;
            Ok(())
        })
    }

    fn is_alive<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            Ok(client.is_alive().await)
        })
    }

    fn get_balance<'py>(&self, py: Python<'py>, address: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let balance = client.get_balance_ss58(&address).await.map_err(map_error)?;
            Ok(PyBalance::new_inner(balance))
        })
    }

    fn get_balance_at_hash<'py>(
        &self,
        py: Python<'py>,
        address: String,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let balance = client
                .get_balance_at_hash(&address, block_hash)
                .await
                .map_err(map_error)?;
            Ok(PyBalance::new_inner(balance))
        })
    }

    fn get_balance_at_block<'py>(
        &self,
        py: Python<'py>,
        address: String,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let balance = client
                .get_balance_at_block(&address, block_hash)
                .await
                .map_err(map_error)?;
            Ok(PyBalance::new_inner(balance))
        })
    }

    fn get_balances_multi<'py>(
        &self,
        py: Python<'py>,
        addresses: Vec<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let refs: Vec<String> = addresses;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let refs_borrowed: Vec<&str> = refs.iter().map(String::as_str).collect();
            let client = client.lock().await;
            let balances = client
                .get_balances_multi(&refs_borrowed)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&balances)
        })
    }

    fn get_block_hash<'py>(
        &self,
        py: Python<'py>,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_number = u64_to_u32(block_number)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let hash = client
                .get_block_hash(block_number)
                .await
                .map_err(map_error)?;
            Ok(hash_to_hex(hash))
        })
    }

    fn get_block_number<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client.get_block_number().await.map_err(map_error)
        })
    }

    fn get_finalized_block_number<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client.get_finalized_block_number().await.map_err(map_error)
        })
    }

    fn pin_latest_block<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let hash = client.pin_latest_block().await.map_err(map_error)?;
            Ok(hash_to_hex(hash))
        })
    }

    fn get_total_stake_at_block<'py>(
        &self,
        py: Python<'py>,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let balance = client
                .get_total_stake_at_block(block_hash)
                .await
                .map_err(map_error)?;
            Ok(PyBalance::new_inner(balance))
        })
    }

    fn get_block_header<'py>(
        &self,
        py: Python<'py>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let (number, hash, parent_hash, state_root) = client
                .get_block_header(block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&(
                number,
                hash_to_hex(hash),
                hash_to_hex(parent_hash),
                hash_to_hex(state_root),
            ))
        })
    }

    fn get_block_extrinsic_count<'py>(
        &self,
        py: Python<'py>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client
                .get_block_extrinsic_count(block_hash)
                .await
                .map_err(map_error)
        })
    }

    fn get_block_timestamp<'py>(
        &self,
        py: Python<'py>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client
                .get_block_timestamp(block_hash)
                .await
                .map_err(map_error)
        })
    }

    fn get_total_issuance<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client.get_total_issuance().await.map_err(map_error)?;
            Ok(PyBalance::new_inner(value))
        })
    }

    fn get_total_stake<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client.get_total_stake().await.map_err(map_error)?;
            Ok(PyBalance::new_inner(value))
        })
    }

    fn get_total_networks<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client.get_total_networks().await.map_err(map_error)
        })
    }

    fn get_block_emission<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client.get_block_emission().await.map_err(map_error)?;
            Ok(PyBalance::new_inner(value))
        })
    }

    fn get_subnet_registration_cost<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .get_subnet_registration_cost()
                .await
                .map_err(map_error)?;
            Ok(PyBalance::new_inner(value))
        })
    }

    fn get_network_overview<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client.get_network_overview().await.map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_total_issuance_at<'py>(
        &self,
        py: Python<'py>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .get_total_issuance_at(block_hash)
                .await
                .map_err(map_error)?;
            Ok(PyBalance::new_inner(value))
        })
    }

    fn get_total_stake_at<'py>(
        &self,
        py: Python<'py>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .get_total_stake_at(block_hash)
                .await
                .map_err(map_error)?;
            Ok(PyBalance::new_inner(value))
        })
    }

    fn get_total_networks_at<'py>(
        &self,
        py: Python<'py>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client
                .get_total_networks_at(block_hash)
                .await
                .map_err(map_error)
        })
    }

    fn get_block_emission_at<'py>(
        &self,
        py: Python<'py>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .get_block_emission_at(block_hash)
                .await
                .map_err(map_error)?;
            Ok(PyBalance::new_inner(value))
        })
    }

    fn get_block_number_at<'py>(
        &self,
        py: Python<'py>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client
                .get_block_number_at(block_hash)
                .await
                .map_err(map_error)
        })
    }

    fn get_all_subnets<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let subnets = client.get_all_subnets().await.map_err(map_error)?;
            to_pyobject_unbound(subnets.as_ref())
        })
    }

    fn get_subnet_info<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let info = client.get_subnet_info(netuid).await.map_err(map_error)?;
            match info {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_subnet_hyperparams<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let info = client
                .get_subnet_hyperparams(netuid)
                .await
                .map_err(map_error)?;
            match info {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_dynamic_info<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let info = client.get_dynamic_info(netuid).await.map_err(map_error)?;
            match info {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_all_dynamic_info<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let info = client.get_all_dynamic_info().await.map_err(map_error)?;
            to_pyobject_unbound(info.as_ref())
        })
    }

    fn get_metagraph<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let metagraph = client.get_metagraph(netuid).await.map_err(map_error)?;
            to_pyobject_unbound(&metagraph)
        })
    }

    fn get_neuron<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
        uid: u16,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let neuron = client.get_neuron(netuid, uid).await.map_err(map_error)?;
            match neuron {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_neurons_lite<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let neurons = client.get_neurons_lite(netuid).await.map_err(map_error)?;
            to_pyobject_unbound(neurons.as_ref())
        })
    }

    fn get_stake_for_coldkey<'py>(
        &self,
        py: Python<'py>,
        coldkey: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let stakes = client
                .get_stake_for_coldkey(&coldkey)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&stakes)
        })
    }

    fn get_stake_for_coldkey_pinned<'py>(
        &self,
        py: Python<'py>,
        coldkey: String,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let stakes = client
                .get_stake_for_coldkey_at_block(&coldkey, block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&stakes)
        })
    }

    fn get_delegates<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let delegates = client.get_delegates().await.map_err(map_error)?;
            to_pyobject_unbound(&delegates)
        })
    }

    fn get_delegate<'py>(&self, py: Python<'py>, hotkey: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let delegate = client.get_delegate(&hotkey).await.map_err(map_error)?;
            match delegate {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_identity<'py>(&self, py: Python<'py>, ss58: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let identity = client.get_identity(&ss58).await.map_err(map_error)?;
            match identity {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_subnet_identity<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let identity = client
                .get_subnet_identity(netuid)
                .await
                .map_err(map_error)?;
            match identity {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_subnet_info_pinned<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let info = client
                .get_subnet_info_at_block(netuid, block_hash)
                .await
                .map_err(map_error)?;
            match info {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_subnet_hyperparams_pinned<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let params = client
                .get_subnet_hyperparams_at_block(netuid, block_hash)
                .await
                .map_err(map_error)?;
            match params {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_identity_pinned<'py>(
        &self,
        py: Python<'py>,
        ss58: String,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let identity = client
                .get_identity_at_block(&ss58, block_hash)
                .await
                .map_err(map_error)?;
            match identity {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_subnet_identity_pinned<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let identity = client
                .get_subnet_identity_at_block(netuid, block_hash)
                .await
                .map_err(map_error)?;
            match identity {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_delegate_pinned<'py>(
        &self,
        py: Python<'py>,
        hotkey: String,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let delegate = client
                .get_delegate_at_block(&hotkey, block_hash)
                .await
                .map_err(map_error)?;
            match delegate {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn list_proxies_pinned<'py>(
        &self,
        py: Python<'py>,
        ss58: String,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let proxies = client
                .list_proxies_at_block(&ss58, block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&proxies)
        })
    }

    fn get_coldkey_swap_scheduled_pinned<'py>(
        &self,
        py: Python<'py>,
        ss58: String,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .get_coldkey_swap_scheduled_at_block(&ss58, block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_child_keys_pinned<'py>(
        &self,
        py: Python<'py>,
        hotkey_ss58: String,
        netuid: Bound<'_, PyAny>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let keys = client
                .get_child_keys_at_block(&hotkey_ss58, netuid, block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&keys)
        })
    }

    fn get_pending_child_keys_pinned<'py>(
        &self,
        py: Python<'py>,
        hotkey_ss58: String,
        netuid: Bound<'_, PyAny>,
        block_hash: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let block_hash = parse_hash(&block_hash)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let keys = client
                .get_pending_child_keys_at_block(&hotkey_ss58, netuid, block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&keys)
        })
    }

    fn get_stake_for_coldkey_at_block<'py>(
        &self,
        py: Python<'py>,
        coldkey: String,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let stakes = client
                .get_stake_for_coldkey_at_block(&coldkey, block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&stakes)
        })
    }

    fn get_identity_at_block<'py>(
        &self,
        py: Python<'py>,
        ss58: String,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let value = client
                .get_identity_at_block(&ss58, block_hash)
                .await
                .map_err(map_error)?;
            match value {
                Some(identity) => to_pyobject_unbound(&identity),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_all_subnets_at_block<'py>(
        &self,
        py: Python<'py>,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let value = client
                .get_all_subnets_at_block(block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_all_dynamic_info_at_block<'py>(
        &self,
        py: Python<'py>,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let value = client
                .get_all_dynamic_info_at_block(block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_dynamic_info_at_block<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let value = client
                .get_dynamic_info_at_block(netuid, block_hash)
                .await
                .map_err(map_error)?;
            match value {
                Some(info) => to_pyobject_unbound(&info),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_neurons_lite_at_block<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let value = client
                .get_neurons_lite_at_block(netuid, block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_neuron_at_block<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
        uid: u16,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let value = client
                .get_neuron_at_block(netuid, uid, block_hash)
                .await
                .map_err(map_error)?;
            match value {
                Some(info) => to_pyobject_unbound(&info),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_delegates_at_block<'py>(
        &self,
        py: Python<'py>,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let value = client
                .get_delegates_at_block(block_hash)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_total_issuance_at_block<'py>(
        &self,
        py: Python<'py>,
        block_number: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let mut client = client.lock().await;
            let block_hash = block_hash_from_number(&mut client, block_number)
                .await
                .map_err(map_error)?;
            let value = client
                .get_total_issuance_at_block(block_hash)
                .await
                .map_err(map_error)?;
            Ok(PyBalance::new_inner(value))
        })
    }

    fn list_proxies<'py>(&self, py: Python<'py>, ss58: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let proxies = client.list_proxies(&ss58).await.map_err(map_error)?;
            to_pyobject_unbound(&proxies)
        })
    }

    fn list_multisig_pending<'py>(
        &self,
        py: Python<'py>,
        multisig_ss58: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .list_multisig_pending(&multisig_ss58)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn list_proxy_announcements<'py>(
        &self,
        py: Python<'py>,
        ss58: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .list_proxy_announcements(&ss58)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_coldkey_swap_scheduled<'py>(
        &self,
        py: Python<'py>,
        ss58: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .get_coldkey_swap_scheduled(&ss58)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_child_keys<'py>(
        &self,
        py: Python<'py>,
        hotkey_ss58: String,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let keys = client
                .get_child_keys(&hotkey_ss58, netuid)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&keys)
        })
    }

    fn get_parent_keys<'py>(
        &self,
        py: Python<'py>,
        hotkey_ss58: String,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let keys = client
                .get_parent_keys(&hotkey_ss58, netuid)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&keys)
        })
    }

    fn get_pending_child_keys<'py>(
        &self,
        py: Python<'py>,
        hotkey_ss58: String,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let keys = client
                .get_pending_child_keys(&hotkey_ss58, netuid)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&keys)
        })
    }

    fn get_delegated<'py>(
        &self,
        py: Python<'py>,
        hotkey_ss58: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let delegated = client
                .get_delegated(&hotkey_ss58)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&delegated)
        })
    }

    fn get_weight_commits<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
        hotkey_ss58: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let commits = client
                .get_weight_commits(netuid, &hotkey_ss58)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&serialize_weight_commits(commits))
        })
    }

    fn get_all_weight_commits<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let commits = client
                .get_all_weight_commits(netuid)
                .await
                .map_err(map_error)?;
            let value: Vec<(String, Vec<(String, u64, u64, u64)>)> = commits
                .into_iter()
                .map(|(account, rows)| {
                    (
                        account.to_string(),
                        rows.into_iter()
                            .map(|(hash, commit_block, first_reveal, last_reveal)| {
                                (hash_to_hex(hash), commit_block, first_reveal, last_reveal)
                            })
                            .collect(),
                    )
                })
                .collect();
            to_pyobject_unbound(&value)
        })
    }

    fn get_reveal_period_epochs<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client
                .get_reveal_period_epochs(netuid)
                .await
                .map_err(map_error)
        })
    }

    fn get_weights_for_uid<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
        uid: u16,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .get_weights_for_uid(netuid, uid)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_all_weights<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client.get_all_weights(netuid).await.map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_commit_reveal_weights_version<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client
                .get_commit_reveal_weights_version()
                .await
                .map_err(map_error)
        })
    }

    fn get_commitment<'py>(
        &self,
        py: Python<'py>,
        netuid: u16,
        hotkey_ss58: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .get_commitment(netuid, &hotkey_ss58)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_all_commitments<'py>(
        &self,
        py: Python<'py>,
        netuid: u16,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let value = client
                .get_all_commitments(netuid)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&value)
        })
    }

    fn get_block_info_for_pow<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let (block_number, hash_bytes) =
                client.get_block_info_for_pow().await.map_err(map_error)?;
            to_pyobject_unbound(&(block_number, format!("0x{}", hex::encode(hash_bytes))))
        })
    }

    fn get_difficulty<'py>(
        &self,
        py: Python<'py>,
        netuid: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            client.get_difficulty(netuid).await.map_err(map_error)
        })
    }

    fn fetch_portfolio<'py>(
        &self,
        py: Python<'py>,
        coldkey_ss58: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let client = client.lock().await;
            let portfolio = agcli::queries::portfolio::fetch_portfolio(&client, &coldkey_ss58)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&portfolio)
        })
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("Client"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("Client"))
    }
}

#[pymethods]
impl PyClientSync {
    #[staticmethod]
    fn connect(url: String) -> PyResult<Self> {
        let client = runtime()
            .block_on(Client::connect(&url))
            .map_err(map_error)?;
        Ok(wrap_client_sync(client))
    }

    #[staticmethod]
    fn connect_with_retry(urls: Vec<String>) -> PyResult<Self> {
        let refs: Vec<&str> = urls.iter().map(String::as_str).collect();
        let client = runtime()
            .block_on(Client::connect_with_retry(&refs))
            .map_err(map_error)?;
        Ok(wrap_client_sync(client))
    }

    #[staticmethod]
    fn best_connection(urls: Vec<String>) -> PyResult<Self> {
        let refs: Vec<&str> = urls.iter().map(String::as_str).collect();
        let client = runtime()
            .block_on(Client::best_connection(&refs))
            .map_err(map_error)?;
        Ok(wrap_client_sync(client))
    }

    #[staticmethod]
    #[pyo3(signature = (network=None))]
    fn connect_network(network: Option<PyRef<'_, PyNetwork>>) -> PyResult<Self> {
        let network = network
            .map(|n| n.inner.clone())
            .unwrap_or(agcli::types::Network::Finney);
        let client = runtime()
            .block_on(Client::connect_network(&network))
            .map_err(map_error)?;
        Ok(wrap_client_sync(client))
    }

    #[staticmethod]
    fn from_config() -> PyResult<Self> {
        let config = Config::load();
        let client = runtime()
            .block_on(client_from_config(&config))
            .map_err(map_error)?;
        Ok(wrap_client_sync(client))
    }

    #[getter]
    fn endpoint(&self) -> PyResult<String> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(client.url().to_string())
            })
            .map_err(map_error)
    }

    #[getter]
    fn network(&self) -> PyResult<PyNetwork> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(PyNetwork {
                    inner: client.network().clone(),
                })
            })
            .map_err(map_error)
    }

    fn set_dry_run(&self, enabled: bool) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let mut client = client.lock().await;
                client.set_dry_run(enabled);
                Ok(())
            })
            .map_err(map_error)
    }

    fn is_dry_run(&self) -> PyResult<bool> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(client.is_dry_run())
            })
            .map_err(map_error)
    }

    fn set_finalization_timeout(&self, timeout: u64) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let mut client = client.lock().await;
                client.set_finalization_timeout(timeout);
                Ok(())
            })
            .map_err(map_error)
    }

    fn finalization_timeout(&self) -> PyResult<u64> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(client.finalization_timeout())
            })
            .map_err(map_error)
    }

    fn set_mortality_blocks(&self, blocks: u64) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let mut client = client.lock().await;
                client.set_mortality_blocks(blocks);
                Ok(())
            })
            .map_err(map_error)
    }

    fn mortality_blocks(&self) -> PyResult<u64> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(client.mortality_blocks())
            })
            .map_err(map_error)
    }

    fn reconnect(&self) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let mut client = client.lock().await;
                client.reconnect().await
            })
            .map_err(map_error)
    }

    fn invalidate_cache(&self) -> PyResult<()> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                client.invalidate_cache().await;
                Ok(())
            })
            .map_err(map_error)
    }

    fn is_alive(&self) -> PyResult<bool> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                Ok(client.is_alive().await)
            })
            .map_err(map_error)
    }

    fn get_balance(&self, address: String) -> PyResult<PyBalance> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                let value = client.get_balance_ss58(&address).await?;
                Ok(PyBalance::new_inner(value))
            })
            .map_err(map_error)
    }

    fn get_total_issuance(&self) -> PyResult<PyBalance> {
        let client = Arc::clone(&self.inner);
        runtime()
            .block_on(async move {
                let client = client.lock().await;
                let value = client.get_total_issuance().await?;
                Ok(PyBalance::new_inner(value))
            })
            .map_err(map_error)
    }

    fn get_metagraph(&self, netuid: Bound<'_, PyAny>) -> PyResult<PyObject> {
        let netuid = netuid_from_py(&netuid)?;
        let client = Arc::clone(&self.inner);
        let value = runtime()
            .block_on(async move {
                let client = client.lock().await;
                client.get_metagraph(netuid).await
            })
            .map_err(map_error)?;
        to_pyobject_unbound(&value)
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("ClientSync"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("ClientSync"))
    }
}

async fn client_from_config(config: &Config) -> anyhow::Result<Client> {
    if let Some(endpoint) = config.endpoint.as_deref() {
        return Client::connect(endpoint).await;
    }
    let network_name = config.network.as_deref().unwrap_or("finney");
    let network = parse_network(network_name)?;
    Client::connect_network(&network).await
}
