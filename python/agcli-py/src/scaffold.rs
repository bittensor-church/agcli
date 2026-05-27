use agcli::scaffold;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use pyo3_async_runtimes::tokio::future_into_py;

use crate::errors::map_error;
use crate::types::to_pyobject_unbound;

#[pyclass(name = "NeuronConfig", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PyNeuronConfig {
    pub(crate) inner: scaffold::NeuronConfig,
}

#[pymethods]
impl PyNeuronConfig {
    #[new]
    #[pyo3(signature = (name, fund_tao=None, register=true))]
    fn new(name: String, fund_tao: Option<f64>, register: bool) -> Self {
        Self {
            inner: scaffold::NeuronConfig {
                name,
                fund_tao,
                register,
            },
        }
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[setter]
    fn set_name(&mut self, value: String) {
        self.inner.name = value;
    }

    #[getter]
    fn fund_tao(&self) -> Option<f64> {
        self.inner.fund_tao
    }

    #[setter]
    fn set_fund_tao(&mut self, value: Option<f64>) {
        self.inner.fund_tao = value;
    }

    #[getter]
    fn register(&self) -> bool {
        self.inner.register
    }

    #[setter]
    fn set_register(&mut self, value: bool) {
        self.inner.register = value;
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dict = PyDict::new(py);
        dict.set_item("name", &self.inner.name)?;
        dict.set_item("fund_tao", self.inner.fund_tao)?;
        dict.set_item("register", self.inner.register)?;
        Ok(dict.into_any())
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("NeuronConfig"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("NeuronConfig"))
    }
}

#[pyclass(name = "SubnetConfig", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PySubnetConfig {
    pub(crate) inner: scaffold::SubnetConfig,
}

#[pymethods]
impl PySubnetConfig {
    #[new]
    #[pyo3(signature = (
        tempo=None,
        max_allowed_validators=None,
        max_allowed_uids=None,
        min_allowed_weights=None,
        max_weight_limit=None,
        immunity_period=None,
        weights_rate_limit=None,
        commit_reveal=None,
        activity_cutoff=None,
        neurons=None,
    ))]
    fn new(
        tempo: Option<u16>,
        max_allowed_validators: Option<u16>,
        max_allowed_uids: Option<u16>,
        min_allowed_weights: Option<u16>,
        max_weight_limit: Option<u16>,
        immunity_period: Option<u16>,
        weights_rate_limit: Option<u64>,
        commit_reveal: Option<bool>,
        activity_cutoff: Option<u16>,
        neurons: Option<Vec<PyNeuronConfig>>,
    ) -> Self {
        let inner = scaffold::SubnetConfig {
            tempo,
            max_allowed_validators,
            max_allowed_uids,
            min_allowed_weights,
            max_weight_limit,
            immunity_period,
            weights_rate_limit,
            commit_reveal,
            activity_cutoff,
            neuron: neurons
                .map(|v| v.into_iter().map(|n| n.inner).collect())
                .unwrap_or_default(),
        };
        Self { inner }
    }

    #[getter]
    fn tempo(&self) -> Option<u16> {
        self.inner.tempo
    }
    #[setter]
    fn set_tempo(&mut self, v: Option<u16>) {
        self.inner.tempo = v;
    }
    #[getter]
    fn max_allowed_validators(&self) -> Option<u16> {
        self.inner.max_allowed_validators
    }
    #[setter]
    fn set_max_allowed_validators(&mut self, v: Option<u16>) {
        self.inner.max_allowed_validators = v;
    }
    #[getter]
    fn max_allowed_uids(&self) -> Option<u16> {
        self.inner.max_allowed_uids
    }
    #[setter]
    fn set_max_allowed_uids(&mut self, v: Option<u16>) {
        self.inner.max_allowed_uids = v;
    }
    #[getter]
    fn min_allowed_weights(&self) -> Option<u16> {
        self.inner.min_allowed_weights
    }
    #[setter]
    fn set_min_allowed_weights(&mut self, v: Option<u16>) {
        self.inner.min_allowed_weights = v;
    }
    #[getter]
    fn max_weight_limit(&self) -> Option<u16> {
        self.inner.max_weight_limit
    }
    #[setter]
    fn set_max_weight_limit(&mut self, v: Option<u16>) {
        self.inner.max_weight_limit = v;
    }
    #[getter]
    fn immunity_period(&self) -> Option<u16> {
        self.inner.immunity_period
    }
    #[setter]
    fn set_immunity_period(&mut self, v: Option<u16>) {
        self.inner.immunity_period = v;
    }
    #[getter]
    fn weights_rate_limit(&self) -> Option<u64> {
        self.inner.weights_rate_limit
    }
    #[setter]
    fn set_weights_rate_limit(&mut self, v: Option<u64>) {
        self.inner.weights_rate_limit = v;
    }
    #[getter]
    fn commit_reveal(&self) -> Option<bool> {
        self.inner.commit_reveal
    }
    #[setter]
    fn set_commit_reveal(&mut self, v: Option<bool>) {
        self.inner.commit_reveal = v;
    }
    #[getter]
    fn activity_cutoff(&self) -> Option<u16> {
        self.inner.activity_cutoff
    }
    #[setter]
    fn set_activity_cutoff(&mut self, v: Option<u16>) {
        self.inner.activity_cutoff = v;
    }

    #[getter]
    fn neurons(&self) -> Vec<PyNeuronConfig> {
        self.inner
            .neuron
            .iter()
            .cloned()
            .map(|inner| PyNeuronConfig { inner })
            .collect()
    }

    #[setter]
    fn set_neurons(&mut self, value: Vec<PyNeuronConfig>) {
        self.inner.neuron = value.into_iter().map(|n| n.inner).collect();
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dict = PyDict::new(py);
        dict.set_item("tempo", self.inner.tempo)?;
        dict.set_item("max_allowed_validators", self.inner.max_allowed_validators)?;
        dict.set_item("max_allowed_uids", self.inner.max_allowed_uids)?;
        dict.set_item("min_allowed_weights", self.inner.min_allowed_weights)?;
        dict.set_item("max_weight_limit", self.inner.max_weight_limit)?;
        dict.set_item("immunity_period", self.inner.immunity_period)?;
        dict.set_item("weights_rate_limit", self.inner.weights_rate_limit)?;
        dict.set_item("commit_reveal", self.inner.commit_reveal)?;
        dict.set_item("activity_cutoff", self.inner.activity_cutoff)?;
        let neurons = PyList::empty(py);
        for n in &self.inner.neuron {
            let cfg = PyNeuronConfig { inner: n.clone() };
            neurons.append(cfg.to_dict(py)?)?;
        }
        dict.set_item("neurons", neurons)?;
        Ok(dict.into_any())
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("SubnetConfig"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("SubnetConfig"))
    }
}

