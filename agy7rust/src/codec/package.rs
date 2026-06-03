use crate::codec::hash::sha256_hex;
use serde_json;

pub fn sort_json_value(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut sorted_map = serde_json::Map::new();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                if let Some(val) = map.get(key) {
                    sorted_map.insert(key.clone(), sort_json_value(val));
                }
            }
            serde_json::Value::Object(sorted_map)
        }
        serde_json::Value::Array(arr) => {
            let sorted_arr: Vec<serde_json::Value> = arr.iter().map(sort_json_value).collect();
            serde_json::Value::Array(sorted_arr)
        }
        other => other.clone(),
    }
}

pub fn canonical_json(value: &serde_json::Value) -> String {
    let sorted = sort_json_value(value);
    serde_json::to_string(&sorted).unwrap_or_default()
}

pub fn collect_field_paths(value: &serde_json::Value) -> Vec<String> {
    let mut paths = Vec::new();
    collect_paths_recursive(value, "$", &mut paths);
    paths
}

fn collect_paths_recursive(value: &serde_json::Value, current_path: &str, paths: &mut Vec<String>) {
    paths.push(current_path.to_string());
    match value {
        serde_json::Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                let next_path = format!("{}.{}", current_path, key);
                collect_paths_recursive(&map[key], &next_path, paths);
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let next_path = format!("{}[{}]", current_path, i);
                collect_paths_recursive(val, &next_path, paths);
            }
        }
        _ => {}
    }
}

pub fn extract_commitment_tokens(value: &serde_json::Value) -> Vec<String> {
    let mut tokens = Vec::new();
    find_tokens_recursive(value, &mut tokens);
    tokens.sort();
    tokens.dedup();
    tokens
}

fn find_tokens_recursive(value: &serde_json::Value, tokens: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                let child = &map[key];
                if is_target_key(key) {
                    if let Some(s) = get_scalar_string(child) {
                        tokens.push(s);
                    }
                }
                find_tokens_recursive(child, tokens);
            }
        }
        serde_json::Value::Array(arr) => {
            for val in arr {
                find_tokens_recursive(val, tokens);
            }
        }
        _ => {}
    }
}

fn is_target_key(key: &str) -> bool {
    matches!(
        key,
        "case_id"
            | "document_type"
            | "authority"
            | "parcel_id"
            | "procedure_type"
            | "applicant"
            | "decision_recommendation"
    )
}

fn get_scalar_string(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

pub fn build_package_from_value(value: &serde_json::Value) -> anyhow::Result<serde_json::Value> {
    let payload = value.clone();
    let payload_canonical = canonical_json(&payload);
    let payload_sha256 = sha256_hex(payload_canonical);

    let field_paths = collect_field_paths(&payload);
    let commitment_tokens = extract_commitment_tokens(&payload);

    let mut sidecar = serde_json::json!({
        "schema_version": "KVTC7-SPARK-1",
        "source_type": "spark_extraction_json",
        "payload_sha256": payload_sha256,
        "field_paths": field_paths,
        "commitment_tokens": commitment_tokens,
        "tool_sequence": ["spark.extractor"]
    });

    let sidecar_preimage_canonical = canonical_json(&sidecar);
    let final_state_hash = sha256_hex(sidecar_preimage_canonical);

    if let serde_json::Value::Object(ref mut map) = sidecar {
        map.insert(
            "final_state_hash".to_string(),
            serde_json::Value::String(final_state_hash),
        );
    }

    let mut package = serde_json::json!({
        "schema": "SPARK-V7-PACKAGE",
        "version": 1,
        "payload": payload,
        "sidecar": sidecar
    });

    let package_preimage_canonical = canonical_json(&package);
    let integrity_hash = sha256_hex(package_preimage_canonical);

    if let serde_json::Value::Object(ref mut map) = package {
        map.insert(
            "integrity_hash".to_string(),
            serde_json::Value::String(integrity_hash),
        );
    }

    Ok(package)
}

pub fn verify_package_value(value: &serde_json::Value) -> anyhow::Result<()> {
    let pkg = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("Package is not a JSON object"))?;

    let schema = pkg
        .get("schema")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("schema mismatch: Missing schema"))?;
    if schema != "SPARK-V7-PACKAGE" {
        return Err(anyhow::anyhow!(
            "schema mismatch: expected SPARK-V7-PACKAGE, got {}",
            schema
        ));
    }

    let version = pkg
        .get("version")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow::anyhow!("version mismatch: Missing version"))?;
    if version != 1 {
        return Err(anyhow::anyhow!(
            "version mismatch: expected 1, got {}",
            version
        ));
    }

    let payload = pkg
        .get("payload")
        .ok_or_else(|| anyhow::anyhow!("Missing payload"))?;

    let sidecar_val = pkg
        .get("sidecar")
        .ok_or_else(|| anyhow::anyhow!("Missing sidecar"))?;
    let sidecar = sidecar_val
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("sidecar is not a JSON object"))?;

    let schema_version = sidecar
        .get("schema_version")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            anyhow::anyhow!("schema_version mismatch: Missing sidecar schema_version")
        })?;
    if schema_version != "KVTC7-SPARK-1" {
        return Err(anyhow::anyhow!(
            "schema_version mismatch: expected KVTC7-SPARK-1, got {}",
            schema_version
        ));
    }

    let source_type = sidecar
        .get("source_type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("source_type mismatch: Missing sidecar source_type"))?;
    if source_type != "spark_extraction_json" {
        return Err(anyhow::anyhow!(
            "source_type mismatch: expected spark_extraction_json, got {}",
            source_type
        ));
    }

    let payload_canonical = canonical_json(payload);
    let calculated_payload_sha256 = sha256_hex(payload_canonical);

    let expected_payload_sha256 = sidecar
        .get("payload_sha256")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            anyhow::anyhow!("payload_sha256 mismatch: Missing sidecar payload_sha256")
        })?;
    if calculated_payload_sha256 != expected_payload_sha256 {
        return Err(anyhow::anyhow!("payload_sha256 mismatch"));
    }

    let expected_final_state_hash = sidecar
        .get("final_state_hash")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            anyhow::anyhow!("final_state_hash mismatch: Missing sidecar final_state_hash")
        })?;

    let mut sidecar_preimage = sidecar_val.clone();
    if let serde_json::Value::Object(ref mut map) = sidecar_preimage {
        map.remove("final_state_hash");
    }
    let sidecar_preimage_canonical = canonical_json(&sidecar_preimage);
    let calculated_final_state_hash = sha256_hex(sidecar_preimage_canonical);

    if calculated_final_state_hash != expected_final_state_hash {
        return Err(anyhow::anyhow!("final_state_hash mismatch"));
    }

    let expected_integrity_hash = pkg
        .get("integrity_hash")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("integrity_hash mismatch: Missing integrity_hash"))?;

    let mut package_preimage = value.clone();
    if let serde_json::Value::Object(ref mut map) = package_preimage {
        map.remove("integrity_hash");
    }
    let package_preimage_canonical = canonical_json(&package_preimage);
    let calculated_integrity_hash = sha256_hex(package_preimage_canonical);

    if calculated_integrity_hash != expected_integrity_hash {
        return Err(anyhow::anyhow!("integrity_hash mismatch"));
    }

    Ok(())
}

