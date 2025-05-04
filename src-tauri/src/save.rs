use crate::{
    error::{InvokeError, InvokeResult},
    print_err,
};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, ChaCha20Poly1305, Key, Nonce,
};
use keyring::{Entry, error::Error as KeyringError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::error;

#[inline]
pub(crate) fn persistent_file_path() -> InvokeResult<PathBuf> {
    let Some(mut path) = dirs::config_local_dir() else {
        error!("could not find app data local path");
        return Err(InvokeError::Path);
    };

    path.push("com.nippo-viewer.app");
    path.push(".nippo");
    Ok(path)
}

#[inline]
fn key() -> InvokeResult<Vec<u8>> {
    let user = whoami::username();
    let kr = Entry::new("com.nippo-viewer.app", &user)?;

    return match kr.get_secret() {
        Ok(v) => Ok(v),
        Err(KeyringError::NoEntry) => {
            let key = ChaCha20Poly1305::generate_key(&mut OsRng);
            let key = key.as_slice();
            kr.set_secret(key)?;
            Ok(key.to_vec())
        }
        Err(e) => Err(e.into())
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Save {
    cv: Vec<u8>,
}

const NONCE_SIZE: usize = 12;

impl Save {
    pub(crate) fn from_text(text: String) -> InvokeResult<Self> {
        let key = key()?;
        let key = Key::from_slice(&key);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut cipher_text = cipher.encrypt(&nonce, text.as_bytes()).map_err(print_err)?;

        let mut cv = nonce.to_vec();
        cv.append(&mut cipher_text);

        Ok(Self { cv })
    }

    pub(crate) fn into_text(&self) -> InvokeResult<String> {
        let key = key()?;
        let key = Key::from_slice(&key);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::from_slice(&self.cv[..NONCE_SIZE]);
        let raw = cipher.decrypt(&nonce, &self.cv[NONCE_SIZE..]).map_err(print_err)?;
        let text = String::from_utf8(raw).map_err(print_err)?;
        Ok(text)
    }
}
