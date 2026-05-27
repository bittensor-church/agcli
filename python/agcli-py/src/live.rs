use std::sync::Arc;

use agcli::types::network::NetUid;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3_async_runtimes::tokio::future_into_py;

use crate::client::PyClient;
use crate::errors::map_error;
use crate::types::netuid_from_py;

#[pyfunction]
#[pyo3(signature = (client, interval_secs=5))]
pub fn live_dynamic<'py>(
    py: Python<'py>,
    client: PyRef<'_, PyClient>,
    interval_secs: u64,
) -> PyResult<Bound<'py, PyAny>> {
    let shared = client.shared_client();
    future_into_py(py, async move {
        let guard = shared.lock().await;
        agcli::live::live_dynamic(&guard, interval_secs)
            .await
            .map_err(map_error)
    })
}

#[pyfunction]
#[pyo3(signature = (client, netuid, interval_secs=5))]
pub fn live_metagraph<'py>(
    py: Python<'py>,
    client: PyRef<'_, PyClient>,
    netuid: Bound<'_, PyAny>,
    interval_secs: u64,
) -> PyResult<Bound<'py, PyAny>> {
    let netuid: NetUid = netuid_from_py(&netuid)?;
    let shared = client.shared_client();
    future_into_py(py, async move {
        let guard = shared.lock().await;
        agcli::live::live_metagraph(&guard, netuid, interval_secs)
            .await
            .map_err(map_error)
    })
}

#[pyfunction]
#[pyo3(signature = (client, coldkey_ss58, interval_secs=5))]
pub fn live_portfolio<'py>(
    py: Python<'py>,
    client: PyRef<'_, PyClient>,
    coldkey_ss58: String,
    interval_secs: u64,
) -> PyResult<Bound<'py, PyAny>> {
    let shared = Arc::clone(&client.shared_client());
    future_into_py(py, async move {
        let guard = shared.lock().await;
        agcli::live::live_portfolio(&guard, &coldkey_ss58, interval_secs)
            .await
            .map_err(map_error)
    })
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(live_dynamic, m)?)?;
    m.add_function(wrap_pyfunction!(live_metagraph, m)?)?;
    m.add_function(wrap_pyfunction!(live_portfolio, m)?)?;
    Ok(())
}
