use crate::errors::RustCompTextError;
use sha2::{Digest, Sha256};

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn compute_sha256(data: &[u8]) -> Result<String, RustCompTextError> {
    Ok(sha256_hex(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_empty() {
        let expected = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        assert_eq!(sha256_hex(b""), expected);
    }

    #[test]
    fn test_sha256_abc() {
        let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        assert_eq!(sha256_hex(b"abc"), expected);
    }

    #[test]
    fn test_sha256_unicode() {
        let input = "unicode sentinel Δpressure=2.4bar emoji=⚠️ sparse_anchor=ALARM-0001";
        let hash = sha256_hex(input.as_bytes());
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_sha256_determinism() {
        let input = b"deterministic input 123";
        let hash1 = sha256_hex(input);
        let hash2 = sha256_hex(input);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_sha256_different() {
        let hash1 = sha256_hex(b"input1");
        let hash2 = sha256_hex(b"input2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_sha256_lowercase_and_length() {
        let hash = sha256_hex(b"test lowercase");
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_lowercase() || c.is_numeric()));
    }
}
