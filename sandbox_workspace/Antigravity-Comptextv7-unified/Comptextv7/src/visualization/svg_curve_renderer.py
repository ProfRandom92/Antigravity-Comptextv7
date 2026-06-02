from __future__ import annotations

from dataclasses import dataclass
from html import escape


@dataclass(frozen=True, slots=True)
class _PointLayout:
    fixture_id: str
    score: float
    x: float
    y: float
    failure_labels: tuple[str, ...]


class SVGCurveRenderer:
    WIDTH = 1000
    HEIGHT = 520
    MARGIN_LEFT = 90
    MARGIN_RIGHT = 40
    MARGIN_TOP = 70
    MARGIN_BOTTOM = 140

    TITLE = "Layered Admissibility Degradation Curve"
    X_LABEL = "Fixture progression"
    Y_LABEL = "overall_admissibility_score"

    X_TICKS: tuple[tuple[str, str], ...] = (
        ("coding_workflow_pr_review_v1", "positive"),
        ("coding_workflow_pr_review_mild_v1", "mild"),
        ("coding_workflow_pr_review_moderate_v1", "moderate"),
        ("coding_workflow_pr_review_degraded_v1", "severe"),
    )

    LEGEND_ITEMS: tuple[str, ...] = ("structural", "relational", "operational", "governance")

    FAILURE_ANNOTATION_ORDER: tuple[str, ...] = (
        "RECOVERY_PATH_INVALID",
        "CAUSAL_DEPENDENCY_LOSS",
        "POLICY_ORDER_BROKEN",
        "INVARIANT_VIOLATION",
    )

    def _fmt(self, value: float) -> str:
        return f"{value:.3f}"

    def _layout_points(self, curve_json: dict) -> tuple[_PointLayout, ...]:
        points_by_fixture = {point["fixture_id"]: point for point in curve_json["points"]}
        plot_width = self.WIDTH - self.MARGIN_LEFT - self.MARGIN_RIGHT
        plot_height = self.HEIGHT - self.MARGIN_TOP - self.MARGIN_BOTTOM

        layouts: list[_PointLayout] = []
        for index, (fixture_id, _) in enumerate(self.X_TICKS):
            point = points_by_fixture[fixture_id]
            score = float(point["overall_admissibility_score"])
            x = self.MARGIN_LEFT + (plot_width * index / (len(self.X_TICKS) - 1))
            y = self.MARGIN_TOP + ((1.0 - score) * plot_height)
            layouts.append(
                _PointLayout(
                    fixture_id=fixture_id,
                    score=score,
                    x=x,
                    y=y,
                    failure_labels=tuple(sorted(point["failure_labels"])),
                )
            )
        return tuple(layouts)

    def render(self, curve_json: dict) -> str:
        layouts = self._layout_points(curve_json)
        plot_bottom = self.HEIGHT - self.MARGIN_BOTTOM
        plot_right = self.WIDTH - self.MARGIN_RIGHT

        polyline_points = " ".join(f"{self._fmt(p.x)},{self._fmt(p.y)}" for p in layouts)
        elements: list[str] = [
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{self.WIDTH}" height="{self.HEIGHT}" viewBox="0 0 {self.WIDTH} {self.HEIGHT}">',
            '  <rect x="0" y="0" width="1000" height="520" fill="#ffffff"/>',
            f'  <text x="{self.WIDTH/2:.1f}" y="36" text-anchor="middle" font-size="22" font-family="monospace" fill="#111111">{self.TITLE}</text>',
            f'  <line x1="{self.MARGIN_LEFT}" y1="{plot_bottom}" x2="{plot_right}" y2="{plot_bottom}" stroke="#222222" stroke-width="1"/>',
            f'  <line x1="{self.MARGIN_LEFT}" y1="{self.MARGIN_TOP}" x2="{self.MARGIN_LEFT}" y2="{plot_bottom}" stroke="#222222" stroke-width="1"/>',
        ]

        for tick_score in (0.0, 0.5, 1.0):
            y = self.MARGIN_TOP + ((1.0 - tick_score) * (self.HEIGHT - self.MARGIN_TOP - self.MARGIN_BOTTOM))
            elements.append(
                f'  <line x1="{self.MARGIN_LEFT}" y1="{self._fmt(y)}" x2="{plot_right}" y2="{self._fmt(y)}" stroke="#e0e0e0" stroke-width="1"/>'
            )
            elements.append(
                f'  <text x="{self.MARGIN_LEFT-12}" y="{self._fmt(y+4)}" text-anchor="end" font-size="12" font-family="monospace" fill="#333333">{self._fmt(tick_score)}</text>'
            )

        for point, (_, stage_name) in zip(layouts, self.X_TICKS):
            elements.append(
                f'  <text x="{self._fmt(point.x)}" y="{plot_bottom+22}" text-anchor="middle" font-size="12" font-family="monospace" fill="#222222">{stage_name}</text>'
            )

        elements.extend(
            [
                f'  <polyline points="{polyline_points}" fill="none" stroke="#0055aa" stroke-width="3"/>',
                f'  <text x="{self.WIDTH/2:.1f}" y="{self.HEIGHT-20}" text-anchor="middle" font-size="13" font-family="monospace" fill="#111111">{self.X_LABEL}</text>',
                f'  <text x="20" y="{self.HEIGHT/2:.1f}" transform="rotate(-90 20 {self.HEIGHT/2:.1f})" text-anchor="middle" font-size="13" font-family="monospace" fill="#111111">{self.Y_LABEL}</text>',
            ]
        )

        for point in layouts:
            elements.append(
                f'  <circle cx="{self._fmt(point.x)}" cy="{self._fmt(point.y)}" r="5" fill="#0055aa"/>'
            )
            elements.append(
                f'  <text x="{self._fmt(point.x)}" y="{self._fmt(point.y-12)}" text-anchor="middle" font-size="11" font-family="monospace" fill="#111111">{escape(point.fixture_id)} | {self._fmt(point.score)}</text>'
            )

        y_base = plot_bottom + 44
        for point in layouts[1:]:
            ordered_labels = [label for label in self.FAILURE_ANNOTATION_ORDER if label in point.failure_labels]
            if ordered_labels:
                elements.append(
                    f'  <text x="{self._fmt(point.x)}" y="{y_base}" text-anchor="middle" font-size="10" font-family="monospace" fill="#aa2200">{", ".join(ordered_labels)}</text>'
                )

        legend_x = 700
        legend_y = 84
        elements.append(f'  <rect x="{legend_x}" y="{legend_y}" width="250" height="104" fill="#f8f8f8" stroke="#cccccc"/>')
        elements.append(f'  <text x="{legend_x+12}" y="{legend_y+18}" font-size="12" font-family="monospace" fill="#111111">Legend (component scores)</text>')
        for idx, item in enumerate(self.LEGEND_ITEMS):
            elements.append(
                f'  <text x="{legend_x+16}" y="{legend_y+36 + idx*16}" font-size="11" font-family="monospace" fill="#333333">- {item}</text>'
            )

        elements.append("</svg>")
        return "\n".join(elements) + "\n"
