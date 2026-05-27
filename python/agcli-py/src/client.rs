use std::sync::Arc;

use agcli::{Client, Config};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3_async_runtimes::tokio::future_into_py;

use crate::errors::map_error;
use crate::types::{netuid_from_py, parse_network, to_pyobject_unbound, PyBalance, PyNetwork};

#[pyclass(name = "Client", module = "agcli._agcli")]
pub struct PyClient {
    inner: Arc<Client>,
}

fn wrap_client(client: Client) -> PyClient {
    PyClient {
        inner: Arc::new(client),
    }
}

#[pymethods]
impl PyClient {
    #[staticmethod]
    #[pyo3(signature = (url))]
    fn connect<'py>(py: Python<'py>, url: String) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, async move {
            let client = Client::connect(&url).await.map_err(map_error)?;
            Ok(wrap_client(client))
        })
    }

    #[staticmethod]
    #[pyo3(signature = (urls))]
    fn connect_with_retry<'py>(py: Python<'py>, urls: Vec<String>) -> PyResult<Bound<'py, PyAny>> {
        future_into_py(py, async move {
            let refs: Vec<&str> = urls.iter().map(String::as_str).collect();
            let client = Client::connect_with_retry(&refs)
                .await
                .map_err(map_error)?;
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
            let client = Client::connect_network(&network)
                .await
                .map_err(map_error)?;
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
    fn endpoint(&self) -> String {
        self.inner.url().to_string()
    }

    fn get_balance<'py>(&self, py: Python<'py>, address: String) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let balance = client.get_balance_ss58(&address).await.map_err(map_error)?;
            Ok(PyBalance::new_inner(balance))
        })
    }

    fn get_all_subnets<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
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
            let params = client
                .get_subnet_hyperparams(netuid)
                .await
                .map_err(map_error)?;
            match params {
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
            let info = client.get_dynamic_info(netuid).await.map_err(map_error)?;
            match info {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
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
            let stakes = client
                .get_stake_for_coldkey(&coldkey)
                .await
                .map_err(map_error)?;
            to_pyobject_unbound(&stakes)
        })
    }

    fn get_delegates<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let delegates = client.get_delegates().await.map_err(map_error)?;
            to_pyobject_unbound(&delegates)
        })
    }

    fn get_delegate<'py>(
        &self,
        py: Python<'py>,
        hotkey: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let delegate = client.get_delegate(&hotkey).await.map_err(map_error)?;
            match delegate {
                Some(value) => to_pyobject_unbound(&value),
                None => Python::with_gil(|py| Ok(py.None())),
            }
        })
    }

    fn get_identity<'py>(
        &self,
        py: Python<'py>,
        ss58: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = Arc::clone(&self.inner);
        future_into_py(py, async move {
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
}

async fn client_from_config(config: &Config) -> anyhow::Result<Client> {
    if let Some(endpoint) = config.endpoint.as_deref() {
        return Client::connect(endpoint).await;
    }
    let network_name = config.network.as_deref().unwrap_or("finney");
    let network = parse_network(network_name)?;
    Client::connect_network(&network).await
}
