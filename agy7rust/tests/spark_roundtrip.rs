use agy7rust::codec::package::{
    build_package_from_value, canonical_json, collect_field_paths, extract_commitment_tokens,
    replay_package_value, verify_package_value,
};
use serde_json::json;

#[test]
fn test_canonical_json_sorting() {
    let original = json!({
        "z": 1,
        "a": {
            "c": 3,
            "b": 2
        },
        "m": [4, 5, 6]
    });

    let canonical = canonical_json(&original);
    // Keys should be sorted alphabetically: a, m, z
    // Under a, keys should be sorted: b, c
    assert_eq!(canonical, r#"{"a":{"b":2,"c":3},"m":[4,5,6],"z":1}"#);
}

#[test]
fn test_field_paths_collection() {
    let original = json!({
        "case_id": "SPARK-1",
        "metadata": {
            "hash": "abc"
        },
        "items": [
            {"name": "first"}
        ]
    });

    let paths = collect_field_paths(&original);
    // Should be sorted alphabetically before traversal
    assert!(paths.contains(&"$".to_string()));
    assert!(paths.contains(&"$.case_id".to_string()));
    assert!(paths.contains(&"$.metadata".to_string()));
    assert!(paths.contains(&"$.metadata.hash".to_string()));
    assert!(paths.contains(&"$.items".to_string()));
    assert!(paths.contains(&"$.items[0]".to_string()));
    assert!(paths.contains(&"$.items[0].name".to_string()));
}

#[test]
fn test_commitment_tokens_extraction() {
    let original = json!({
        "case_id": "SPARK-1",
        "document_type": "PDF",
        "authority": "Amt",
        "procedure_type": "Normal",
        "applicant": "XYZ GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "123",
                "decision_recommendation": "Yes"
            }
        },
        "ignored_field": "secret"
    });

    let tokens = extract_commitment_tokens(&original);
    assert_eq!(tokens.len(), 7);
    assert!(tokens.contains(&"SPARK-1".to_string()));
    assert!(tokens.contains(&"PDF".to_string()));
    assert!(tokens.contains(&"Amt".to_string()));
    assert!(tokens.contains(&"Normal".to_string()));
    assert!(tokens.contains(&"XYZ GmbH".to_string()));
    assert!(tokens.contains(&"123".to_string()));
    assert!(tokens.contains(&"Yes".to_string()));
    // Ignored field value should not be extracted
    assert!(!tokens.contains(&"secret".to_string()));
}

#[test]
fn test_package_verification_lifecycle() {
    let original = json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    // 1. Build package
    let package = build_package_from_value(&original).expect("Build package failed");

    // 2. Verification of valid package
    assert!(verify_package_value(&package).is_ok());

    // 3. Mutate payload (tampering)
    let mut mutated_payload = package.clone();
    mutated_payload["payload"]["extraction"]["fields"]["parcel_id"] = json!("TAMPERED");
    assert!(verify_package_value(&mutated_payload).is_err());

    // 4. Mutate sidecar hash (tampering)
    let mut mutated_sidecar = package.clone();
    mutated_sidecar["sidecar"]["payload_sha256"] =
        json!("0000000000000000000000000000000000000000000000000000000000000000");
    assert!(verify_package_value(&mutated_sidecar).is_err());

    // 5. Mutate integrity hash (tampering)
    let mut mutated_integrity = package.clone();
    mutated_integrity["integrity_hash"] =
        json!("0000000000000000000000000000000000000000000000000000000000000000");
    assert!(verify_package_value(&mutated_integrity).is_err());
}

#[test]
fn test_replay_canonical_format() {
    let original = json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let package = build_package_from_value(&original).expect("Build package failed");
    let replay = replay_package_value(&package).expect("Replay failed");

    assert_eq!(replay["schema"].as_str().unwrap(), "SPARK-V7-REPLAY");
    assert_eq!(
        replay["source_type"].as_str().unwrap(),
        "spark_extraction_json"
    );
    assert!(replay.get("payload_sha256").is_some());
    assert!(replay.get("tool_sequence").is_some());
    assert!(replay.get("commitment_tokens").is_some());
    assert!(replay.get("field_paths").is_some());
    // Should NOT contain the original payload fields directly
    assert!(replay.get("payload").is_none());
}

