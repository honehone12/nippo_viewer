use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum InvokeError {
    #[error("failed to request or unexpected response")]
    HttpRequestError
}

impl Serialize for InvokeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}
