from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction

from src.validation.contract_validator import Layer, ValidationResult


@dataclass(frozen=True, slots=True)
class LayerScore:
    layer: Layer
    passed_contracts: tuple[str, ...]
    failed_contracts: tuple[str, ...]
    failure_labels: tuple[str, ...]
    score: float


@dataclass(frozen=True, slots=True)
class AdmissibilityScore:
    structural_score: float
    relational_score: float
    operational_score: float
    governance_score: float
    overall_admissibility_score: float
    expected_admissible: bool
    observed_admissible: bool
    passed_contracts: tuple[str, ...]
    failed_contracts: tuple[str, ...]
    failure_labels: tuple[str, ...]
    layer_scores: tuple[LayerScore, ...]


class AdmissibilityScorer:
    _LAYER_ORDER: tuple[Layer, ...] = (
        Layer.STRUCTURAL,
        Layer.RELATIONAL,
        Layer.OPERATIONAL,
        Layer.GOVERNANCE,
    )

    def score(self, results: list[ValidationResult], expected_admissible: bool | None = None) -> AdmissibilityScore:
        observed_admissible = all(result.passed for result in results)
        effective_expected = observed_admissible if expected_admissible is None else expected_admissible

        passed_contracts = tuple(sorted(result.contract_id for result in results if result.passed))
        failed_contracts = tuple(sorted(result.contract_id for result in results if not result.passed))
        failure_labels = tuple(sorted({result.failure_label for result in results if result.failure_label is not None}))

        layer_scores: list[LayerScore] = []
        score_by_layer: dict[Layer, float] = {}
        score_fraction_by_layer: dict[Layer, Fraction] = {}

        for layer in self._LAYER_ORDER:
            layer_results = [result for result in results if result.layer == layer]
            passed_in_layer = tuple(sorted(result.contract_id for result in layer_results if result.passed))
            failed_in_layer = tuple(sorted(result.contract_id for result in layer_results if not result.passed))
            labels_in_layer = tuple(sorted({result.failure_label for result in layer_results if result.failure_label is not None}))
            total_contracts = len(layer_results)
            layer_score_fraction = Fraction(1, 1) if total_contracts == 0 else Fraction(len(passed_in_layer), total_contracts)
            layer_score = float(layer_score_fraction)
            score_by_layer[layer] = layer_score
            score_fraction_by_layer[layer] = layer_score_fraction
            layer_scores.append(
                LayerScore(
                    layer=layer,
                    passed_contracts=passed_in_layer,
                    failed_contracts=failed_in_layer,
                    failure_labels=labels_in_layer,
                    score=layer_score,
                )
            )

        overall_score_fraction = sum(score_fraction_by_layer[layer] for layer in self._LAYER_ORDER) / len(self._LAYER_ORDER)
        overall_admissibility_score = float(overall_score_fraction)

        return AdmissibilityScore(
            structural_score=score_by_layer[Layer.STRUCTURAL],
            relational_score=score_by_layer[Layer.RELATIONAL],
            operational_score=score_by_layer[Layer.OPERATIONAL],
            governance_score=score_by_layer[Layer.GOVERNANCE],
            overall_admissibility_score=overall_admissibility_score,
            expected_admissible=effective_expected,
            observed_admissible=observed_admissible,
            passed_contracts=passed_contracts,
            failed_contracts=failed_contracts,
            failure_labels=failure_labels,
            layer_scores=tuple(layer_scores),
        )

    def to_dict(self, score: AdmissibilityScore) -> dict[str, object]:
        return {
            "structural_score": score.structural_score,
            "relational_score": score.relational_score,
            "operational_score": score.operational_score,
            "governance_score": score.governance_score,
            "overall_admissibility_score": score.overall_admissibility_score,
            "expected_admissible": score.expected_admissible,
            "observed_admissible": score.observed_admissible,
            "passed_contracts": list(score.passed_contracts),
            "failed_contracts": list(score.failed_contracts),
            "failure_labels": list(score.failure_labels),
            "layer_scores": [
                {
                    "layer": layer_score.layer.value,
                    "passed_contracts": list(layer_score.passed_contracts),
                    "failed_contracts": list(layer_score.failed_contracts),
                    "failure_labels": list(layer_score.failure_labels),
                    "score": layer_score.score,
                }
                for layer_score in score.layer_scores
            ],
        }
