use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::errors::map_error;

pub fn to_pyobject<'a, T: serde::Serialize>(
    py: Python<'a>,
    value: &T,
) -> PyResult<Bound<'a, PyAny>> {
    pythonize::pythonize(py, value).map_err(|e| map_error(anyhow::anyhow!("{e}")))
}

pub fn to_pyobject_unbound<T: serde::Serialize>(value: &T) -> PyResult<PyObject> {
    Python::with_gil(|py| to_pyobject(py, value).map(|obj| obj.unbind()))
}

pub fn hash_to_hex(hash: agcli::Hash) -> String {
    format!("0x{}", hex::encode(hash.as_ref()))
}

pub fn parse_hash(value: &Bound<'_, PyAny>) -> PyResult<agcli::Hash> {
    if let Ok(bytes) = value.extract::<Vec<u8>>() {
        if bytes.len() != 32 {
            return Err(map_error(anyhow::anyhow!(
                "expected 32-byte hash, got {} bytes",
                bytes.len()
            )));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        return Ok(arr.into());
    }

    if let Ok(hex_str) = value.extract::<String>() {
        let trimmed = hex_str.trim();
        let raw = trimmed.strip_prefix("0x").unwrap_or(trimmed);
        let bytes = hex::decode(raw).map_err(|e| map_error(anyhow::anyhow!("{e}")))?;
        if bytes.len() != 32 {
            return Err(map_error(anyhow::anyhow!(
                "expected 32-byte hash hex string, got {} bytes",
                bytes.len()
            )));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        return Ok(arr.into());
    }

    Err(map_error(anyhow::anyhow!(
        "expected block hash as bytes or hex string, got {}",
        value.get_type().name()?
    )))
}

pub fn u64_to_u32(block_number: u64) -> PyResult<u32> {
    if block_number > u32::MAX as u64 {
        return Err(map_error(anyhow::anyhow!(
            "block number {block_number} exceeds max u32 value"
        )));
    }
    Ok(block_number as u32)
}

pub fn netuid_from_py(value: &Bound<'_, PyAny>) -> PyResult<agcli::types::network::NetUid> {
    if let Ok(uid) = value.extract::<u16>() {
        return Ok(agcli::types::network::NetUid(uid));
    }
    if let Ok(uid) = value.extract::<u32>() {
        if uid > u16::MAX as u32 {
            return Err(map_error(anyhow::anyhow!(
                "netuid {uid} exceeds maximum value {}",
                u16::MAX
            )));
        }
        return Ok(agcli::types::network::NetUid(uid as u16));
    }
    if let Ok(uid) = value.extract::<i64>() {
        if uid < 0 || uid > u16::MAX as i64 {
            return Err(map_error(anyhow::anyhow!("invalid netuid: {uid}")));
        }
        return Ok(agcli::types::network::NetUid(uid as u16));
    }
    if let Ok(netuid) = value.extract::<PyRef<'_, PyNetUid>>() {
        return Ok(netuid.inner);
    }
    Err(map_error(anyhow::anyhow!(
        "expected netuid as int or NetUid, got {}",
        value.get_type().name()?
    )))
}

#[pyclass(name = "Balance", module = "agcli._agcli")]
#[derive(Clone, Copy)]
pub struct PyBalance {
    inner: agcli::Balance,
}

#[pymethods]
impl PyBalance {
    #[new]
    #[pyo3(signature = (*, rao=None, tao=None))]
    fn new(rao: Option<u64>, tao: Option<f64>) -> PyResult<Self> {
        match (rao, tao) {
            (Some(r), None) => Ok(Self {
                inner: agcli::Balance::from_rao(r),
            }),
            (None, Some(t)) => Ok(Self {
                inner: agcli::Balance::from_tao(t),
            }),
            (Some(_), Some(_)) => Err(map_error(anyhow::anyhow!(
                "pass either rao= or tao=, not both"
            ))),
            (None, None) => Ok(Self {
                inner: agcli::Balance::ZERO,
            }),
        }
    }

    #[staticmethod]
    fn from_rao(rao: u64) -> Self {
        Self {
            inner: agcli::Balance::from_rao(rao),
        }
    }

    #[staticmethod]
    fn from_tao(tao: f64) -> Self {
        Self {
            inner: agcli::Balance::from_tao(tao),
        }
    }

    #[getter]
    fn rao(&self) -> u64 {
        self.inner.rao()
    }

    #[getter]
    fn tao(&self) -> f64 {
        self.inner.tao()
    }

    fn __repr__(&self) -> String {
        format!(
            "Balance(rao={}, tao={})",
            self.inner.rao(),
            self.inner.tao()
        )
    }

    fn __str__(&self) -> String {
        self.inner.display_tao()
    }
}

impl PyBalance {
    pub fn new_inner(inner: agcli::Balance) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> agcli::Balance {
        self.inner
    }
}

#[pyclass(name = "NetUid", module = "agcli._agcli")]
#[derive(Clone, Copy)]
pub struct PyNetUid {
    pub(crate) inner: agcli::types::network::NetUid,
}

#[pymethods]
impl PyNetUid {
    #[new]
    fn new(value: u16) -> Self {
        Self {
            inner: agcli::types::network::NetUid(value),
        }
    }

    #[getter]
    fn value(&self) -> u16 {
        self.inner.as_u16()
    }

    fn __repr__(&self) -> String {
        format!("NetUid({})", self.inner.as_u16())
    }

    fn __int__(&self) -> u16 {
        self.inner.as_u16()
    }
}

impl PyNetUid {
    pub fn new_inner(inner: agcli::types::network::NetUid) -> Self {
        Self { inner }
    }
}

#[pyclass(name = "Network", module = "agcli._agcli")]
#[derive(Clone)]
pub struct PyNetwork {
    pub(crate) inner: agcli::types::Network,
}

#[pymethods]
impl PyNetwork {
    #[staticmethod]
    fn finney() -> Self {
        Self {
            inner: agcli::types::Network::Finney,
        }
    }

    #[staticmethod]
    fn test() -> Self {
        Self {
            inner: agcli::types::Network::Test,
        }
    }

    #[staticmethod]
    fn local() -> Self {
        Self {
            inner: agcli::types::Network::Local,
        }
    }

    #[staticmethod]
    fn archive() -> Self {
        Self {
            inner: agcli::types::Network::Archive,
        }
    }

    #[staticmethod]
    fn custom(url: String) -> Self {
        Self {
            inner: agcli::types::Network::Custom(url),
        }
    }

    #[staticmethod]
    fn parse(name: &str) -> PyResult<Self> {
        parse_network(name)
            .map(|inner| Self { inner })
            .map_err(map_error)
    }

    #[getter]
    fn ws_url(&self) -> String {
        self.inner.ws_url().to_string()
    }

    fn __repr__(&self) -> String {
        self.inner.to_string()
    }
}

pub fn parse_network(name: &str) -> anyhow::Result<agcli::types::Network> {
    match name.to_lowercase().as_str() {
        "finney" | "main" | "mainnet" => Ok(agcli::types::Network::Finney),
        "test" | "testnet" => Ok(agcli::types::Network::Test),
        "local" | "localhost" => Ok(agcli::types::Network::Local),
        "archive" => Ok(agcli::types::Network::Archive),
        other if other.starts_with("wss://") || other.starts_with("ws://") => {
            Ok(agcli::types::Network::Custom(other.to_string()))
        }
        other => Err(anyhow::anyhow!(
            "unknown network '{other}'; expected finney, test, local, archive, or a ws(s) URL"
        )),
    }
}
