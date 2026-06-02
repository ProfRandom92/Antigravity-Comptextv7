use crate::codec::schema::ContextPackage;
use crate::errors::RustCompTextError;

pub fn decode_package(input: &str) -> Result<ContextPackage, RustCompTextError> {
    let package: ContextPackage = serde_json::from_str(input).map_err(RustCompTextError::Json)?;
    Ok(package)
}
