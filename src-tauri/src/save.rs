use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use base64::prelude::*;
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng}, 
    AeadCore, ChaCha20Poly1305, Key, Nonce
};
use crate::{error::{InvokeError, InvokeResult}, print_err};
use tracing::error;

#[inline]
pub(crate) fn persistent_file_path() -> InvokeResult<PathBuf> {
    let Some(mut path) = dirs::config_local_dir() else {
        error!("could not find app data local path");
        return Err(InvokeError::Unknown);
    };

    path.push("com.nippo-viewer.app");
    path.push(".nippo");
    Ok(path)
}

#[inline]
pub(crate) fn key() -> InvokeResult<Vec<u8>> {
    #[cfg(debug_assertions)]
    let key = option_env!("SAVE_KEY").expect("env for save key is not set"); 
    #[cfg(not(debug_assertions))]
    let key = env!("SAVE_KEY");

    let key = BASE64_STANDARD.decode(key).map_err(print_err)?;
    Ok(key)
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Save {
    pub(crate) n: Vec<u8>,
    pub(crate) tex: Vec<u8>
}

impl Save {
    pub(crate) fn from_text(text: String) -> InvokeResult<Self> {
        let key = key()?;
        let key = Key::from_slice(&key);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let cipher_text = cipher.encrypt(&nonce, text.as_bytes())
            .map_err(print_err)?;

        Ok(Self { 
            n: nonce.to_vec(), 
            tex: cipher_text 
        })
    }

    pub(crate) fn into_text(&self) -> InvokeResult<String> {
        let key = key()?;
        let key = Key::from_slice(&key);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = Nonce::from_slice(&self.n);
        let raw = cipher.decrypt(&nonce, self.tex.as_slice())
            .map_err(print_err)?;
        let text = String::from_utf8(raw).map_err(print_err)?;
        Ok(text)
    }
}
