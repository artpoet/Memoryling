use keyring_core::{Entry, Error};

const SERVICE: &str = "app.memoryling.desktop";
const USER: &str = "openai-api-key";

pub(crate) trait CredentialVault: Send + Sync {
    fn has_key(&self) -> Result<bool, String>;
    fn load_key(&self) -> Result<Option<String>, String>;
    fn save_key(&self, api_key: &str) -> Result<(), String>;
    fn delete_key(&self) -> Result<(), String>;
}

pub(crate) struct SystemCredentialVault;

impl SystemCredentialVault {
    #[cfg(target_os = "windows")]
    fn entry() -> Result<Entry, String> {
        use std::sync::OnceLock;

        static INITIALIZED: OnceLock<Result<(), String>> = OnceLock::new();
        INITIALIZED
            .get_or_init(|| {
                let store = windows_native_keyring_store::Store::new()
                    .map_err(|_| "Windows Credential Manager is unavailable.".to_string())?;
                keyring_core::set_default_store(store);
                Ok(())
            })
            .clone()?;
        Entry::new(SERVICE, USER)
            .map_err(|_| "Windows Credential Manager is unavailable.".to_string())
    }

    #[cfg(not(target_os = "windows"))]
    fn entry() -> Result<Entry, String> {
        Err("Secure API-key storage is available only in the Windows build.".to_string())
    }
}

impl CredentialVault for SystemCredentialVault {
    fn has_key(&self) -> Result<bool, String> {
        self.load_key().map(|key| key.is_some())
    }

    fn load_key(&self) -> Result<Option<String>, String> {
        match Self::entry()?.get_password() {
            Ok(secret) => Ok(Some(secret)),
            Err(Error::NoEntry) => Ok(None),
            Err(_) => Err(
                "Memoryling could not read the API key from Windows Credential Manager."
                    .to_string(),
            ),
        }
    }

    fn save_key(&self, api_key: &str) -> Result<(), String> {
        let key = api_key.trim();
        if key.len() < 20
            || key.len() > 512
            || key
                .chars()
                .any(|character| character.is_whitespace() || character.is_control())
        {
            return Err("Enter a valid OpenAI API key.".to_string());
        }
        Self::entry()?.set_password(key).map_err(|_| {
            "Memoryling could not save the API key in Windows Credential Manager.".to_string()
        })
    }

    fn delete_key(&self) -> Result<(), String> {
        match Self::entry()?.delete_credential() {
            Ok(()) | Err(Error::NoEntry) => Ok(()),
            Err(_) => Err(
                "Memoryling could not delete the API key from Windows Credential Manager."
                    .to_string(),
            ),
        }
    }
}

#[cfg(test)]
pub(crate) struct MemoryCredentialVault {
    key: std::sync::Mutex<Option<String>>,
}

#[cfg(test)]
impl MemoryCredentialVault {
    pub(crate) fn with_key(key: Option<&str>) -> Self {
        Self {
            key: std::sync::Mutex::new(key.map(str::to_string)),
        }
    }
}

#[cfg(test)]
impl CredentialVault for MemoryCredentialVault {
    fn has_key(&self) -> Result<bool, String> {
        Ok(self.key.lock().map_err(|_| "vault".to_string())?.is_some())
    }

    fn load_key(&self) -> Result<Option<String>, String> {
        Ok(self.key.lock().map_err(|_| "vault".to_string())?.clone())
    }

    fn save_key(&self, api_key: &str) -> Result<(), String> {
        *self.key.lock().map_err(|_| "vault".to_string())? = Some(api_key.to_string());
        Ok(())
    }

    fn delete_key(&self) -> Result<(), String> {
        *self.key.lock().map_err(|_| "vault".to_string())? = None;
        Ok(())
    }
}
