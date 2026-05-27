use agcli::Wallet;
use agcli::chain::subxt::ext::sp_core::{crypto::Ss58Codec, sr25519, Pair as _};
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
    fn create_from_uri(wallet_dir: String, uri: String, password: String) -> PyResult<Self> {
        let wallet = Wallet::create_from_uri(wallet_dir, &uri, &password).map_err(map_error)?;
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
    fn coldkey_public_ss58(&self) -> Option<String> {
        self.inner.coldkey_ss58().map(str::to_string)
    }

    #[getter]
    fn hotkey_ss58(&self) -> Option<String> {
        self.inner.hotkey_ss58().map(str::to_string)
    }

    fn list_hotkeys(&self) -> PyResult<Vec<String>> {
        self.inner.list_hotkeys().map_err(map_error)
    }

    fn sign_message(&self, role: &str, message: Vec<u8>) -> PyResult<Vec<u8>> {
        let signature = match role {
            "coldkey" => self.inner.coldkey().map_err(map_error)?.sign(&message),
            "hotkey" => self.inner.hotkey().map_err(map_error)?.sign(&message),
            other => {
                return Err(map_error(anyhow::anyhow!(
                    "invalid role '{other}', expected 'coldkey' or 'hotkey'"
                )))
            }
        };
        Ok(signature.0.to_vec())
    }

    #[staticmethod]
    fn verify_message(ss58: String, message: Vec<u8>, signature: Vec<u8>) -> PyResult<bool> {
        if signature.len() != 64 {
            return Err(map_error(anyhow::anyhow!(
                "invalid signature length {}, expected 64 bytes",
                signature.len()
            )));
        }
        let public = sr25519::Public::from_ss58check(&ss58)
            .map_err(|e| map_error(anyhow::anyhow!("{e}")))?;
        let mut signature_bytes = [0u8; 64];
        signature_bytes.copy_from_slice(&signature);
        let sig = sr25519::Signature::from_raw(signature_bytes);
        Ok(sr25519::Pair::verify(&sig, &message, &public))
    }

    fn __repr__(&self) -> String {
        format!(
            "Wallet(name={:?}, path={:?})",
            self.inner.name,
            self.inner.path
        )
    }
}
