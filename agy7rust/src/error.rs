use std::fmt;

#[derive(Debug)]
pub enum SparkError {
    ValidationError(String),
    SerializationError(String),
}

impl fmt::Display for SparkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
        }
    }
}

impl std::error::Error for SparkError {}