#[pyclass(name = "ChainConfig", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PyChainConfig {
    pub(crate) inner: scaffold::ChainConfig,
}

#[pymethods]
impl PyChainConfig {
    #[new]
    #[pyo3(signature = (
        image=None,
        container=None,
        port=None,
        start=None,
        timeout=None,
    ))]
    fn new(
        image: Option<String>,
        container: Option<String>,
        port: Option<u16>,
        start: Option<bool>,
        timeout: Option<u64>,
    ) -> Self {
        let mut inner = scaffold::ChainConfig::default();
        if let Some(v) = image {
            inner.image = v;
        }
        if let Some(v) = container {
            inner.container = v;
        }
        if let Some(v) = port {
            inner.port = v;
        }
        if let Some(v) = start {
            inner.start = v;
        }
        if let Some(v) = timeout {
            inner.timeout = v;
        }
        Self { inner }
    }

    #[getter]
    fn image(&self) -> String {
        self.inner.image.clone()
    }
    #[setter]
    fn set_image(&mut self, v: String) {
        self.inner.image = v;
    }
    #[getter]
    fn container(&self) -> String {
        self.inner.container.clone()
    }
    #[setter]
    fn set_container(&mut self, v: String) {
        self.inner.container = v;
    }
    #[getter]
    fn port(&self) -> u16 {
        self.inner.port
    }
    #[setter]
    fn set_port(&mut self, v: u16) {
        self.inner.port = v;
    }
    #[getter]
    fn start(&self) -> bool {
        self.inner.start
    }
    #[setter]
    fn set_start(&mut self, v: bool) {
        self.inner.start = v;
    }
    #[getter]
    fn timeout(&self) -> u64 {
        self.inner.timeout
    }
    #[setter]
    fn set_timeout(&mut self, v: u64) {
        self.inner.timeout = v;
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dict = PyDict::new(py);
        dict.set_item("image", &self.inner.image)?;
        dict.set_item("container", &self.inner.container)?;
        dict.set_item("port", self.inner.port)?;
        dict.set_item("start", self.inner.start)?;
        dict.set_item("timeout", self.inner.timeout)?;
        Ok(dict.into_any())
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("ChainConfig"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("ChainConfig"))
    }
}

#[pyclass(name = "ScaffoldConfig", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PyScaffoldConfig {
    pub(crate) inner: scaffold::ScaffoldConfig,
}

#[pymethods]
impl PyScaffoldConfig {
    #[new]
    #[pyo3(signature = (chain=None, subnets=None))]
    fn new(chain: Option<PyChainConfig>, subnets: Option<Vec<PySubnetConfig>>) -> Self {
        let inner = scaffold::ScaffoldConfig {
            chain: chain
                .map(|c| c.inner)
                .unwrap_or_else(scaffold::ChainConfig::default),
            subnet: subnets
                .map(|v| v.into_iter().map(|s| s.inner).collect())
                .unwrap_or_default(),
        };
        Self { inner }
    }

