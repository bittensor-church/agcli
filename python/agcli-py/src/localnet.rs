use agcli::localnet;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3_async_runtimes::tokio::future_into_py;

use crate::errors::map_error;
use crate::types::to_pyobject_unbound;

#[pyclass(name = "LocalnetConfig", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PyLocalnetConfig {
    pub(crate) inner: localnet::LocalnetConfig,
}

#[pymethods]
impl PyLocalnetConfig {
    #[new]
    #[pyo3(signature = (
        image=None,
        container_name=None,
        port=None,
        wait=None,
        wait_timeout=None,
    ))]
    fn new(
        image: Option<String>,
        container_name: Option<String>,
        port: Option<u16>,
        wait: Option<bool>,
        wait_timeout: Option<u64>,
    ) -> Self {
        let mut cfg = localnet::LocalnetConfig::default();
        if let Some(v) = image {
            cfg.image = v;
        }
        if let Some(v) = container_name {
            cfg.container_name = v;
        }
        if let Some(v) = port {
            cfg.port = v;
        }
        if let Some(v) = wait {
            cfg.wait = v;
        }
        if let Some(v) = wait_timeout {
            cfg.wait_timeout = v;
        }
        Self { inner: cfg }
    }

    #[getter]
    fn image(&self) -> String {
        self.inner.image.clone()
    }

    #[setter]
    fn set_image(&mut self, value: String) {
        self.inner.image = value;
    }

    #[getter]
    fn container_name(&self) -> String {
        self.inner.container_name.clone()
    }

    #[setter]
    fn set_container_name(&mut self, value: String) {
        self.inner.container_name = value;
    }

    #[getter]
    fn port(&self) -> u16 {
        self.inner.port
    }

    #[setter]
    fn set_port(&mut self, value: u16) {
        self.inner.port = value;
    }

    #[getter]
    fn wait(&self) -> bool {
        self.inner.wait
    }

    #[setter]
    fn set_wait(&mut self, value: bool) {
        self.inner.wait = value;
    }

    #[getter]
    fn wait_timeout(&self) -> u64 {
        self.inner.wait_timeout
    }

    #[setter]
    fn set_wait_timeout(&mut self, value: u64) {
        self.inner.wait_timeout = value;
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("image", &self.inner.image)?;
        dict.set_item("container_name", &self.inner.container_name)?;
        dict.set_item("port", self.inner.port)?;
        dict.set_item("wait", self.inner.wait)?;
        dict.set_item("wait_timeout", self.inner.wait_timeout)?;
        Ok(dict.into_any())
    }

    fn __repr__(&self) -> String {
        format!(
            "LocalnetConfig(image={:?}, container_name={:?}, port={}, wait={}, wait_timeout={})",
            self.inner.image,
            self.inner.container_name,
            self.inner.port,
            self.inner.wait,
            self.inner.wait_timeout
        )
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("LocalnetConfig"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("LocalnetConfig"))
    }
}

#[pyclass(name = "DevAccount", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PyDevAccount {
    pub(crate) inner: localnet::DevAccount,
}

#[pymethods]
impl PyDevAccount {
    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[getter]
    fn uri(&self) -> String {
        self.inner.uri.clone()
    }

    #[getter]
    fn ss58(&self) -> String {
        self.inner.ss58.clone()
    }

    #[getter]
    fn balance(&self) -> String {
        self.inner.balance.clone()
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("name", &self.inner.name)?;
        dict.set_item("uri", &self.inner.uri)?;
        dict.set_item("ss58", &self.inner.ss58)?;
        dict.set_item("balance", &self.inner.balance)?;
        Ok(dict.into_any())
    }

    fn __repr__(&self) -> String {
        format!(
            "DevAccount(name={:?}, uri={:?}, ss58={:?})",
            self.inner.name, self.inner.uri, self.inner.ss58
        )
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("DevAccount"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("DevAccount"))
    }
}

#[pyclass(name = "LocalnetInfo", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PyLocalnetInfo {
    pub(crate) inner: localnet::LocalnetInfo,
}

#[pymethods]
impl PyLocalnetInfo {
    #[getter]
    fn container_name(&self) -> String {
        self.inner.container_name.clone()
    }

    #[getter]
    fn container_id(&self) -> String {
        self.inner.container_id.clone()
    }

    #[getter]
    fn image(&self) -> String {
        self.inner.image.clone()
    }

    #[getter]
    fn endpoint(&self) -> String {
        self.inner.endpoint.clone()
    }

    #[getter]
    fn port(&self) -> u16 {
        self.inner.port
    }

    #[getter]
    fn block_height(&self) -> u64 {
        self.inner.block_height
    }

    #[getter]
    fn dev_accounts(&self) -> Vec<PyDevAccount> {
        self.inner
            .dev_accounts
            .iter()
            .cloned()
            .map(|a| PyDevAccount { inner: a })
            .collect()
    }

