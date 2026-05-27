use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

create_exception!(_agcli, AgcliError, PyException);

pub fn map_error(err: anyhow::Error) -> PyErr {
    let message = format!("{:#}", err);
    let code = agcli::error::classify(&err);
    let hint = agcli::error::hint(code, &message).map(str::to_string);
    Python::with_gil(|py| {
        let py_err = AgcliError::new_err(message);
        let value = py_err.value(py);
        let _ = value.setattr("code", code);
        if let Some(h) = hint {
            let _ = value.setattr("hint", h);
        }
        py_err
    })
}
