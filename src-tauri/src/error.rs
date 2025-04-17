use thiserror::Error;
use serde::Serialize;
use tracing::error;

#[derive(Error, Debug)]
pub enum InvokeError {
    #[error("failed to request or unexpected response")]
    Http(#[from] reqwest::Error),
    #[error("failed to read or write file")]
    Fs,
    #[error("failed to handle chrono")]
    Chrono,
    #[error("failed to serialize or deserialize json")]
    Json(#[from] serde_json::Error),
    #[error("invalid input")]
    Input,
    #[error("internal system error")]
    Internal,
    #[error("failed to encode or decode base64")]
    Base64(#[from] base64::DecodeError),
    #[error("failed to parse uuid")]
    Uuid(#[from] uuid::Error)
}

impl Serialize for InvokeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

#[inline]
pub(crate) fn print_err<E>(err: E) -> E
where E: std::error::Error {
    error!("{err}");
    err
}