    fn to_dict(&self) -> PyResult<PyObject> {
        to_pyobject_unbound(&self.inner)
    }

    fn __repr__(&self) -> String {
        format!(
            "LocalnetInfo(container_name={:?}, endpoint={:?}, block_height={})",
            self.inner.container_name, self.inner.endpoint, self.inner.block_height
        )
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("LocalnetInfo"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("LocalnetInfo"))
    }
}

#[pyclass(name = "LocalnetStatus", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PyLocalnetStatus {
    pub(crate) inner: localnet::LocalnetStatus,
}

#[pymethods]
impl PyLocalnetStatus {
    #[getter]
    fn running(&self) -> bool {
        self.inner.running
    }

    #[getter]
    fn container_name(&self) -> String {
        self.inner.container_name.clone()
    }

    #[getter]
    fn container_id(&self) -> Option<String> {
        self.inner.container_id.clone()
    }

    #[getter]
    fn image(&self) -> Option<String> {
        self.inner.image.clone()
    }

    #[getter]
    fn endpoint(&self) -> Option<String> {
        self.inner.endpoint.clone()
    }

    #[getter]
    fn block_height(&self) -> Option<u64> {
        self.inner.block_height
    }

    #[getter]
    fn started_at(&self) -> Option<String> {
        self.inner.started_at.clone()
    }

    #[getter]
    fn uptime(&self) -> Option<String> {
        self.inner.started_at.clone()
    }

    fn to_dict(&self) -> PyResult<PyObject> {
        to_pyobject_unbound(&self.inner)
    }

    fn __repr__(&self) -> String {
        format!(
            "LocalnetStatus(running={}, container_name={:?}, endpoint={:?})",
            self.inner.running, self.inner.container_name, self.inner.endpoint
        )
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("LocalnetStatus"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("LocalnetStatus"))
    }
}

#[pyfunction]
pub fn dev_accounts() -> Vec<PyDevAccount> {
    localnet::dev_accounts()
        .into_iter()
        .map(|a| PyDevAccount { inner: a })
        .collect()
}

#[pyfunction]
#[pyo3(signature = (config=None))]
pub fn start<'py>(
    py: Python<'py>,
    config: Option<PyRef<'_, PyLocalnetConfig>>,
) -> PyResult<Bound<'py, PyAny>> {
    let cfg = config
        .map(|c| c.inner.clone())
        .unwrap_or_else(localnet::LocalnetConfig::default);
    future_into_py(py, async move {
        let info = localnet::start(&cfg).await.map_err(map_error)?;
        Ok(PyLocalnetInfo { inner: info })
    })
}

#[pyfunction]
pub fn stop(container_name: String) -> PyResult<()> {
    localnet::stop(&container_name).map_err(map_error)
}

#[pyfunction]
#[pyo3(signature = (container_name=None, port=9944))]
pub fn status<'py>(
    py: Python<'py>,
    container_name: Option<String>,
    port: u16,
) -> PyResult<Bound<'py, PyAny>> {
    let name = container_name.unwrap_or_else(|| localnet::DEFAULT_CONTAINER.to_string());
    future_into_py(py, async move {
        let s = localnet::status(&name, port).await.map_err(map_error)?;
        Ok(PyLocalnetStatus { inner: s })
    })
}

#[pyfunction]
#[pyo3(signature = (config=None))]
pub fn reset<'py>(
    py: Python<'py>,
    config: Option<PyRef<'_, PyLocalnetConfig>>,
) -> PyResult<Bound<'py, PyAny>> {
    let cfg = config
        .map(|c| c.inner.clone())
        .unwrap_or_else(localnet::LocalnetConfig::default);
    future_into_py(py, async move {
        let info = localnet::reset(&cfg).await.map_err(map_error)?;
        Ok(PyLocalnetInfo { inner: info })
    })
}

#[pyfunction]
#[pyo3(signature = (container_name, tail=None))]
pub fn logs(container_name: String, tail: Option<u32>) -> PyResult<String> {
    localnet::logs(&container_name, tail).map_err(map_error)
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyLocalnetConfig>()?;
    m.add_class::<PyLocalnetInfo>()?;
    m.add_class::<PyLocalnetStatus>()?;
    m.add_class::<PyDevAccount>()?;
    m.add_function(wrap_pyfunction!(dev_accounts, m)?)?;
    m.add_function(wrap_pyfunction!(start, m)?)?;
    m.add_function(wrap_pyfunction!(stop, m)?)?;
    m.add_function(wrap_pyfunction!(status, m)?)?;
    m.add_function(wrap_pyfunction!(reset, m)?)?;
    m.add_function(wrap_pyfunction!(logs, m)?)?;
    m.add("DEFAULT_IMAGE", localnet::DEFAULT_IMAGE)?;
    m.add("DEFAULT_CONTAINER", localnet::DEFAULT_CONTAINER)?;
    m.add("DEFAULT_WS", localnet::DEFAULT_WS)?;
    Ok(())
}
