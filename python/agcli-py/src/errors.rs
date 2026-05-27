use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

create_exception!(_agcli, AgcliError, PyException);
create_exception!(_agcli, NetworkError, AgcliError);
create_exception!(_agcli, AuthError, AgcliError);
create_exception!(_agcli, ValidationError, AgcliError);
create_exception!(_agcli, ChainError, AgcliError);
create_exception!(_agcli, TimeoutError, AgcliError);
create_exception!(_agcli, IOError, AgcliError);

pub fn map_error(err: anyhow::Error) -> PyErr {
    let message = format!("{:#}", err);
    let code = agcli::error::classify(&err);
    let hint = agcli::error::hint(code, &message).map(str::to_string);
    Python::with_gil(|py| {
        let py_err = match code {
            agcli::error::exit_code::NETWORK => NetworkError::new_err(message),
            agcli::error::exit_code::AUTH => AuthError::new_err(message),
            agcli::error::exit_code::VALIDATION => ValidationError::new_err(message),
            agcli::error::exit_code::CHAIN => ChainError::new_err(message),
            agcli::error::exit_code::TIMEOUT => TimeoutError::new_err(message),
            agcli::error::exit_code::IO => IOError::new_err(message),
            _ => AgcliError::new_err(message),
        };
        let value = py_err.value(py);
        let _ = value.setattr("code", code);
        if let Some(h) = hint {
            let _ = value.setattr("hint", h);
        }
        py_err
    })
}

#[pyfunction]
pub fn raise_test_error(message: String) -> PyResult<()> {
    Err(map_error(anyhow::anyhow!(message)))
}
