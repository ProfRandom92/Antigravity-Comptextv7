use agy7rust::codec::package::canonical_json;
use agy7rust::sha256_hex;
use serde_json::Value;
use std::fs;

#[test]
fn test_pdf_extraction_fixture_contract_shape() {
    let fixture = fs::read_to_string("../examples/spark/pdf_extraction_fixture.json")
        .expect("failed to read PDF extraction fixture");
    let value: Value = serde_json::from_str(&fixture).expect("fixture should parse as JSON");

    assert_eq!(value["schema_version"], "PDF-EXTRACTION-V1");
    assert_non_empty_string(&value["source_file"], "source_file");
    assert_eq!(value["tool_metadata"]["converter"], "manual");
    assert_eq!(value["tool_metadata"]["extraction_mode"], "manual_fixture");

    let extracted_fields = &value["extracted_fields"];
    assert_non_empty_string(
        &extracted_fields["procedure_goal"],
        "extracted_fields.procedure_goal",
    );
    assert_non_empty_string(&extracted_fields["authority"], "extracted_fields.authority");
    assert_non_empty_array(
        &extracted_fields["decision_points"],
        "extracted_fields.decision_points",
    );
    assert_non_empty_array(
        &extracted_fields["required_documents"],
        "extracted_fields.required_documents",
    );
    assert_eq!(extracted_fields["review_required"], true);

    let tables = value["tables"]
        .as_array()
        .expect("tables should be an array");
    let first_table = tables.first().expect("fixture should include a table");
    let first_table_rows = first_table["rows"]
        .as_array()
        .expect("first table rows should be an array");
    assert_eq!(first_table_rows.len(), 3);

    let canonical = canonical_json(&value);
    let hash_once = sha256_hex(&canonical);
    let hash_twice = sha256_hex(canonical_json(&value));
    assert_eq!(hash_once, hash_twice);
    assert_eq!(hash_once.len(), 64);
}

fn assert_non_empty_string(value: &Value, label: &str) {
    assert!(
        value.as_str().is_some_and(|text| !text.trim().is_empty()),
        "{label} should be a non-empty string"
    );
}

fn assert_non_empty_array(value: &Value, label: &str) {
    assert!(
        value.as_array().is_some_and(|items| !items.is_empty()),
        "{label} should be a non-empty array"
    );
}