pub fn replay_package_value(value: &serde_json::Value) -> anyhow::Result<serde_json::Value> {
    verify_package_value(value)?;

    let sidecar = value
        .get("sidecar")
        .ok_or_else(|| anyhow::anyhow!("Missing sidecar"))?;

    let source_type = sidecar
        .get("source_type")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let payload_sha256 = sidecar
        .get("payload_sha256")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let tool_sequence = sidecar
        .get("tool_sequence")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let commitment_tokens = sidecar
        .get("commitment_tokens")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let field_paths = sidecar
        .get("field_paths")
        .cloned()
        .unwrap_or(serde_json::Value::Null);

    let replay = serde_json::json!({
        "schema": "SPARK-V7-REPLAY",
        "source_type": source_type,
        "payload_sha256": payload_sha256,
        "tool_sequence": tool_sequence,
        "commitment_tokens": commitment_tokens,
        "field_paths": field_paths
    });

    Ok(replay)
}

pub fn get_value_by_path(
    value: &serde_json::Value,
    path: &str,
) -> anyhow::Result<serde_json::Value> {
    if !path.starts_with('$') {
        return Err(anyhow::anyhow!("unsupported path syntax"));
    }

    if path.contains('[') || path.contains(']') {
        return Err(anyhow::anyhow!("unsupported path syntax"));
    }

    if path == "$" {
        return Ok(value.clone());
    }

    if !path.starts_with("$.") {
        return Err(anyhow::anyhow!("unsupported path syntax"));
    }

    let parts = path[2..].split('.');
    let mut current = value;

    for part in parts {
        if part.is_empty() {
            return Err(anyhow::anyhow!("unsupported path syntax"));
        }
        if let serde_json::Value::Object(map) = current {
            if let Some(next_val) = map.get(part) {
                current = next_val;
            } else {
                return Err(anyhow::anyhow!("required field missing: {}", path));
            }
        } else {
            return Err(anyhow::anyhow!("required field missing: {}", path));
        }
    }

    Ok(current.clone())
}

pub fn validate_schema(
    input_val: &serde_json::Value,
    schema_val: &serde_json::Value,
) -> anyhow::Result<(String, usize, usize)> {
    let schema_obj = schema_val
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("schema is not a JSON object"))?;

    let schema_type = schema_obj
        .get("schema")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("schema mismatch"))?;
    if schema_type != "SPARK-V7-SCHEMA" {
        return Err(anyhow::anyhow!("schema mismatch"));
    }

    let version = schema_obj
        .get("version")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow::anyhow!("unsupported schema version"))?;
    if version != 1 {
        return Err(anyhow::anyhow!("unsupported schema version"));
    }

    let schema_name = schema_obj
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing schema name"))?
        .to_string();

    let required_paths_val = schema_obj
        .get("required_field_paths")
        .ok_or_else(|| anyhow::anyhow!("missing required_field_paths"))?;
    let required_paths = required_paths_val
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("missing required_field_paths"))?;

    let mut path_strings = Vec::new();
    for p in required_paths {
        if let Some(s) = p.as_str() {
            path_strings.push(s);
        } else {
            return Err(anyhow::anyhow!("missing required_field_paths"));
        }
    }

    let required_count = path_strings.len();
    let mut checked_count = 0;

    for path in path_strings {
        let val = match get_value_by_path(input_val, path) {
            Ok(v) => v,
            Err(e) => {
                let err_msg = e.to_string();
                if err_msg.contains("unsupported path syntax") {
                    return Err(e);
                } else {
                    return Err(anyhow::anyhow!("required field missing: {}", path));
                }
            }
        };

        match val {
            serde_json::Value::String(s) => {
                if s.trim().is_empty() {
                    return Err(anyhow::anyhow!("required field empty: {}", path));
                }
            }
            serde_json::Value::Number(_) | serde_json::Value::Bool(_) => {}
            serde_json::Value::Null
            | serde_json::Value::Object(_)
            | serde_json::Value::Array(_) => {
                return Err(anyhow::anyhow!("required field not scalar: {}", path));
            }
        }
        checked_count += 1;
    }

    Ok((schema_name, required_count, checked_count))
}
