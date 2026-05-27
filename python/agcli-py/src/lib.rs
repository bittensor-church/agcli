mod chain_data;
mod client;
mod config;
mod errors;
mod extrinsics;
mod runtime;
mod types;
mod wallet;

use pyo3::prelude::*;

use chain_data::{
    PyAlphaBalance, PyChainIdentity, PyDelegateInfo, PyDynamicInfo, PyMetagraph, PyNeuronInfo,
    PyNeuronInfoLite, PyStakeInfo, PySubnetHyperparameters, PySubnetIdentity, PySubnetInfo,
};
use client::{PyClient, PyClientSync};
use config::PyConfig;
use errors::{
    AgcliError, AuthError, ChainError, IOError, NetworkError, TimeoutError, ValidationError,
};
use types::{PyBalance, PyNetUid, PyNetwork};
use wallet::PyWallet;

/// Python bindings for the agcli Bittensor SDK.
#[pymodule]
fn _agcli(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    runtime::runtime();
    m.add("AgcliError", py.get_type::<AgcliError>())?;
    m.add("NetworkError", py.get_type::<NetworkError>())?;
    m.add("AuthError", py.get_type::<AuthError>())?;
    m.add("ValidationError", py.get_type::<ValidationError>())?;
    m.add("ChainError", py.get_type::<ChainError>())?;
    m.add("TimeoutError", py.get_type::<TimeoutError>())?;
    m.add("IOError", py.get_type::<IOError>())?;
    m.add_class::<PyBalance>()?;
    m.add_class::<PyAlphaBalance>()?;
    m.add_class::<PyNetUid>()?;
    m.add_class::<PyNetwork>()?;
    m.add_class::<PySubnetInfo>()?;
    m.add_class::<PyNeuronInfo>()?;
    m.add_class::<PyNeuronInfoLite>()?;
    m.add_class::<PyMetagraph>()?;
    m.add_class::<PyStakeInfo>()?;
    m.add_class::<PyDelegateInfo>()?;
    m.add_class::<PyDynamicInfo>()?;
    m.add_class::<PySubnetHyperparameters>()?;
    m.add_class::<PyChainIdentity>()?;
    m.add_class::<PySubnetIdentity>()?;
    m.add_class::<PyConfig>()?;
    m.add_class::<PyClient>()?;
    m.add_class::<PyClientSync>()?;
    m.add_class::<PyWallet>()?;
    m.add_function(wrap_pyfunction!(errors::raise_test_error, m)?)?;
    Ok(())
}