#[test]
fn test_schema_checking_scenarios() {
    let valid_input = json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id",
            "$.document_type",
            "$.authority",
            "$.procedure_type",
            "$.applicant",
            "$.extraction.fields.parcel_id",
            "$.extraction.fields.location",
            "$.extraction.fields.project_type",
            "$.extraction.fields.decision_recommendation",
            "$.metadata.source_pdf_sha256"
        ],
        "non_claims": []
    });

    // 1. Valid fixture passes schema-check logic
    let res = agy7rust::codec::package::validate_schema(&valid_input, &valid_schema);
    assert!(res.is_ok());
    let (name, required, checked) = res.unwrap();
    assert_eq!(name, "genehmigung_v1");
    assert_eq!(required, 10);
    assert_eq!(checked, 10);

    // 2. Missing $.extraction.fields.parcel_id fails
    let mut missing_parcel = valid_input.clone();
    missing_parcel["extraction"]["fields"]
        .as_object_mut()
        .unwrap()
        .remove("parcel_id");
    let res = agy7rust::codec::package::validate_schema(&missing_parcel, &valid_schema);
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "required field missing: $.extraction.fields.parcel_id"
    );

    // 3. Empty $.extraction.fields.decision_recommendation fails
    let mut empty_decision = valid_input.clone();
    empty_decision["extraction"]["fields"]["decision_recommendation"] = json!("   ");
    let res = agy7rust::codec::package::validate_schema(&empty_decision, &valid_schema);
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "required field empty: $.extraction.fields.decision_recommendation"
    );

    // 4. Object value at required scalar path fails
    let mut object_instead_of_scalar = valid_input.clone();
    object_instead_of_scalar["extraction"]["fields"]["parcel_id"] = json!({ "nested": "object" });
    let res = agy7rust::codec::package::validate_schema(&object_instead_of_scalar, &valid_schema);
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "required field not scalar: $.extraction.fields.parcel_id"
    );

    // 5. Invalid schema name fails
    let mut bad_schema_type = valid_schema.clone();
    bad_schema_type["schema"] = json!("SPARK-V7-INVALID");
    let res = agy7rust::codec::package::validate_schema(&valid_input, &bad_schema_type);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "schema mismatch");

    // 6. Unsupported path syntax fails cleanly
    let mut unsupported_path_schema = valid_schema.clone();
    unsupported_path_schema["required_field_paths"] = json!(["$.extraction.fields[0].parcel_id"]);
    let res = agy7rust::codec::package::validate_schema(&valid_input, &unsupported_path_schema);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "unsupported path syntax");
}

#[test]
fn test_context_model_shape_accepts_minimal_valid_context() {
    use agy7rust::context::model::{ContextValidation, OperationalContext};

    let ctx = OperationalContext {
        context_id: "ctx_123".to_string(),
        source_package_hash: "hash_456".to_string(),
        schema_name: "genehmigung_v1".to_string(),
        schema_version: 1,
        required_field_paths: vec!["$.case_id".to_string()],
        satisfied_field_paths: vec![],
        missing_field_paths: vec![],
        constraints: vec![],
        required_order: vec![],
        dependency_edges: vec![],
        blockers: vec![],
        recovery_paths: vec![],
        validation: ContextValidation {
            valid: true,
            failure_labels: vec![],
            issues: vec![],
        },
        non_claims: vec!["no_mcp_integration".to_string()],
    };

    assert!(ctx.validate_model_shape().is_ok());
}

