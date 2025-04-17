use thiserror::Error;
use serde::Serialize;
use tracing::error;

#[derive(Error, Debug)]
pub enum InvokeError {
    #[error("this does not usually happen")]
    Unknown,
    #[error("failed to request or unexpected response")]
    Http(#[from] reqwest::Error),
    #[error("failed to read or write file")]
    Fs(#[from] tokio::io::Error),
    #[error("failed to handle chrono")]
    Chrono,
    #[error("failed to serialize or deserialize json")]
    Json(#[from] serde_json::Error),
    #[error("invalid input")]
    Input,
    #[error("internal system error")]
    Internal,
    #[error("invalid utf-8 encoding")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("failed to encode or decode base64")]
    Base64(#[from] base64::DecodeError),
    #[error("failed to parse uuid")]
    Uuid(#[from] uuid::Error),
    #[error("failed to encrypt or decrypt")]
    Aead(#[from] chacha20poly1305::Error)
}

impl Serialize for InvokeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

pub type InvokeResult<T> = Result<T, InvokeError>;

#[inline]
pub(crate) fn print_err<E>(err: E) -> E
where E: std::error::Error {
    error!("{err}");
    err
}
