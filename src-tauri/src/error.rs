use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum InvokeError {
    #[error("failed to request or unexpected response")]
    HttpError,
    #[error("failed to read or write file")]
    FsError,
    #[error("failed to handle chrono")]
    ChronoError,
    #[error("failed to serialize or deserialize json")]
    JsonError
}

impl Serialize for InvokeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}
