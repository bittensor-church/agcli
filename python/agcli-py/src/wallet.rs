use std::sync::Arc;

use agcli::chain::subxt::ext::sp_core::{crypto::Ss58Codec, sr25519, Pair as _};
use agcli::Wallet;
use pyo3::prelude::*;
use tokio::sync::Mutex;

use crate::errors::map_error;
use crate::runtime::runtime;

pub(crate) type SharedWallet = Arc<Mutex<Wallet>>;

#[pyclass(name = "Wallet", module = "agcli._agcli")]
pub struct PyWallet {
    inner: SharedWallet,
}

impl PyWallet {
    pub(crate) fn shared_wallet(&self) -> SharedWallet {
        Arc::clone(&self.inner)
    }
}

fn wrap_wallet(wallet: Wallet) -> PyWallet {
    PyWallet {
        inner: Arc::new(Mutex::new(wallet)),
    }
}

#[pymethods]
impl PyWallet {
    #[staticmethod]
    fn open(path: String) -> PyResult<Self> {
        let wallet = Wallet::open(path).map_err(map_error)?;
        Ok(wrap_wallet(wallet))
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
        Ok((wrap_wallet(wallet), coldkey_mnemonic, hotkey_mnemonic))
    }

    #[staticmethod]
    #[pyo3(signature = (wallet_dir, name, mnemonic, password))]
    fn import_from_mnemonic(
        wallet_dir: String,
        name: String,
        mnemonic: String,
        password: String,
    ) -> PyResult<Self> {
        let wallet = Wallet::import_from_mnemonic(wallet_dir, &name, &mnemonic, &password)
            .map_err(map_error)?;
        Ok(wrap_wallet(wallet))
    }

    #[staticmethod]
    fn create_from_uri(wallet_dir: String, uri: String, password: String) -> PyResult<Self> {
        let wallet = Wallet::create_from_uri(wallet_dir, &uri, &password).map_err(map_error)?;
        Ok(wrap_wallet(wallet))
    }

    #[staticmethod]
    fn list_wallets(wallet_dir: String) -> PyResult<Vec<String>> {
        Wallet::list_wallets(wallet_dir).map_err(map_error)
    }

    fn unlock_coldkey(&self, password: String) -> PyResult<()> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let mut wallet = wallet.lock().await;
                wallet.unlock_coldkey(&password)
            })
            .map_err(map_error)
    }

    fn unlock_coldkey_with_password(&self, password: String) -> PyResult<()> {
        self.unlock_coldkey(password)
    }

    fn load_hotkey(&self, hotkey_name: String) -> PyResult<()> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let mut wallet = wallet.lock().await;
                wallet.load_hotkey(&hotkey_name)
            })
            .map_err(map_error)
    }

    fn lock(&self) -> PyResult<()> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let mut wallet = wallet.lock().await;
                let reopened = Wallet::open(&wallet.path)?;
                *wallet = reopened;
                Ok(())
            })
            .map_err(map_error)
    }

    #[getter]
    fn is_coldkey_unlocked(&self) -> PyResult<bool> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let wallet = wallet.lock().await;
                Ok(wallet.coldkey().is_ok())
            })
            .map_err(map_error)
    }

    #[getter]
    fn is_hotkey_loaded(&self) -> PyResult<bool> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let wallet = wallet.lock().await;
                Ok(wallet.hotkey().is_ok())
            })
            .map_err(map_error)
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let wallet = wallet.lock().await;
                Ok(wallet.name.clone())
            })
            .map_err(map_error)
    }

    #[getter]
    fn path(&self) -> PyResult<String> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let wallet = wallet.lock().await;
                Ok(wallet.path.display().to_string())
            })
            .map_err(map_error)
    }

    #[getter]
    fn coldkey_ss58(&self) -> PyResult<Option<String>> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let wallet = wallet.lock().await;
                Ok(wallet.coldkey_ss58().map(str::to_string))
            })
            .map_err(map_error)
    }

    #[getter]
    fn coldkey_public_ss58(&self) -> PyResult<Option<String>> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let wallet = wallet.lock().await;
                Ok(wallet.coldkey_ss58().map(str::to_string))
            })
            .map_err(map_error)
    }

    #[getter]
    fn hotkey_ss58(&self) -> PyResult<Option<String>> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let wallet = wallet.lock().await;
                Ok(wallet.hotkey_ss58().map(str::to_string))
            })
            .map_err(map_error)
    }

    fn list_hotkeys(&self) -> PyResult<Vec<String>> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let wallet = wallet.lock().await;
                wallet.list_hotkeys()
            })
            .map_err(map_error)
    }

    fn sign_message(&self, role: &str, message: Vec<u8>) -> PyResult<Vec<u8>> {
        let wallet = self.shared_wallet();
        runtime()
            .block_on(async move {
                let wallet = wallet.lock().await;
                let signature = match role {
                    "coldkey" => wallet.coldkey()?.sign(&message),
                    "hotkey" => wallet.hotkey()?.sign(&message),
                    other => {
                        anyhow::bail!("invalid role '{other}', expected 'coldkey' or 'hotkey'")
                    }
                };
                Ok(signature.0.to_vec())
            })
            .map_err(map_error)
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

    fn __getstate__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("Wallet"))
    }

    fn __reduce__(&self) -> PyResult<()> {
        Err(crate::errors::pickle_blocked("Wallet"))
    }

    fn __repr__(&self) -> String {
        let wallet = self.shared_wallet();
        match runtime().block_on(async move {
            let wallet = wallet.lock().await;
            Ok::<String, anyhow::Error>(format!(
                "Wallet(name={:?}, path={:?})",
                wallet.name, wallet.path
            ))
        }) {
            Ok(value) => value,
            Err(_) => "Wallet(<unavailable>)".to_string(),
        }
    }
}
