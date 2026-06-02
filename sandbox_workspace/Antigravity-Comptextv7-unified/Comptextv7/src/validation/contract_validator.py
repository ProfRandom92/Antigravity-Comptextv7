from __future__ import annotations

from collections import deque
from dataclasses import dataclass, field
from enum import Enum
from typing import Any

from src.validation.dependency_graph_comparator import DependencyGraph, DependencyGraphComparator, RelationType


class Layer(str, Enum):
    STRUCTURAL = "structural"
    RELATIONAL = "relational"
    OPERATIONAL = "operational"
    GOVERNANCE = "governance"


class ContractType(str, Enum):
    ORDERING = "ordering"
    REACHABILITY = "reachability"
    CAUSALITY = "causality"
    INVARIANT = "invariant"


@dataclass(frozen=True, slots=True)
class ValidationResult:
    contract_id: str
    layer: Layer
    contract_type: ContractType
    passed: bool
    severity: str
    failure_label: str | None = None
    invariant_category: str | None = None
    deterministic_evidence: dict[str, Any] = field(default_factory=dict)


class ContractValidationError(Exception):
    pass


class ContractValidator:
    def validate_contracts(
        self,
        original: dict[str, Any],
        reconstructed: dict[str, Any],
        contracts: list[dict[str, Any]],
    ) -> list[ValidationResult]:
        return [self.validate_contract(original=original, reconstructed=reconstructed, contract=contract) for contract in contracts]

    def validate_contract(
        self,
        original: dict[str, Any],
        reconstructed: dict[str, Any],
        contract: dict[str, Any],
    ) -> ValidationResult:
        contract_id = str(contract.get("contract_id", ""))
        if not contract_id:
            raise ContractValidationError("contract missing required field: contract_id")

        layer = self._parse_layer(contract.get("layer"))
        contract_type = self._parse_contract_type(contract.get("type"))
        severity = str(contract.get("severity", "")).strip()
        if not severity:
            raise ContractValidationError(f"contract '{contract_id}' missing required field: severity")

        if "definition" not in contract or not isinstance(contract["definition"], dict):
            raise ContractValidationError(f"contract '{contract_id}' missing required object field: definition")
        definition = contract["definition"]

        if contract_type == ContractType.ORDERING:
            return self._validate_ordering(contract_id, layer, severity, definition, reconstructed)
        if contract_type == ContractType.REACHABILITY:
            return self._validate_reachability(contract_id, layer, severity, definition, original, reconstructed)
        if contract_type == ContractType.CAUSALITY:
            return self._validate_causality(contract_id, layer, severity, definition, original, reconstructed)
        if contract_type == ContractType.INVARIANT:
            return self._validate_invariant(contract_id, layer, severity, definition, original, reconstructed)

        raise ContractValidationError(f"unsupported contract type: {contract_type.value}")

    def _parse_layer(self, layer_value: Any) -> Layer:
        try:
            return Layer(str(layer_value))
        except ValueError as exc:
            raise ContractValidationError(f"invalid layer value: {layer_value}") from exc

    def _parse_contract_type(self, type_value: Any) -> ContractType:
        try:
            return ContractType(str(type_value))
        except ValueError as exc:
            raise ContractValidationError(f"unknown contract type: {type_value}") from exc

    def _extract_graph(self, payload: dict[str, Any], label: str) -> DependencyGraph:
        graph_payload = payload.get("dependency_graph")
        if not isinstance(graph_payload, dict):
            raise ContractValidationError(f"{label} missing required object field: dependency_graph")
        try:
            return DependencyGraph.from_dict(graph_payload)
        except Exception as exc:
            raise ContractValidationError(f"invalid dependency_graph in {label}: {exc}") from exc

    def _extract_observed_actions(self, reconstructed: dict[str, Any]) -> list[str]:
        events = reconstructed.get("events", reconstructed.get("trace"))
        if not isinstance(events, list):
            raise ContractValidationError("reconstructed payload missing events/trace list for ordering contract")

        observed: list[str] = []
        for event in events:
            if not isinstance(event, dict):
                continue
            action = event.get("action", event.get("tool"))
            if isinstance(action, str) and action:
                observed.append(action)
        return observed

    def _is_ordered_subsequence(self, required: list[str], observed: list[str]) -> bool:
        if not required:
            return True
        req_index = 0
        for action in observed:
            if action == required[req_index]:
                req_index += 1
                if req_index == len(required):
                    return True
        return False

    def _validate_ordering(self, contract_id: str, layer: Layer, severity: str, definition: dict[str, Any], reconstructed: dict[str, Any]) -> ValidationResult:
        required_sequence = definition.get("required_sequence")
        if not isinstance(required_sequence, list) or not all(isinstance(item, str) for item in required_sequence):
            raise ContractValidationError(f"contract '{contract_id}' requires definition.required_sequence as list[str]")

        observed_sequence = self._extract_observed_actions(reconstructed)
        passed = self._is_ordered_subsequence(required_sequence, observed_sequence)
        return ValidationResult(
            contract_id=contract_id,
            layer=layer,
            contract_type=ContractType.ORDERING,
            passed=passed,
            severity=severity,
            failure_label=None if passed else "POLICY_ORDER_BROKEN",
            invariant_category=None if passed else "ordering",
            deterministic_evidence={"required_sequence": required_sequence, "observed_sequence": observed_sequence},
        )

    def _comparison_summary(self, result: Any) -> tuple[dict[str, float], list[str]]:
        metrics = {
            "reachability_preservation": result.reachability_preservation,
            "dependency_integrity_score": result.dependency_integrity_score,
            "causal_preservation_score": result.causal_preservation_score,
            "temporal_order_violation_rate": result.temporal_order_violation_rate,
        }
        labels = [failure.label for failure in result.failures]
        return metrics, labels

    def _reachable_targets(self, graph: DependencyGraph, source: str, targets: list[str]) -> list[str]:
        adjacency: dict[str, list[str]] = {}
        for edge in graph.get_edges():
            adjacency.setdefault(edge.source, []).append(edge.target)
        for node in adjacency:
            adjacency[node].sort()

        visited: set[str] = {source}
        queue: deque[str] = deque([source])
        while queue:
            node = queue.popleft()
            for neighbor in adjacency.get(node, []):
                if neighbor not in visited:
                    visited.add(neighbor)
                    queue.append(neighbor)

        return sorted(target for target in targets if target in visited)

    def _validate_reachability(self, contract_id: str, layer: Layer, severity: str, definition: dict[str, Any], original: dict[str, Any], reconstructed: dict[str, Any]) -> ValidationResult:
        source = definition.get("from")
        targets = definition.get("to")
        min_paths = definition.get("min_paths")
        if not isinstance(source, str) or not source:
            raise ContractValidationError(f"contract '{contract_id}' requires definition.from as non-empty string")
        if not isinstance(targets, list) or not all(isinstance(target, str) and target for target in targets):
            raise ContractValidationError(f"contract '{contract_id}' requires definition.to as list[str]")
        if not isinstance(min_paths, int) or min_paths < 0:
            raise ContractValidationError(f"contract '{contract_id}' requires definition.min_paths as non-negative int")

        original_graph = self._extract_graph(original, "original")
        reconstructed_graph = self._extract_graph(reconstructed, "reconstructed")
        comparison = DependencyGraphComparator().compare(original_graph, reconstructed_graph)
        metrics, labels = self._comparison_summary(comparison)

        reachable_targets = self._reachable_targets(reconstructed_graph, source, targets)
        missing_targets = sorted(set(targets) - set(reachable_targets))
        passed = len(reachable_targets) >= min_paths

        return ValidationResult(
            contract_id=contract_id,
            layer=layer,
            contract_type=ContractType.REACHABILITY,
            passed=passed,
            severity=severity,
            failure_label=None if passed else "RECOVERY_PATH_INVALID",
            invariant_category=None if passed else "reachability",
            deterministic_evidence={
                "source": source,
                "targets": targets,
                "reachable_targets": reachable_targets,
                "missing_targets": missing_targets,
                "min_paths": min_paths,
                "comparator_metrics": metrics,
                "comparator_failure_labels": labels,
            },
        )

    def _validate_causality(self, contract_id: str, layer: Layer, severity: str, definition: dict[str, Any], original: dict[str, Any], reconstructed: dict[str, Any]) -> ValidationResult:
        required_edges = definition.get("required_causal_edges")
        if not isinstance(required_edges, list):
            raise ContractValidationError(f"contract '{contract_id}' requires definition.required_causal_edges as list")

        normalized_required: list[tuple[str, str]] = []
        for entry in required_edges:
            if not isinstance(entry, list) or len(entry) != 2 or not all(isinstance(part, str) and part for part in entry):
                raise ContractValidationError(f"contract '{contract_id}' has invalid causal edge entry: {entry}")
            normalized_required.append((entry[0], entry[1]))

        reconstructed_graph = self._extract_graph(reconstructed, "reconstructed")
        reconstructed_causal = {(e.source, e.target) for e in reconstructed_graph.get_edges() if e.relation == RelationType.CAUSAL}
        missing_causal = sorted([list(edge) for edge in normalized_required if edge not in reconstructed_causal])

        evidence: dict[str, Any] = {
            "required_causal_edges": [list(edge) for edge in normalized_required],
            "missing_causal_edges": missing_causal,
        }

        if "dependency_graph" in original:
            original_graph = self._extract_graph(original, "original")
            comparison = DependencyGraphComparator().compare(original_graph, reconstructed_graph)
            metrics, labels = self._comparison_summary(comparison)
            evidence["comparator_metrics"] = metrics
            evidence["comparator_failure_labels"] = labels

        passed = len(missing_causal) == 0
        return ValidationResult(
            contract_id=contract_id,
            layer=layer,
            contract_type=ContractType.CAUSALITY,
            passed=passed,
            severity=severity,
            failure_label=None if passed else "CAUSAL_DEPENDENCY_LOSS",
            invariant_category=None if passed else "causality",
            deterministic_evidence=evidence,
        )

    def _validate_invariant(self, contract_id: str, layer: Layer, severity: str, definition: dict[str, Any], original: dict[str, Any], reconstructed: dict[str, Any]) -> ValidationResult:
        rule = definition.get("rule")
        if rule != "no_orphan_dependencies":
            raise ContractValidationError(f"contract '{contract_id}' supports only invariant rule 'no_orphan_dependencies'")

        original_graph = self._extract_graph(original, "original")
        reconstructed_graph = self._extract_graph(reconstructed, "reconstructed")
        comparison = DependencyGraphComparator().compare(original_graph, reconstructed_graph)
        metrics, labels = self._comparison_summary(comparison)

        passed = "ORPHAN_DEPENDENCY" not in labels
        return ValidationResult(
            contract_id=contract_id,
            layer=layer,
            contract_type=ContractType.INVARIANT,
            passed=passed,
            severity=severity,
            failure_label=None if passed else "INVARIANT_VIOLATION",
            invariant_category=None if passed else "reachability",
            deterministic_evidence={"rule": rule, "comparator_metrics": metrics, "comparator_failure_labels": labels},
        )
