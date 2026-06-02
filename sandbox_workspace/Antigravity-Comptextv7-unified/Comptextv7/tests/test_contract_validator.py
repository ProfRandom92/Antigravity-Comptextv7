from __future__ import annotations

import pytest

from src.validation.contract_validator import ContractValidationError, ContractValidator
from src.validation.dependency_graph_comparator import DependencyEdge, DependencyGraph, DependencyNode, RelationType


def _graph_payload(nodes: list[str], edges: list[tuple[str, str, RelationType]]) -> dict[str, object]:
    graph = DependencyGraph()
    for node_id in nodes:
        graph.add_node(DependencyNode(node_id=node_id, label=node_id, metadata=None))
    for source, target, relation in edges:
        graph.add_edge(DependencyEdge(source=source, target=target, relation=relation, metadata=None))
    return graph.to_dict()


def test_ordering_contract_passes_when_required_sequence_is_subsequence() -> None:
    contract = {
        "contract_id": "pre_merge_review",
        "layer": "operational",
        "type": "ordering",
        "definition": {"required_sequence": ["generate_patch", "run_tests", "human_review", "merge"]},
        "severity": "CRITICAL",
    }
    reconstructed = {
        "events": [
            {"action": "setup"},
            {"tool": "generate_patch"},
            {"action": "run_tests"},
            {"tool": "human_review"},
            {"action": "merge"},
        ]
    }

    result = ContractValidator().validate_contract({}, reconstructed, contract)

    assert result.passed is True
    assert result.failure_label is None


def test_ordering_contract_fails_with_policy_order_broken_and_evidence() -> None:
    contract = {
        "contract_id": "pre_merge_review",
        "layer": "operational",
        "type": "ordering",
        "definition": {"required_sequence": ["generate_patch", "run_tests", "human_review", "merge"]},
        "severity": "CRITICAL",
    }
    reconstructed = {"trace": [{"action": "generate_patch"}, {"action": "human_review"}, {"action": "run_tests"}]}

    result = ContractValidator().validate_contract({}, reconstructed, contract)

    assert result.passed is False
    assert result.failure_label == "POLICY_ORDER_BROKEN"
    assert result.invariant_category == "ordering"
    assert result.deterministic_evidence["required_sequence"] == ["generate_patch", "run_tests", "human_review", "merge"]
    assert result.deterministic_evidence["observed_sequence"] == ["generate_patch", "human_review", "run_tests"]


def test_reachability_contract_passes_when_target_reachable() -> None:
    contract = {
        "contract_id": "recovery_path_available",
        "layer": "relational",
        "type": "reachability",
        "definition": {"from": "main_workflow_failure", "to": ["rollback", "escalate_to_human"], "min_paths": 1},
        "severity": "HIGH",
    }
    original = {
        "dependency_graph": _graph_payload(
            ["main_workflow_failure", "rollback", "escalate_to_human"],
            [("main_workflow_failure", "rollback", RelationType.RECOVERY)],
        )
    }
    reconstructed = {
        "dependency_graph": _graph_payload(
            ["main_workflow_failure", "rollback", "escalate_to_human"],
            [("main_workflow_failure", "rollback", RelationType.RECOVERY)],
        )
    }

    result = ContractValidator().validate_contract(original, reconstructed, contract)

    assert result.passed is True
    assert result.deterministic_evidence["reachable_targets"] == ["rollback"]


def test_reachability_contract_fails_with_recovery_path_invalid() -> None:
    contract = {
        "contract_id": "recovery_path_available",
        "layer": "relational",
        "type": "reachability",
        "definition": {"from": "main_workflow_failure", "to": ["rollback", "escalate_to_human"], "min_paths": 2},
        "severity": "HIGH",
    }
    original = {
        "dependency_graph": _graph_payload(
            ["main_workflow_failure", "rollback", "escalate_to_human"],
            [
                ("main_workflow_failure", "rollback", RelationType.RECOVERY),
                ("main_workflow_failure", "escalate_to_human", RelationType.RECOVERY),
            ],
        )
    }
    reconstructed = {
        "dependency_graph": _graph_payload(
            ["main_workflow_failure", "rollback", "escalate_to_human"],
            [("main_workflow_failure", "rollback", RelationType.RECOVERY)],
        )
    }

    result = ContractValidator().validate_contract(original, reconstructed, contract)

    assert result.passed is False
    assert result.failure_label == "RECOVERY_PATH_INVALID"
    assert result.invariant_category == "reachability"


