use agcli::Wallet;
use pyo3::prelude::*;

use crate::errors::map_error;

#[pyclass(name = "Wallet", module = "agcli._agcli")]
pub struct PyWallet {
    inner: Wallet,
}

#[pymethods]
impl PyWallet {
    #[staticmethod]
    fn open(path: String) -> PyResult<Self> {
        let wallet = Wallet::open(path).map_err(map_error)?;
        Ok(Self { inner: wallet })
    }

    #[staticmethod]
    #[pyo3(signature = (wallet_dir, name, password, hotkey_name=None))]
    fn create(
        wallet_dir: String,
        name: String,
        password: String,
        hotkey_name: Option<String>,
    ) -> PyResult<(Self, String, String)> {
        let hotkey_name = hotkey_name.unwrap_or_else(|| "default".to_string());
        let (wallet, coldkey_mnemonic, hotkey_mnemonic) =
            Wallet::create(wallet_dir, &name, &password, &hotkey_name).map_err(map_error)?;
        Ok((
            Self { inner: wallet },
            coldkey_mnemonic,
            hotkey_mnemonic,
        ))
    }

    #[staticmethod]
    #[pyo3(signature = (wallet_dir, name, mnemonic, password))]
    fn import_from_mnemonic(
        wallet_dir: String,
        name: String,
        mnemonic: String,
        password: String,
    ) -> PyResult<Self> {
        let wallet =
            Wallet::import_from_mnemonic(wallet_dir, &name, &mnemonic, &password).map_err(map_error)?;
        Ok(Self { inner: wallet })
    }

    #[staticmethod]
    fn list_wallets(wallet_dir: String) -> PyResult<Vec<String>> {
        Wallet::list_wallets(wallet_dir).map_err(map_error)
    }

    fn unlock_coldkey(&mut self, password: String) -> PyResult<()> {
        self.inner.unlock_coldkey(&password).map_err(map_error)
    }

    fn load_hotkey(&mut self, hotkey_name: String) -> PyResult<()> {
        self.inner.load_hotkey(&hotkey_name).map_err(map_error)
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[getter]
    fn path(&self) -> String {
        self.inner.path.display().to_string()
    }

    #[getter]
    fn coldkey_ss58(&self) -> Option<String> {
        self.inner.coldkey_ss58().map(str::to_string)
    }

    #[getter]
    fn hotkey_ss58(&self) -> Option<String> {
        self.inner.hotkey_ss58().map(str::to_string)
    }

    fn list_hotkeys(&self) -> PyResult<Vec<String>> {
        self.inner.list_hotkeys().map_err(map_error)
    }

    fn __repr__(&self) -> String {
        format!(
            "Wallet(name={:?}, path={:?})",
            self.inner.name,
            self.inner.path
        )
    }
}
