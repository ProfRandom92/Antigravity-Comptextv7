from __future__ import annotations

import json
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any

from src.validation.admissibility_scorer import AdmissibilityScorer
from src.validation.contract_validator import ContractValidator


MANIFEST_PATH = Path("fixtures/manifest.json")
LAYERED_CURVE_FAMILY = "coding_workflow_pr_review"
LAYERED_CURVE_LEVELS = ("baseline", "mild", "moderate", "severe")


@dataclass(frozen=True, slots=True)
class FixtureScorePoint:
    fixture_id: str
    fixture_version: str
    fixture_path: str
    expected_admissible: bool
    observed_admissible: bool
    structural_score: float
    relational_score: float
    operational_score: float
    governance_score: float
    overall_admissibility_score: float
    passed_contracts: tuple[str, ...]
    failed_contracts: tuple[str, ...]
    failure_labels: tuple[str, ...]


@dataclass(frozen=True, slots=True)
class DegradationCurve:
    curve_id: str
    version: str
    generated_by: str
    points: tuple[FixtureScorePoint, ...]


class DegradationCurveGenerator:
    VERSION = "1.0"

    def _load_json(self, path: Path) -> dict[str, Any]:
        if not path.exists():
            raise FileNotFoundError(f"missing required fixture file: {path}")
        return json.loads(path.read_text(encoding="utf-8"))

    def _fixture_version(self, fixture_path: Path, expected_admissibility: dict[str, Any]) -> str:
        if "fixture_version" not in expected_admissibility:
            raise ValueError(f"missing fixture_version in {fixture_path / 'expected/admissibility.json'}")
        return str(expected_admissibility["fixture_version"])

    def _validate_expected_failures(
        self,
        fixture_path: Path,
        expected_failures_payload: dict[str, Any],
        observed_failure_labels: tuple[str, ...],
    ) -> None:
        expected = set(expected_failures_payload.get("expected_failures", []))
        disallowed = set(expected_failures_payload.get("disallowed_failures", []))
        observed = set(observed_failure_labels)

        missing_expected = sorted(expected - observed)
        if missing_expected:
            raise ValueError(f"missing expected failure labels for {fixture_path}: {missing_expected}")

        emitted_disallowed = sorted(disallowed & observed)
        if emitted_disallowed:
            raise ValueError(f"emitted disallowed failure labels for {fixture_path}: {emitted_disallowed}")

    def evaluate_fixture(self, fixture_path: Path) -> FixtureScorePoint:
        original = {
            **self._load_json(fixture_path / "original/trace.json"),
            **self._load_json(fixture_path / "original/state.json"),
            "dependency_graph": self._load_json(fixture_path / "original/dependency_graph.json"),
        }
        reconstructed = {
            **self._load_json(fixture_path / "reconstructed/trace.json"),
            **self._load_json(fixture_path / "reconstructed/state.json"),
            "dependency_graph": self._load_json(fixture_path / "reconstructed/dependency_graph.json"),
        }
        contracts_dir = fixture_path / "original/contracts"
        contracts = [self._load_json(contract_path) for contract_path in sorted(contracts_dir.glob("*.json"))]
        if not contracts:
            raise FileNotFoundError(f"no contract files found in fixture: {contracts_dir}")

        expected_admissibility = self._load_json(fixture_path / "expected/admissibility.json")
        expected_admissible = bool(expected_admissibility["expected_admissible"])
        fixture_version = self._fixture_version(fixture_path, expected_admissibility)
        expected_failures = self._load_json(fixture_path / "expected/failures.json")

        results = ContractValidator().validate_contracts(original=original, reconstructed=reconstructed, contracts=contracts)
        score = AdmissibilityScorer().score(results, expected_admissible=expected_admissible)
        self._validate_expected_failures(fixture_path, expected_failures, score.failure_labels)

        return FixtureScorePoint(
            fixture_id=fixture_path.name,
            fixture_version=fixture_version,
            fixture_path=fixture_path.as_posix(),
            expected_admissible=score.expected_admissible,
            observed_admissible=score.observed_admissible,
            structural_score=score.structural_score,
            relational_score=score.relational_score,
            operational_score=score.operational_score,
            governance_score=score.governance_score,
            overall_admissibility_score=score.overall_admissibility_score,
            passed_contracts=tuple(sorted(score.passed_contracts)),
            failed_contracts=tuple(sorted(score.failed_contracts)),
            failure_labels=tuple(sorted(score.failure_labels)),
        )

    def _load_fixture_manifest(self, manifest_path: Path = MANIFEST_PATH) -> tuple[dict[str, Any], ...]:
        manifest = self._load_json(manifest_path)
        fixtures = manifest.get("fixtures")
        if not isinstance(fixtures, list):
            raise ValueError(f"invalid fixture manifest format: {manifest_path}")
        return tuple(fixtures)

    def fixtures_for_layered_admissibility_curve(self, manifest_path: Path = MANIFEST_PATH) -> tuple[Path, ...]:
        level_to_path: dict[str, Path] = {}

        for entry in self._load_fixture_manifest(manifest_path):
            if entry.get("family") != LAYERED_CURVE_FAMILY:
                continue
            level = entry.get("degradation_level")
            if level in LAYERED_CURVE_LEVELS:
                path_str = entry.get("path")
                if not path_str:
                    raise ValueError(f"missing path for fixture in manifest: {entry.get('fixture_id')}")
                level_to_path[str(level)] = Path(path_str)

        missing_levels = [level for level in LAYERED_CURVE_LEVELS if level not in level_to_path]
        if missing_levels:
            raise ValueError(f"missing layered admissibility fixtures for levels: {missing_levels}")

        return tuple(level_to_path[level] for level in LAYERED_CURVE_LEVELS)

    def generate(self, fixtures: list[Path] | tuple[Path, ...], curve_id: str) -> DegradationCurve:
        points = tuple(self.evaluate_fixture(path) for path in fixtures)
        return DegradationCurve(curve_id=curve_id, version=self.VERSION, generated_by=self.__class__.__name__, points=points)

    def to_dict(self, curve: DegradationCurve) -> dict[str, object]:
        return {
            "curve_id": curve.curve_id,
            "version": curve.version,
            "generated_by": curve.generated_by,
            "points": [
                {
                    **asdict(point),
                    "passed_contracts": list(point.passed_contracts),
                    "failed_contracts": list(point.failed_contracts),
                    "failure_labels": list(point.failure_labels),
                }
                for point in curve.points
            ],
        }

    def write_json(self, curve: DegradationCurve, output_path: Path) -> None:
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(json.dumps(self.to_dict(curve), indent=2, sort_keys=True) + "\n", encoding="utf-8")

    def write_markdown(self, curve: DegradationCurve, output_path: Path) -> None:
        output_path.parent.mkdir(parents=True, exist_ok=True)
        rows = []
        for point in curve.points:
            labels = ", ".join(point.failure_labels) if point.failure_labels else "none"
            rows.append(
                f"| {point.fixture_id} | {str(point.expected_admissible).lower()} | {str(point.observed_admissible).lower()} | "
                f"{point.structural_score:.3f} | {point.relational_score:.3f} | {point.operational_score:.3f} | "
                f"{point.governance_score:.3f} | {point.overall_admissibility_score:.3f} | {labels} |"
            )

        markdown = "\n".join(
            [
                "# Layered Admissibility Degradation Benchmark",
                "",
                "## Purpose",
                "",
                "Deterministically compare admissibility outcomes across fixture bundles using ContractValidator and AdmissibilityScorer.",
                "",
                "## Fixture results",
                "",
                "| fixture_id | expected_admissible | observed_admissible | structural_score | relational_score | operational_score | governance_score | overall_admissibility_score | failure_labels |",
                "| --- | --- | --- | --- | --- | --- | --- | --- | --- |",
                *rows,
                "",
                "## Interpretation",
                "",
                "The positive fixture remains fully admissible while the degraded fixture shows deterministic score loss and explicit failure labels.",
                "",
                "## Non-goals",
                "",
                "- no LLM judges",
                "- no embeddings",
                "- no fuzzy matching",
                "- no semantic equivalence",
                "",
                "## Future",
                "",
                "- add more fixture families",
                "- add progressive degradation levels",
                "- add SVG curve visualization later",
                "",
            ]
        )
        output_path.write_text(markdown, encoding="utf-8")
