from __future__ import annotations

import json
from pathlib import Path

import pytest

from src.validation.degradation_curve_generator import DegradationCurveGenerator


POS_FIXTURE = Path("fixtures/coding_workflow_pr_review_v1")
MILD_FIXTURE = Path("fixtures/coding_workflow_pr_review_mild_v1")
MODERATE_FIXTURE = Path("fixtures/coding_workflow_pr_review_moderate_v1")
NEG_FIXTURE = Path("fixtures/coding_workflow_pr_review_degraded_v1")
ARTIFACT_PATH = Path("artifacts/layered_admissibility_results.json")
CURVE_ID = "coding_workflow_pr_review_curve_v1"


def test_evaluate_positive_fixture_scores_one() -> None:
    point = DegradationCurveGenerator().evaluate_fixture(POS_FIXTURE)
    assert point.fixture_version == "1.0.0"
    assert point.observed_admissible is True
    assert point.overall_admissibility_score == 1.0
    assert point.failed_contracts == ()
    assert point.failure_labels == ()


def test_evaluate_negative_fixture_detects_expected_failures() -> None:
    point = DegradationCurveGenerator().evaluate_fixture(NEG_FIXTURE)
    assert point.fixture_version == "1.0.0"
    assert point.observed_admissible is False
    assert point.overall_admissibility_score < 1.0
    assert {
        "POLICY_ORDER_BROKEN",
        "RECOVERY_PATH_INVALID",
        "CAUSAL_DEPENDENCY_LOSS",
        "INVARIANT_VIOLATION",
    }.issubset(set(point.failure_labels))


def test_generate_curve_is_deterministic() -> None:
    generator = DegradationCurveGenerator()
    fixtures = generator.fixtures_for_layered_admissibility_curve()
    assert generator.to_dict(generator.generate(fixtures, curve_id=CURVE_ID)) == generator.to_dict(
        generator.generate(fixtures, curve_id=CURVE_ID)
    )




def test_layered_curve_fixtures_are_loaded_from_manifest_order() -> None:
    fixtures = DegradationCurveGenerator().fixtures_for_layered_admissibility_curve()
    assert [fixture.as_posix() for fixture in fixtures] == [
        POS_FIXTURE.as_posix(),
        MILD_FIXTURE.as_posix(),
        MODERATE_FIXTURE.as_posix(),
        NEG_FIXTURE.as_posix(),
    ]

def test_to_dict_is_json_compatible_and_sorted() -> None:
    generator = DegradationCurveGenerator()
    curve = generator.generate(generator.fixtures_for_layered_admissibility_curve(), curve_id=CURVE_ID)
    curve_dict = generator.to_dict(curve)
    json.dumps(curve_dict, sort_keys=True)
    assert [point["fixture_path"] for point in curve_dict["points"]] == [
        POS_FIXTURE.as_posix(),
        MILD_FIXTURE.as_posix(),
        MODERATE_FIXTURE.as_posix(),
        NEG_FIXTURE.as_posix(),
    ]


def test_write_json_matches_committed_artifact(tmp_path: Path) -> None:
    generator = DegradationCurveGenerator()
    curve = generator.generate(generator.fixtures_for_layered_admissibility_curve(), curve_id=CURVE_ID)
    generated_path = tmp_path / "layered_admissibility_results.json"
    generator.write_json(curve, generated_path)

    generated = json.loads(generated_path.read_text(encoding="utf-8"))
    committed = json.loads(ARTIFACT_PATH.read_text(encoding="utf-8"))
    assert generated == committed


def test_write_markdown_contains_fixture_rows(tmp_path: Path) -> None:
    generator = DegradationCurveGenerator()
    curve = generator.generate(generator.fixtures_for_layered_admissibility_curve(), curve_id=CURVE_ID)
    markdown_path = tmp_path / "layered_admissibility.md"
    generator.write_markdown(curve, markdown_path)

    content = markdown_path.read_text(encoding="utf-8")
    assert "coding_workflow_pr_review_v1" in content
    assert "coding_workflow_pr_review_mild_v1" in content
    assert "coding_workflow_pr_review_moderate_v1" in content
    assert "coding_workflow_pr_review_degraded_v1" in content
    assert "POLICY_ORDER_BROKEN" in content
    assert "RECOVERY_PATH_INVALID" in content


def test_missing_fixture_file_raises_clear_error(tmp_path: Path) -> None:
    incomplete = tmp_path / "fixture"
    incomplete.mkdir(parents=True)
    with pytest.raises(FileNotFoundError, match="missing required fixture file"):
        DegradationCurveGenerator().evaluate_fixture(incomplete)


def test_missing_expected_failure_label_raises_clear_error() -> None:
    generator = DegradationCurveGenerator()

    with pytest.raises(ValueError, match="missing expected failure labels"):
        generator._validate_expected_failures(
            Path("fixtures/example"),
            {"expected_failures": ["MISSING_EXPECTED_FAILURE"], "disallowed_failures": []},
            ("OBSERVED_FAILURE",),
        )


def test_disallowed_failure_label_raises_clear_error() -> None:
    generator = DegradationCurveGenerator()

    with pytest.raises(ValueError, match="emitted disallowed failure labels"):
        generator._validate_expected_failures(
            Path("fixtures/example"),
            {"expected_failures": [], "disallowed_failures": ["DISALLOWED_FAILURE"]},
            ("DISALLOWED_FAILURE",),
        )


def test_progressive_curve_scores_are_monotonic_or_non_increasing() -> None:
    generator = DegradationCurveGenerator()
    curve = generator.generate(generator.fixtures_for_layered_admissibility_curve(), curve_id=CURVE_ID)
    points = {point.fixture_id: point for point in curve.points}

    assert points["coding_workflow_pr_review_v1"].overall_admissibility_score == 1.0
    assert points["coding_workflow_pr_review_mild_v1"].overall_admissibility_score < points["coding_workflow_pr_review_v1"].overall_admissibility_score
    assert points["coding_workflow_pr_review_moderate_v1"].overall_admissibility_score <= points["coding_workflow_pr_review_mild_v1"].overall_admissibility_score
    assert points["coding_workflow_pr_review_degraded_v1"].overall_admissibility_score <= points["coding_workflow_pr_review_moderate_v1"].overall_admissibility_score


def test_mild_fixture_only_expected_recovery_failure() -> None:
    point = DegradationCurveGenerator().evaluate_fixture(MILD_FIXTURE)
    labels = set(point.failure_labels)

    assert point.observed_admissible is False
    assert "RECOVERY_PATH_INVALID" in labels
    assert "POLICY_ORDER_BROKEN" not in labels
    assert "CAUSAL_DEPENDENCY_LOSS" not in labels
    assert "INVARIANT_VIOLATION" not in labels


def test_moderate_fixture_expected_recovery_and_causality_failures() -> None:
    point = DegradationCurveGenerator().evaluate_fixture(MODERATE_FIXTURE)
    labels = set(point.failure_labels)

    assert "RECOVERY_PATH_INVALID" in labels
    assert "CAUSAL_DEPENDENCY_LOSS" in labels
    assert "POLICY_ORDER_BROKEN" not in labels
    assert "INVARIANT_VIOLATION" not in labels
