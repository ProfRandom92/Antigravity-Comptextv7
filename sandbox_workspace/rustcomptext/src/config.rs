use crate::errors::RustCompTextError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub version: u32,
    pub codec: String,
    pub hash_algorithm: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            codec: "plain.v1".to_string(),
            hash_algorithm: "sha256".to_string(),
        }
    }
}

pub fn config_to_stable_json(config: &Config) -> Result<String, RustCompTextError> {
    let serialized = serde_json::to_string(config)?;
    Ok(serialized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        assert_eq!(config.version, 1);
        assert_eq!(config.codec, "plain.v1");
        assert_eq!(config.hash_algorithm, "sha256");
    }

    #[test]
    fn test_config_stable_json_determinism() {
        let config = Config::default();
        let json1 = config_to_stable_json(&config).unwrap();
        let json2 = config_to_stable_json(&config).unwrap();
        assert_eq!(json1, json2);
    }
}
