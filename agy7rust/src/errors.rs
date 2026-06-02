use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustCompTextError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("UTF-8 validation error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Invalid schema error: {0}")]
    InvalidSchema(String),

    #[error("Verification error: {0}")]
    Verification(String),

    #[error("CLI error: {0}")]
    Cli(String),
}