    #[getter]
    fn chain(&self) -> PyChainConfig {
        PyChainConfig {
            inner: self.inner.chain.clone(),
        }
    }

    #[setter]
    fn set_chain(&mut self, value: PyChainConfig) {
        self.inner.chain = value.inner;
    }

    #[getter]
    fn subnets(&self) -> Vec<PySubnetConfig> {
        self.inner
            .subnet
            .iter()
            .cloned()
            .map(|inner| PySubnetConfig { inner })
            .collect()
    }

    #[setter]
    fn set_subnets(&mut self, value: Vec<PySubnetConfig>) {
        self.inner.subnet = value.into_iter().map(|s| s.inner).collect();
    }

    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let dict = PyDict::new(py);
        let chain = PyChainConfig {
            inner: self.inner.chain.clone(),
        };
        dict.set_item("chain", chain.to_dict(py)?)?;
        let subnets = PyList::empty(py);
        for s in &self.inner.subnet {
            let sc = PySubnetConfig { inner: s.clone() };
            subnets.append(sc.to_dict(py)?)?;
        }
        dict.set_item("subnets", subnets)?;
        Ok(dict.into_any())
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("ScaffoldConfig"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("ScaffoldConfig"))
    }
}

#[pyclass(name = "NeuronResult", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PyNeuronResult {
    pub(crate) inner: scaffold::NeuronResult,
}

#[pymethods]
impl PyNeuronResult {
    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }
    #[getter]
    fn ss58(&self) -> String {
        self.inner.ss58.clone()
    }
    #[getter]
    fn seed(&self) -> String {
        self.inner.seed.clone()
    }
    #[getter]
    fn uid(&self) -> Option<u16> {
        self.inner.uid
    }
    #[getter]
    fn balance_tao(&self) -> Option<f64> {
        self.inner.balance_tao
    }

    fn to_dict(&self) -> PyResult<PyObject> {
        to_pyobject_unbound(&self.inner)
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("NeuronResult"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("NeuronResult"))
    }
}

#[pyclass(name = "SubnetResult", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PySubnetResult {
    pub(crate) inner: scaffold::SubnetResult,
}

#[pymethods]
impl PySubnetResult {
    #[getter]
    fn netuid(&self) -> u16 {
        self.inner.netuid
    }

    #[getter]
    fn hyperparams(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            pythonize::pythonize(py, &self.inner.hyperparams)
                .map(|b| b.unbind())
                .map_err(|e| map_error(anyhow::anyhow!("{e}")))
        })
    }

    #[getter]
    fn neurons(&self) -> Vec<PyNeuronResult> {
        self.inner
            .neurons
            .iter()
            .cloned()
            .map(|inner| PyNeuronResult { inner })
            .collect()
    }

    fn to_dict(&self) -> PyResult<PyObject> {
        to_pyobject_unbound(&self.inner)
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("SubnetResult"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("SubnetResult"))
    }
}

#[pyclass(name = "ScaffoldResult", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PyScaffoldResult {
    pub(crate) inner: scaffold::ScaffoldResult,
}

#[pymethods]
impl PyScaffoldResult {
    #[getter]
    fn endpoint(&self) -> String {
        self.inner.endpoint.clone()
    }
    #[getter]
    fn container(&self) -> Option<String> {
        self.inner.container.clone()
    }
    #[getter]
    fn block_height(&self) -> u64 {
        self.inner.block_height
    }
    #[getter]
    fn subnets(&self) -> Vec<PySubnetResult> {
        self.inner
            .subnets
            .iter()
            .cloned()
            .map(|inner| PySubnetResult { inner })
            .collect()
    }

    fn to_dict(&self) -> PyResult<PyObject> {
        to_pyobject_unbound(&self.inner)
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("ScaffoldResult"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("ScaffoldResult"))
    }
}

#[pyfunction]
pub fn load_config(path: String) -> PyResult<PyScaffoldConfig> {
    let inner = scaffold::load_config(&path).map_err(map_error)?;
    Ok(PyScaffoldConfig { inner })
}

#[pyfunction]
pub fn run<'py>(
    py: Python<'py>,
    config: PyRef<'_, PyScaffoldConfig>,
) -> PyResult<Bound<'py, PyAny>> {
    let cfg = config.inner.clone();
    future_into_py(py, async move {
        let result = scaffold::run(&cfg).await.map_err(map_error)?;
        Ok(PyScaffoldResult { inner: result })
    })
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyNeuronConfig>()?;
    m.add_class::<PySubnetConfig>()?;
    m.add_class::<PyChainConfig>()?;
    m.add_class::<PyScaffoldConfig>()?;
    m.add_class::<PyNeuronResult>()?;
    m.add_class::<PySubnetResult>()?;
    m.add_class::<PyScaffoldResult>()?;
    m.add_function(wrap_pyfunction!(load_config, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