#[test]
fn test_context_model_sort_stable_is_deterministic() {
    use agy7rust::context::model::{ContextDependencyEdge, ContextValidation, OperationalContext};

    let mut ctx = OperationalContext {
        context_id: "ctx_123".to_string(),
        source_package_hash: "hash_456".to_string(),
        schema_name: "genehmigung_v1".to_string(),
        schema_version: 1,
        required_field_paths: vec!["$.document_type".to_string(), "$.case_id".to_string()],
        satisfied_field_paths: vec!["$.applicant".to_string(), "$.authority".to_string()],
        missing_field_paths: vec!["$.parcel_id".to_string()],
        constraints: vec!["no_unsafe".to_string(), "no_side_effects".to_string()],
        required_order: vec!["b".to_string(), "a".to_string()],
        dependency_edges: vec![
            ContextDependencyEdge {
                source: "b".to_string(),
                target: "y".to_string(),
            },
            ContextDependencyEdge {
                source: "a".to_string(),
                target: "z".to_string(),
            },
            ContextDependencyEdge {
                source: "b".to_string(),
                target: "x".to_string(),
            },
        ],
        blockers: vec![ContextDependencyEdge {
            source: "c".to_string(),
            target: "w".to_string(),
        }],
        recovery_paths: vec![ContextDependencyEdge {
            source: "d".to_string(),
            target: "v".to_string(),
        }],
        validation: ContextValidation {
            valid: false,
            failure_labels: vec!["label_b".to_string(), "label_a".to_string()],
            issues: vec!["issue_b".to_string(), "issue_a".to_string()],
        },
        non_claims: vec!["claim_b".to_string(), "claim_a".to_string()],
    };

    ctx.sort_stable();

    assert_eq!(
        ctx.required_field_paths,
        vec!["$.case_id".to_string(), "$.document_type".to_string()]
    );
    assert_eq!(
        ctx.satisfied_field_paths,
        vec!["$.applicant".to_string(), "$.authority".to_string()]
    );
    assert_eq!(ctx.missing_field_paths, vec!["$.parcel_id".to_string()]);
    assert_eq!(
        ctx.constraints,
        vec!["no_side_effects".to_string(), "no_unsafe".to_string()]
    );
    assert_eq!(ctx.required_order, vec!["a".to_string(), "b".to_string()]);

    assert_eq!(
        ctx.dependency_edges,
        vec![
            ContextDependencyEdge {
                source: "a".to_string(),
                target: "z".to_string()
            },
            ContextDependencyEdge {
                source: "b".to_string(),
                target: "x".to_string()
            },
            ContextDependencyEdge {
                source: "b".to_string(),
                target: "y".to_string()
            },
        ]
    );

    assert_eq!(
        ctx.validation.failure_labels,
        vec!["label_a".to_string(), "label_b".to_string()]
    );
    assert_eq!(
        ctx.validation.issues,
        vec!["issue_a".to_string(), "issue_b".to_string()]
    );
    assert_eq!(
        ctx.non_claims,
        vec!["claim_a".to_string(), "claim_b".to_string()]
    );
}

#[test]
fn test_context_model_rejects_missing_context_id() {
    use agy7rust::context::model::{ContextValidation, OperationalContext};

    let ctx = OperationalContext {
        context_id: "".to_string(),
        source_package_hash: "hash_456".to_string(),
        schema_name: "genehmigung_v1".to_string(),
        schema_version: 1,
        required_field_paths: vec!["$.case_id".to_string()],
        satisfied_field_paths: vec![],
        missing_field_paths: vec![],
        constraints: vec![],
        required_order: vec![],
        dependency_edges: vec![],
        blockers: vec![],
        recovery_paths: vec![],
        validation: ContextValidation {
            valid: true,
            failure_labels: vec![],
            issues: vec![],
        },
        non_claims: vec!["no_mcp_integration".to_string()],
    };

    assert_eq!(
        ctx.validate_model_shape().unwrap_err(),
        "missing context_id"
    );
}

#[test]
fn test_context_model_rejects_missing_source_package_hash() {
    use agy7rust::context::model::{ContextValidation, OperationalContext};

    let ctx = OperationalContext {
        context_id: "ctx_123".to_string(),
        source_package_hash: "".to_string(),
        schema_name: "genehmigung_v1".to_string(),
        schema_version: 1,
        required_field_paths: vec!["$.case_id".to_string()],
        satisfied_field_paths: vec![],
        missing_field_paths: vec![],
        constraints: vec![],
        required_order: vec![],
        dependency_edges: vec![],
        blockers: vec![],
        recovery_paths: vec![],
        validation: ContextValidation {
            valid: true,
            failure_labels: vec![],
            issues: vec![],
        },
        non_claims: vec!["no_mcp_integration".to_string()],
    };

    assert_eq!(
        ctx.validate_model_shape().unwrap_err(),
        "missing source_package_hash"
    );
}

