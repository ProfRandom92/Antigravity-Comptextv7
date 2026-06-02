use crate::codec::encode::{build_plain_v1_package, encode_package};
use crate::errors::RustCompTextError;
use std::fs;
use std::path::Path;

pub fn run(input: &Path, output: &Path) -> Result<(), RustCompTextError> {
    // Read input bytes
    let input_bytes = fs::read(input)?;

    // Parse UTF-8 text
    let input_text = String::from_utf8(input_bytes)?;

    // Build package
    let package = build_plain_v1_package(&input_text)?;

    // Encode package to string
    let serialized = encode_package(&package)?;

    // Write to output path
    fs::write(output, serialized.as_bytes())?;

    Ok(())
}
