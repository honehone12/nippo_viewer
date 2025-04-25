use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum InvokeError {
    #[error("arithmetic error")]
    Arithmetic,
    #[error("could not resolve path")]
    Path,
    #[error("failed to request or unexpected response")]
    Http(#[from] reqwest::Error),
    #[error("failed on fs or ipc")]
    Io(#[from] std::io::Error),
    #[error("failed to handle chrono")]
    Chrono,
    #[error("failed to handle system time")]
    SystemTime(#[from] std::time::SystemTimeError),
    #[error("failed to serialize or deserialize json")]
    Json(#[from] serde_json::Error),
    #[error("invalid input")]
    Input,
    #[error("invalid utf-8 encoding")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("failed to encode or decode base64")]
    Base64(#[from] base64::DecodeError),
    #[error("failed to parse uuid")]
    Uuid(#[from] uuid::Error),
    #[error("failed to encrypt or decrypt")]
    Aead(#[from] chacha20poly1305::Error),
}

impl Serialize for InvokeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type InvokeResult<T> = Result<T, InvokeError>;

#[inline]
pub(crate) fn print_err<E>(err: E) -> E
where
    E: std::error::Error,
{
    error!("{err}");
    err
}