#[test]
fn test_context_model_rejects_missing_required_field_paths() {
    use agy7rust::context::model::{ContextValidation, OperationalContext};

    let mut ctx = OperationalContext {
        context_id: "ctx_123".to_string(),
        source_package_hash: "hash_456".to_string(),
        schema_name: "genehmigung_v1".to_string(),
        schema_version: 1,
        required_field_paths: vec![],
        satisfied_field_paths: vec![],
        missing_field_paths: vec![],
        constraints: vec![],
        required_order: vec![],
        dependency_edges: vec![],
        blockers: vec![],
        recovery_paths: vec![],
        validation: ContextValidation {
            valid: true,
            failure_labels: vec![],
            issues: vec![],
        },
        non_claims: vec!["no_mcp_integration".to_string()],
    };

    assert_eq!(
        ctx.validate_model_shape().unwrap_err(),
        "missing required_field_paths"
    );

    ctx.required_field_paths = vec!["".to_string()];
    assert_eq!(
        ctx.validate_model_shape().unwrap_err(),
        "empty required_field_path"
    );
}

#[test]
fn test_context_model_rejects_empty_dependency_edge() {
    use agy7rust::context::model::{ContextDependencyEdge, ContextValidation, OperationalContext};

    let mut ctx = OperationalContext {
        context_id: "ctx_123".to_string(),
        source_package_hash: "hash_456".to_string(),
        schema_name: "genehmigung_v1".to_string(),
        schema_version: 1,
        required_field_paths: vec!["$.case_id".to_string()],
        satisfied_field_paths: vec![],
        missing_field_paths: vec![],
        constraints: vec![],
        required_order: vec![],
        dependency_edges: vec![ContextDependencyEdge {
            source: "".to_string(),
            target: "b".to_string(),
        }],
        blockers: vec![],
        recovery_paths: vec![],
        validation: ContextValidation {
            valid: true,
            failure_labels: vec![],
            issues: vec![],
        },
        non_claims: vec!["no_mcp_integration".to_string()],
    };

    assert_eq!(
        ctx.validate_model_shape().unwrap_err(),
        "empty dependency edge"
    );

    ctx.dependency_edges = vec![];
    ctx.blockers = vec![ContextDependencyEdge {
        source: "a".to_string(),
        target: "".to_string(),
    }];
    assert_eq!(
        ctx.validate_model_shape().unwrap_err(),
        "empty blocker edge"
    );

    ctx.blockers = vec![];
    ctx.recovery_paths = vec![ContextDependencyEdge {
        source: "".to_string(),
        target: "".to_string(),
    }];
    assert_eq!(
        ctx.validate_model_shape().unwrap_err(),
        "empty recovery edge"
    );
}

#[test]
fn test_context_model_serialization_has_no_raw_payload_fields() {
    use agy7rust::context::model::{ContextValidation, OperationalContext};

    let ctx = OperationalContext {
        context_id: "ctx_123".to_string(),
        source_package_hash: "hash_456".to_string(),
        schema_name: "genehmigung_v1".to_string(),
        schema_version: 1,
        required_field_paths: vec!["$.case_id".to_string()],
        satisfied_field_paths: vec![],
        missing_field_paths: vec![],
        constraints: vec![],
        required_order: vec![],
        dependency_edges: vec![],
        blockers: vec![],
        recovery_paths: vec![],
        validation: ContextValidation {
            valid: true,
            failure_labels: vec![],
            issues: vec![],
        },
        non_claims: vec!["no_mcp_integration".to_string()],
    };

    let serialized = serde_json::to_string(&ctx).expect("Serialization failed");

    assert!(!serialized.contains("applicant"));
    assert!(!serialized.contains("decision_recommendation"));
    assert!(!serialized.contains("extraction.notes"));
    assert!(!serialized.contains("source_pdf_contents"));
    assert!(!serialized.contains("\"payload\":"));
}

