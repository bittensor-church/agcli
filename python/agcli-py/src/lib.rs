mod client;
mod errors;
mod runtime;
mod types;
mod wallet;

use pyo3::prelude::*;

use client::PyClient;
use errors::AgcliError;
use types::{PyBalance, PyNetUid, PyNetwork};
use wallet::PyWallet;

/// Python bindings for the agcli Bittensor SDK.
#[pymodule]
fn _agcli(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    runtime::runtime();
    m.add("AgcliError", py.get_type::<AgcliError>())?;
    m.add_class::<PyBalance>()?;
    m.add_class::<PyNetUid>()?;
    m.add_class::<PyNetwork>()?;
    m.add_class::<PyClient>()?;
    m.add_class::<PyWallet>()?;
    Ok(())
}
