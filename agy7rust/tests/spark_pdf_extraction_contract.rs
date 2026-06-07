use agy7rust::validate_pdf_extraction_contract_value;
use serde_json::Value;
use std::fs;

#[test]
fn test_pdf_extraction_fixture_contract_shape() {
    let fixture = fs::read_to_string("../examples/spark/pdf_extraction_fixture.json")
        .expect("failed to read PDF extraction fixture");
    let value: Value = serde_json::from_str(&fixture).expect("fixture should parse as JSON");
    let validation =
        validate_pdf_extraction_contract_value(&value).expect("fixture contract should validate");

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

    assert_eq!(validation.page_count, 2);
    assert_eq!(validation.table_count, 1);
    assert_eq!(validation.first_table_row_count, 3);
    assert_eq!(validation.canonical_hash.len(), 64);
    assert!(!validation.canonical_json.is_empty());
}

#[test]
fn test_pdf_extraction_contract_rejects_wrong_schema_version() {
    let mut value = load_fixture_value();
    value["schema_version"] = Value::String("PDF-EXTRACTION-V0".to_string());

    let err = validate_pdf_extraction_contract_value(&value)
        .unwrap_err()
        .to_string();
    assert_eq!(err, "schema_version mismatch");
}

#[test]
fn test_pdf_extraction_contract_rejects_missing_required_field() {
    let mut value = load_fixture_value();
    value["extracted_fields"]
        .as_object_mut()
        .expect("extracted_fields should be an object")
        .remove("procedure_goal");

    let err = validate_pdf_extraction_contract_value(&value)
        .unwrap_err()
        .to_string();
    assert!(err.contains("missing field `procedure_goal`"));
}

#[test]
fn test_pdf_extraction_contract_rejects_unsupported_converter() {
    let mut value = load_fixture_value();
    value["tool_metadata"]["converter"] = Value::String("unsupported".to_string());

    let err = validate_pdf_extraction_contract_value(&value)
        .unwrap_err()
        .to_string();
    assert_eq!(err, "tool_metadata.converter unsupported");
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

fn load_fixture_value() -> Value {
    let fixture = fs::read_to_string("../examples/spark/pdf_extraction_fixture.json")
        .expect("failed to read PDF extraction fixture");
    serde_json::from_str(&fixture).expect("fixture should parse as JSON")
}