#[test]
fn test_context_build_from_current_package_and_schema_succeeds() {
    use agy7rust::codec::package::build_package_from_value;
    use agy7rust::context::build_context;

    let valid_input = serde_json::json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = serde_json::json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id",
            "$.document_type",
            "$.authority",
            "$.procedure_type",
            "$.applicant",
            "$.extraction.fields.parcel_id",
            "$.extraction.fields.location",
            "$.extraction.fields.project_type",
            "$.extraction.fields.decision_recommendation",
            "$.metadata.source_pdf_sha256"
        ],
        "non_claims": []
    });

    let package = build_package_from_value(&valid_input).unwrap();
    let ctx = build_context(&package, &valid_schema);
    assert!(ctx.is_ok());
    let context_val = ctx.unwrap();
    assert_eq!(context_val.schema_name, "genehmigung_v1");
    assert!(context_val.validation.valid);
}

#[test]
fn test_context_build_output_validates_model_shape() {
    use agy7rust::codec::package::build_package_from_value;
    use agy7rust::context::build_context;

    let valid_input = serde_json::json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = serde_json::json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id",
            "$.document_type",
            "$.authority",
            "$.procedure_type",
            "$.applicant",
            "$.extraction.fields.parcel_id",
            "$.extraction.fields.location",
            "$.extraction.fields.project_type",
            "$.extraction.fields.decision_recommendation",
            "$.metadata.source_pdf_sha256"
        ],
        "non_claims": []
    });

    let package = build_package_from_value(&valid_input).unwrap();
    let context_val = build_context(&package, &valid_schema).unwrap();
    assert!(context_val.validate_model_shape().is_ok());
}

#[test]
fn test_context_build_repeated_output_is_byte_identical() {
    use agy7rust::codec::package::build_package_from_value;
    use agy7rust::context::build_context;

    let valid_input = serde_json::json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = serde_json::json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id",
            "$.document_type",
            "$.authority",
            "$.procedure_type",
            "$.applicant",
            "$.extraction.fields.parcel_id",
            "$.extraction.fields.location",
            "$.extraction.fields.project_type",
            "$.extraction.fields.decision_recommendation",
            "$.metadata.source_pdf_sha256"
        ],
        "non_claims": []
    });

    let package = build_package_from_value(&valid_input).unwrap();

    let ctx1 = build_context(&package, &valid_schema).unwrap();
    let ctx2 = build_context(&package, &valid_schema).unwrap();

    let s1 = serde_json::to_string_pretty(&ctx1).unwrap();
    let s2 = serde_json::to_string_pretty(&ctx2).unwrap();

    assert_eq!(s1, s2);
}

#[test]
fn test_context_build_missing_required_path_is_reported() {
    use agy7rust::codec::package::build_package_from_value;
    use agy7rust::context::build_context;

    let valid_input = serde_json::json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = serde_json::json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id",
            "$.extraction.fields.parcel_id"
        ],
        "non_claims": []
    });

    let package = build_package_from_value(&valid_input).unwrap();
    let context_val = build_context(&package, &valid_schema).unwrap();

    assert!(!context_val.validation.valid);
    assert!(context_val
        .missing_field_paths
        .contains(&"$.extraction.fields.parcel_id".to_string()));
    assert!(context_val
        .validation
        .failure_labels
        .contains(&"MISSING_REQUIRED_FIELD".to_string()));
    assert!(context_val.validation.issues[0].contains("$.extraction.fields.parcel_id"));
}

#[test]
fn test_context_build_context_id_is_deterministic() {
    use agy7rust::codec::package::build_package_from_value;
    use agy7rust::context::build_context;

    let valid_input = serde_json::json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = serde_json::json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id"
        ],
        "non_claims": []
    });

    let package = build_package_from_value(&valid_input).unwrap();
    let ctx1 = build_context(&package, &valid_schema).unwrap();
    let ctx2 = build_context(&package, &valid_schema).unwrap();

    assert_eq!(ctx1.context_id, ctx2.context_id);
    assert!(!ctx1.context_id.is_empty());
}

