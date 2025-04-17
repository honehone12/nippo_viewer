use std::path::PathBuf;
use dotenvy_macro::dotenv;
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
    let key = dotenv!("SAVE_KEY");
    let key = BASE64_STANDARD.decode(key).map_err(print_err)?;
    Ok(key)
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Save {
    ct: Vec<u8>
}

impl Save {
    pub(crate) fn from_text(text: String) -> InvokeResult<Self> {
        let key = key()?;
        let key = Key::from_slice(&key);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut cipher_text = cipher.encrypt(&nonce, text.as_bytes())
            .map_err(print_err)?;

        let mut ct = nonce.to_vec();
        ct.append(&mut cipher_text);

        Ok(Self { 
            ct 
        })
    }

    pub(crate) fn into_text(&self) -> InvokeResult<String> {
        let key = key()?;
        let key = Key::from_slice(&key);
        let cipher = ChaCha20Poly1305::new(&key);
        const N: usize = 12;
        let nonce = Nonce::from_slice(&self.ct[..N]);
        let raw = cipher.decrypt(&nonce, &self.ct[N..])
            .map_err(print_err)?;
        let text = String::from_utf8(raw).map_err(print_err)?;
        Ok(text)
    }
}