def test_causality_contract_passes_when_required_causal_edge_exists() -> None:
    contract = {
        "contract_id": "causal_failure_blocks_deploy",
        "layer": "relational",
        "type": "causality",
        "definition": {"required_causal_edges": [["security_scan_failed", "deploy_blocked"]]},
        "severity": "HIGH",
    }
    reconstructed = {
        "dependency_graph": _graph_payload(
            ["security_scan_failed", "deploy_blocked"],
            [("security_scan_failed", "deploy_blocked", RelationType.CAUSAL)],
        )
    }

    result = ContractValidator().validate_contract({}, reconstructed, contract)

    assert result.passed is True
    assert result.deterministic_evidence["missing_causal_edges"] == []


def test_causality_contract_fails_with_causal_dependency_loss() -> None:
    contract = {
        "contract_id": "causal_failure_blocks_deploy",
        "layer": "relational",
        "type": "causality",
        "definition": {"required_causal_edges": [["security_scan_failed", "deploy_blocked"]]},
        "severity": "HIGH",
    }
    reconstructed = {
        "dependency_graph": _graph_payload(["security_scan_failed", "deploy_blocked"], []),
    }

    result = ContractValidator().validate_contract({}, reconstructed, contract)

    assert result.passed is False
    assert result.failure_label == "CAUSAL_DEPENDENCY_LOSS"


def test_invariant_no_orphan_dependencies_fails_when_reconstructed_graph_has_orphan() -> None:
    contract = {
        "contract_id": "no_orphan_dependencies",
        "layer": "relational",
        "type": "invariant",
        "definition": {"rule": "no_orphan_dependencies"},
        "severity": "HIGH",
    }
    original = {
        "dependency_graph": _graph_payload(
            ["A", "B", "C"],
            [("A", "B", RelationType.PREREQUISITE), ("C", "B", RelationType.DATA_FLOW)],
        )
    }
    reconstructed = {
        "dependency_graph": _graph_payload(["A", "B", "C"], [("A", "C", RelationType.PREREQUISITE)]),
    }

    result = ContractValidator().validate_contract(original, reconstructed, contract)

    assert result.passed is False
    assert result.failure_label == "INVARIANT_VIOLATION"


def test_malformed_contract_raises_contract_validation_error() -> None:
    malformed = {
        "contract_id": "bad",
        "layer": "relational",
        "type": "reachability",
        "definition": {"from": "a", "to": ["b"]},
        "severity": "HIGH",
    }

    with pytest.raises(ContractValidationError):
        ContractValidator().validate_contract({"dependency_graph": _graph_payload(["a", "b"], [])}, {"dependency_graph": _graph_payload(["a", "b"], [])}, malformed)


def test_unknown_contract_type_raises_contract_validation_error() -> None:
    contract = {
        "contract_id": "unknown",
        "layer": "relational",
        "type": "unknown_type",
        "definition": {},
        "severity": "HIGH",
    }

    with pytest.raises(ContractValidationError):
        ContractValidator().validate_contract({}, {}, contract)


def test_relational_contract_evidence_contains_comparator_metrics() -> None:
    contract = {
        "contract_id": "recovery_path_available",
        "layer": "relational",
        "type": "reachability",
        "definition": {"from": "main_workflow_failure", "to": ["rollback"], "min_paths": 1},
        "severity": "HIGH",
    }
    payload = _graph_payload(["main_workflow_failure", "rollback"], [("main_workflow_failure", "rollback", RelationType.RECOVERY)])

    result = ContractValidator().validate_contract({"dependency_graph": payload}, {"dependency_graph": payload}, contract)

    assert "comparator_metrics" in result.deterministic_evidence
    metrics = result.deterministic_evidence["comparator_metrics"]
    assert set(metrics.keys()) == {
        "reachability_preservation",
        "dependency_integrity_score",
        "causal_preservation_score",
        "temporal_order_violation_rate",
    }