#[test]
fn test_context_build_json_does_not_leak_raw_payload_values() {
    use agy7rust::codec::package::build_package_from_value;
    use agy7rust::context::build_context;

    let valid_input = serde_json::json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = serde_json::json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id",
            "$.applicant"
        ],
        "non_claims": []
    });

    let package = build_package_from_value(&valid_input).unwrap();
    let context_val = build_context(&package, &valid_schema).unwrap();

    let serialized = serde_json::to_string(&context_val).unwrap();

    assert!(!serialized.contains("Nordwind Energie GmbH"));
    assert!(!serialized.contains("Zustimmung mit Nebenbestimmungen zur Schallemission"));
    assert!(!serialized.contains("Static test case for planning approval"));
    assert!(!serialized.contains("source_pdf_contents"));
    assert!(!serialized.contains("\"payload\":"));
}

#[test]
fn test_context_build_command_exists() {
    use agy7rust::commands::Cli;
    use clap::CommandFactory;

    let cmd = Cli::command();
    let subcommands: Vec<&str> = cmd.get_subcommands().map(|c| c.get_name()).collect();
    assert!(subcommands.contains(&"context-build"));
}

#[test]
fn test_context_render_command_exists() {
    use agy7rust::commands::Cli;
    use clap::CommandFactory;

    let cmd = Cli::command();
    let subcommands: Vec<&str> = cmd.get_subcommands().map(|c| c.get_name()).collect();
    assert!(subcommands.contains(&"context-render"));
}

#[test]
fn test_context_render_deterministic() {
    use agy7rust::codec::package::build_package_from_value;
    use agy7rust::context::{build_context, render_context};

    let valid_input = serde_json::json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = serde_json::json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id",
            "$.document_type"
        ],
        "non_claims": []
    });

    let package = build_package_from_value(&valid_input).unwrap();
    let context_val = build_context(&package, &valid_schema).unwrap();

    let r1 = render_context(&context_val);
    let r2 = render_context(&context_val);

    assert_eq!(r1, r2);
    assert!(r1.ends_with('\n'));
}

#[test]
fn test_context_render_leak_free() {
    use agy7rust::codec::package::build_package_from_value;
    use agy7rust::context::{build_context, render_context};

    let valid_input = serde_json::json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = serde_json::json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id",
            "$.applicant"
        ],
        "non_claims": []
    });

    let package = build_package_from_value(&valid_input).unwrap();
    let context_val = build_context(&package, &valid_schema).unwrap();

    let rendered = render_context(&context_val);

    assert!(!rendered.contains("Nordwind Energie GmbH"));
    assert!(!rendered.contains("Zustimmung mit Nebenbestimmungen zur Schallemission"));
    assert!(!rendered.contains("Static test case for planning approval"));
    assert!(!rendered.contains("source_pdf_contents"));
}

#[test]
fn test_context_validate_command_exists() {
    use agy7rust::commands::Cli;
    use clap::CommandFactory;

    let cmd = Cli::command();
    let subcommands: Vec<&str> = cmd.get_subcommands().map(|c| c.get_name()).collect();
    assert!(subcommands.contains(&"context-validate"));
}

#[test]
fn test_context_validate_deterministic() {
    use agy7rust::codec::package::build_package_from_value;
    use agy7rust::context::{build_context, validate_context};

    let valid_input = serde_json::json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = serde_json::json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id"
        ],
        "non_claims": []
    });

    let package = build_package_from_value(&valid_input).unwrap();
    let context_val = build_context(&package, &valid_schema).unwrap();

    let r1 = validate_context(&context_val);
    let r2 = validate_context(&context_val);

    assert_eq!(r1, r2);
    assert!(r1.is_ok());
}

