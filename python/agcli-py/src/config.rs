use std::collections::HashMap;
use std::path::PathBuf;

use agcli::Config;
use pyo3::prelude::*;

use crate::errors::map_error;

#[pyclass(name = "Config", module = "agcli._agcli")]
#[derive(Clone, Default)]
pub struct PyConfig {
    #[pyo3(get, set)]
    pub network: Option<String>,
    #[pyo3(get, set)]
    pub endpoint: Option<String>,
    #[pyo3(get, set)]
    pub wallet_dir: Option<String>,
    #[pyo3(get, set)]
    pub wallet: Option<String>,
    #[pyo3(get, set)]
    pub hotkey: Option<String>,
    #[pyo3(get, set)]
    pub output: Option<String>,
    #[pyo3(get, set)]
    pub proxy: Option<String>,
    #[pyo3(get, set)]
    pub live_interval: Option<u64>,
    #[pyo3(get, set)]
    pub batch: Option<bool>,
    #[pyo3(get, set)]
    pub spending_limits: Option<HashMap<String, f64>>,
    #[pyo3(get, set)]
    pub finalization_timeout: Option<u64>,
    #[pyo3(get, set)]
    pub mortality_blocks: Option<u64>,
}

impl PyConfig {
    pub fn from_inner(inner: Config) -> Self {
        Self {
            network: inner.network,
            endpoint: inner.endpoint,
            wallet_dir: inner.wallet_dir,
            wallet: inner.wallet,
            hotkey: inner.hotkey,
            output: inner.output,
            proxy: inner.proxy,
            live_interval: inner.live_interval,
            batch: inner.batch,
            spending_limits: inner.spending_limits,
            finalization_timeout: inner.finalization_timeout,
            mortality_blocks: inner.mortality_blocks,
        }
    }

    pub fn to_inner(&self) -> Config {
        Config {
            network: self.network.clone(),
            endpoint: self.endpoint.clone(),
            wallet_dir: self.wallet_dir.clone(),
            wallet: self.wallet.clone(),
            hotkey: self.hotkey.clone(),
            output: self.output.clone(),
            proxy: self.proxy.clone(),
            live_interval: self.live_interval,
            batch: self.batch,
            spending_limits: self.spending_limits.clone(),
            finalization_timeout: self.finalization_timeout,
            mortality_blocks: self.mortality_blocks,
        }
    }
}

#[pymethods]
impl PyConfig {
    #[new]
    #[pyo3(signature = (
        network=None,
        endpoint=None,
        wallet_dir=None,
        wallet=None,
        hotkey=None,
        output=None,
        proxy=None,
        live_interval=None,
        batch=None,
        spending_limits=None,
        finalization_timeout=None,
        mortality_blocks=None
    ))]
    fn new(
        network: Option<String>,
        endpoint: Option<String>,
        wallet_dir: Option<String>,
        wallet: Option<String>,
        hotkey: Option<String>,
        output: Option<String>,
        proxy: Option<String>,
        live_interval: Option<u64>,
        batch: Option<bool>,
        spending_limits: Option<HashMap<String, f64>>,
        finalization_timeout: Option<u64>,
        mortality_blocks: Option<u64>,
    ) -> Self {
        Self {
            network,
            endpoint,
            wallet_dir,
            wallet,
            hotkey,
            output,
            proxy,
            live_interval,
            batch,
            spending_limits,
            finalization_timeout,
            mortality_blocks,
        }
    }

    #[staticmethod]
    fn load() -> Self {
        Self::from_inner(Config::load())
    }

    fn save(&self) -> PyResult<()> {
        self.to_inner().save().map_err(map_error)
    }

    #[staticmethod]
    fn load_from(path: String) -> PyResult<Self> {
        let cfg = Config::load_from(PathBuf::from(path).as_path()).map_err(map_error)?;
        Ok(Self::from_inner(cfg))
    }

    fn save_to(&self, path: String) -> PyResult<()> {
        self.to_inner()
            .save_to(PathBuf::from(path).as_path())
            .map_err(map_error)
    }

    #[staticmethod]
    fn default_path() -> String {
        Config::default_path().display().to_string()
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("Config"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("Config"))
    }
}
