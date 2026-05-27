use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use serde::Serialize;

use crate::errors::map_error;
use crate::types::to_pyobject_unbound;

fn json_field_to_pyobject<T: Serialize>(value: &T, field: &str) -> PyResult<PyObject> {
    let json = serde_json::to_value(value).map_err(|e| map_error(anyhow::anyhow!("{e}")))?;
    if let serde_json::Value::Object(map) = json {
        if let Some(field_value) = map.get(field) {
            return Python::with_gil(|py| {
                pythonize::pythonize(py, field_value)
                    .map(|obj| obj.unbind())
                    .map_err(|e| map_error(anyhow::anyhow!("{e}")))
            });
        }
    }
    Err(PyAttributeError::new_err(format!(
        "attribute '{field}' not found"
    )))
}

macro_rules! serde_pyclass {
    ($py_name:ident, $inner:path, $class_name:literal) => {
        #[pyclass(name = $class_name, module = "agcli._agcli")]
        #[derive(Clone)]
        pub struct $py_name {
            inner: $inner,
        }

        #[pymethods]
        impl $py_name {
            #[new]
            fn new(value: &Bound<'_, PyAny>) -> PyResult<Self> {
                let inner: $inner =
                    pythonize::depythonize(value).map_err(|e| map_error(anyhow::anyhow!("{e}")))?;
                Ok(Self { inner })
            }

            fn to_dict(&self) -> PyResult<PyObject> {
                to_pyobject_unbound(&self.inner)
            }

            fn __getattr__(&self, field: &str) -> PyResult<PyObject> {
                json_field_to_pyobject(&self.inner, field)
            }

            fn __getstate__(&self) -> PyResult<()> {
                Err(crate::errors::pickle_blocked($class_name))
            }

            fn __reduce__(&self) -> PyResult<()> {
                Err(crate::errors::pickle_blocked($class_name))
            }
        }
    };
}

serde_pyclass!(
    PySubnetInfo,
    agcli::types::chain_data::SubnetInfo,
    "SubnetInfo"
);
serde_pyclass!(
    PyNeuronInfo,
    agcli::types::chain_data::NeuronInfo,
    "NeuronInfo"
);
serde_pyclass!(
    PyNeuronInfoLite,
    agcli::types::chain_data::NeuronInfoLite,
    "NeuronInfoLite"
);
serde_pyclass!(
    PyMetagraph,
    agcli::types::chain_data::Metagraph,
    "Metagraph"
);
serde_pyclass!(
    PyStakeInfo,
    agcli::types::chain_data::StakeInfo,
    "StakeInfo"
);
serde_pyclass!(
    PyDelegateInfo,
    agcli::types::chain_data::DelegateInfo,
    "DelegateInfo"
);
serde_pyclass!(
    PyDynamicInfo,
    agcli::types::chain_data::DynamicInfo,
    "DynamicInfo"
);
serde_pyclass!(
    PySubnetHyperparameters,
    agcli::types::chain_data::SubnetHyperparameters,
    "SubnetHyperparameters"
);
serde_pyclass!(
    PyChainIdentity,
    agcli::types::chain_data::ChainIdentity,
    "ChainIdentity"
);
serde_pyclass!(
    PySubnetIdentity,
    agcli::types::chain_data::SubnetIdentity,
    "SubnetIdentity"
);

impl PySubnetIdentity {
    pub(crate) fn inner_clone(&self) -> agcli::types::chain_data::SubnetIdentity {
        self.inner.clone()
    }
}

#[pyclass(name = "AlphaBalance", module = "agcli._agcli")]
#[derive(Clone, Copy)]
pub struct PyAlphaBalance {
    inner: agcli::types::balance::AlphaBalance,
}

#[pymethods]
impl PyAlphaBalance {
    #[new]
    fn new(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(raw) = value.extract::<u64>() {
            return Ok(Self {
                inner: agcli::types::balance::AlphaBalance::from_raw(raw),
            });
        }
        let inner: agcli::types::balance::AlphaBalance =
            pythonize::depythonize(value).map_err(|e| map_error(anyhow::anyhow!("{e}")))?;
        Ok(Self { inner })
    }

    #[getter]
    fn raw(&self) -> u64 {
        self.inner.raw()
    }

    #[getter]
    fn rao(&self) -> u64 {
        self.inner.raw()
    }

    #[getter]
    fn tao(&self) -> f64 {
        self.inner.raw() as f64 / 1_000_000_000_f64
    }

    fn to_dict(&self) -> PyResult<PyObject> {
        to_pyobject_unbound(&self.inner)
    }

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("AlphaBalance"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("AlphaBalance"))
    }
}