#[test]
fn test_context_validate_leak_free() {
    use agy7rust::codec::package::build_package_from_value;
    use agy7rust::context::{build_context, validate_context};

    let valid_input = serde_json::json!({
        "case_id": "SPARK-2026-0042",
        "document_type": "BImSchG-Genehmigungsantrag",
        "authority": "Staatliches Amt fuer Umwelt und Arbeitsschutz",
        "procedure_type": "Vereinfachtes Verfahren",
        "applicant": "Nordwind Energie GmbH",
        "extraction": {
            "fields": {
                "parcel_id": "DE-NI-004-9872",
                "location": "Gemarkung Neustadt, Flur 12, Flueck 5",
                "project_type": "Errichtung Windkraftanlage WKA-03",
                "decision_recommendation": "Zustimmung mit Nebenbestimmungen zur Schallemission"
            },
            "confidence": 0.945,
            "notes": "Static test case for planning approval acceleration pipeline."
        },
        "metadata": {
            "source_pdf_sha256": "8f395d98a72b0c3cf0d262e3b8a3e03d428e253f1bdf4cfa489d2b5801d3b46fc"
        }
    });

    let valid_schema = serde_json::json!({
        "schema": "SPARK-V7-SCHEMA",
        "version": 1,
        "name": "genehmigung_v1",
        "fixture_type": "synthetic_spark_style",
        "required_field_paths": [
            "$.case_id"
        ],
        "non_claims": []
    });

    let package = build_package_from_value(&valid_input).unwrap();
    let context_val = build_context(&package, &valid_schema).unwrap();

    let res = validate_context(&context_val);
    assert!(res.is_ok());
}

#[test]
fn test_context_validate_invalid_shape_fails() {
    use agy7rust::context::model::{ContextValidation, OperationalContext};
    use agy7rust::context::validate_context;

    let ctx = OperationalContext {
        context_id: "".to_string(),
        source_package_hash: "hash_456".to_string(),
        schema_name: "genehmigung_v1".to_string(),
        schema_version: 1,
        required_field_paths: vec!["$.case_id".to_string()],
        satisfied_field_paths: vec![],
        missing_field_paths: vec![],
        constraints: vec![],
        required_order: vec![],
        dependency_edges: vec![],
        blockers: vec![],
        recovery_paths: vec![],
        validation: ContextValidation {
            valid: true,
            failure_labels: vec![],
            issues: vec![],
        },
        non_claims: vec!["no_mcp_integration".to_string()],
    };

    let res = validate_context(&ctx);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("validate_model_shape failed"));
}

#[test]
fn test_sparkctl_doctor_execution() {
    use std::process::Command;
    let output = Command::new("cargo")
        .args(&["run", "--bin", "sparkctl", "--", "doctor"])
        .output()
        .expect("failed to execute cargo run");

    assert!(output.status.success());
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    assert!(stdout_str.contains("=== sparkctl doctor report ==="));
    assert!(stdout_str.contains("doctor result: PASS"));
}

#[test]
fn test_sparkctl_rust_validate_execution() {
    use std::process::Command;
    let output = Command::new("cargo")
        .env("SPARKCTL_IN_TEST", "1")
        .args(&["run", "--bin", "sparkctl", "--", "rust-validate"])
        .output()
        .expect("failed to execute cargo run");

    assert!(output.status.success());
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    assert!(stdout_str.contains("=== sparkctl rust-validate ==="));
    assert!(stdout_str.contains("rust-validate result: PASS"));
}

#[test]
fn test_sparkctl_context_all_execution() {
    use std::process::Command;
    let output = Command::new("cargo")
        .args(&["run", "--bin", "sparkctl", "--", "context-all"])
        .output()
        .expect("failed to execute cargo run");

    assert!(output.status.success());
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    assert!(stdout_str.contains("=== sparkctl context-all ==="));
    assert!(stdout_str.contains("context-all result: PASS"));
}

#[test]
fn test_sparkctl_spark_demo_execution() {
    use std::process::Command;
    let output = Command::new("cargo")
        .args(&["run", "--bin", "sparkctl", "--", "spark-demo"])
        .output()
        .expect("failed to execute cargo run");

    assert!(output.status.success());
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    assert!(stdout_str.contains("=== sparkctl spark-demo ==="));
    assert!(stdout_str.contains("spark-demo result: PASS"));
}
