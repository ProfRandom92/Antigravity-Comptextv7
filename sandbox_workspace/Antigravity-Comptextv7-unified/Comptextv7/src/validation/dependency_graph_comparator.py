from __future__ import annotations

from collections import deque
from dataclasses import dataclass
from enum import Enum


class RelationType(Enum):
    PREREQUISITE = "PREREQUISITE"
    CAUSAL = "CAUSAL"
    TEMPORAL = "TEMPORAL"
    DATA_FLOW = "DATA_FLOW"
    BLOCKER = "BLOCKER"
    RECOVERY = "RECOVERY"


@dataclass(frozen=True, slots=True)
class DependencyNode:
    node_id: str
    label: str = ""
    metadata: dict[str, object] | None = None

    def to_dict(self) -> dict[str, object]:
        return {"node_id": self.node_id, "label": self.label, "metadata": dict(self.metadata or {})}


@dataclass(frozen=True, slots=True)
class DependencyEdge:
    source: str
    target: str
    relation: RelationType
    metadata: dict[str, object] | None = None

    def to_dict(self) -> dict[str, object]:
        return {
            "source": self.source,
            "target": self.target,
            "relation": self.relation.value,
            "metadata": dict(self.metadata or {}),
        }


@dataclass(frozen=True, slots=True)
class GraphFailureEvidence:
    label: str
    severity: str
    invariant_category: str
    affected_nodes: tuple[str, ...]
    affected_edges: tuple[tuple[str, str, str], ...]
    details: dict[str, object]


@dataclass(frozen=True, slots=True)
class GraphComparisonResult:
    dependency_integrity_score: float
    orphan_rate: float
    detached_dependency_rate: float
    acyclicity_preserved: bool
    reachability_preservation: float
    temporal_order_violation_rate: float
    causal_preservation_score: float
    failures: tuple[GraphFailureEvidence, ...]


class GraphComparisonError(Exception):
    pass


class DependencyGraph:
    graph_version = "1.0"

    def __init__(self) -> None:
        self._nodes: dict[str, DependencyNode] = {}
        self._edges: set[tuple[str, str, RelationType]] = set()
        self._edge_metadata: dict[tuple[str, str, RelationType], dict[str, object]] = {}

    def add_node(self, node: DependencyNode) -> None:
        self._nodes[node.node_id] = DependencyNode(node.node_id, node.label, dict(node.metadata or {}))

    def add_edge(self, edge: DependencyEdge) -> None:
        if edge.source not in self._nodes or edge.target not in self._nodes:
            raise GraphComparisonError("edge references unknown node")
        key = (edge.source, edge.target, edge.relation)
        self._edges.add(key)
        self._edge_metadata[key] = dict(edge.metadata or {})

    def get_nodes(self) -> tuple[DependencyNode, ...]:
        return tuple(sorted(self._nodes.values(), key=lambda item: item.node_id))

    def get_edges(self) -> tuple[DependencyEdge, ...]:
        ordered = sorted(self._edges, key=lambda item: (item[0], item[1], item[2].value))
        return tuple(
            DependencyEdge(source, target, relation, dict(self._edge_metadata.get((source, target, relation), {})))
            for source, target, relation in ordered
        )

    def to_dict(self) -> dict[str, object]:
        return {
            "graph_version": self.graph_version,
            "nodes": [node.to_dict() for node in self.get_nodes()],
            "edges": [edge.to_dict() for edge in self.get_edges()],
        }

    @classmethod
    def from_dict(cls, payload: dict[str, object]) -> DependencyGraph:
        graph = cls()
        for node_data in payload.get("nodes", []):
            node_payload = dict(node_data)
            graph.add_node(
                DependencyNode(
                    node_id=str(node_payload["node_id"]),
                    label=str(node_payload.get("label", "")),
                    metadata=dict(node_payload.get("metadata") or {}),
                )
            )
        for edge_data in payload.get("edges", []):
            edge_payload = dict(edge_data)
            graph.add_edge(
                DependencyEdge(
                    source=str(edge_payload["source"]),
                    target=str(edge_payload["target"]),
                    relation=RelationType(str(edge_payload["relation"])),
                    metadata=dict(edge_payload.get("metadata") or {}),
                )
            )
        return graph


class DependencyGraphComparator:
    def compare(self, original: DependencyGraph, reconstructed: DependencyGraph) -> GraphComparisonResult:
        try:
            return self._compare(original, reconstructed)
        except GraphComparisonError:
            raise
        except Exception as exc:
            raise GraphComparisonError(f"graph comparison failed: {exc}") from exc

    def _compare(self, original: DependencyGraph, reconstructed: DependencyGraph) -> GraphComparisonResult:
        original_nodes = {node.node_id for node in original.get_nodes()}
        reconstructed_nodes = {node.node_id for node in reconstructed.get_nodes()}
        original_edges = {(e.source, e.target, e.relation.value) for e in original.get_edges()}
        reconstructed_edges = {(e.source, e.target, e.relation.value) for e in reconstructed.get_edges()}

        node_union = len(original_nodes | reconstructed_nodes)
        node_jaccard = 1.0 if node_union == 0 else len(original_nodes & reconstructed_nodes) / node_union
        edge_union = len(original_edges | reconstructed_edges)
        edge_jaccard = 1.0 if edge_union == 0 else len(original_edges & reconstructed_edges) / edge_union
        dependency_integrity_score = (node_jaccard + edge_jaccard) / 2.0

        original_incoming = self._incoming_counts(original_nodes, original_edges)
        reconstructed_incoming = self._incoming_counts(reconstructed_nodes, reconstructed_edges)
        orphan_nodes = tuple(
            sorted(node for node in original_nodes if original_incoming.get(node, 0) > 0 and reconstructed_incoming.get(node, 0) == 0)
        )
        original_node_count = len(original_nodes)
        orphan_rate = 0.0 if original_node_count == 0 else len(orphan_nodes) / original_node_count

        missing_edges = tuple(sorted(original_edges - reconstructed_edges))
        original_edge_count = len(original_edges)
        detached_dependency_rate = 0.0 if original_edge_count == 0 else len(missing_edges) / original_edge_count

        original_acyclic = self._is_acyclic(original_nodes, original_edges)
        reconstructed_acyclic = self._is_acyclic(reconstructed_nodes, reconstructed_edges)
        acyclicity_preserved = not (original_acyclic and not reconstructed_acyclic)

        original_reach = self._reachable_pairs(original_nodes, original_edges)
        reconstructed_reach = self._reachable_pairs(reconstructed_nodes, reconstructed_edges)
        if len(original_reach) == 0:
            reachability_preservation = 1.0
        else:
            reachability_preservation = len(original_reach & reconstructed_reach) / len(original_reach)

        temporal_order_violation_rate = self._temporal_violation_rate(original, reconstructed)

        original_causal = {edge for edge in original_edges if edge[2] == RelationType.CAUSAL.value}
        reconstructed_causal = {edge for edge in reconstructed_edges if edge[2] == RelationType.CAUSAL.value}
        if len(original_causal) == 0:
            causal_preservation_score = 1.0
        else:
            causal_preservation_score = len(original_causal & reconstructed_causal) / len(original_causal)

        failures: list[GraphFailureEvidence] = []
        if missing_edges:
            failures.append(
                GraphFailureEvidence(
                    label="DETACHED_DEPENDENCY",
                    severity="HIGH",
                    invariant_category="dependency",
                    affected_nodes=(),
                    affected_edges=missing_edges,
                    details={
                        "reason": "original_dependency_edges_missing",
                        "missing_edge_count": len(missing_edges),
                        "original_edge_count": original_edge_count,
                    },
                )
            )
        if orphan_nodes:
            failures.append(
                GraphFailureEvidence(
                    label="ORPHAN_DEPENDENCY",
                    severity="HIGH",
                    invariant_category="reachability",
                    affected_nodes=orphan_nodes,
                    affected_edges=(),
                    details={
                        "reason": "dependent_nodes_lost_incoming_dependencies",
                        "orphan_count": len(orphan_nodes),
                        "original_node_count": original_node_count,
                    },
                )
            )
        if not acyclicity_preserved:
            failures.append(
                GraphFailureEvidence(
                    label="CYCLE_INTRODUCED",
                    severity="CRITICAL",
                    invariant_category="acyclicity",
                    affected_nodes=tuple(sorted(reconstructed_nodes)),
                    affected_edges=tuple(sorted(reconstructed_edges)),
                    details={
                        "reason": "cycle_introduced_in_reconstructed_graph",
                        "original_acyclic": original_acyclic,
                        "reconstructed_acyclic": reconstructed_acyclic,
                    },
                )
            )
        if reachability_preservation < 1.0:
            failures.append(
                GraphFailureEvidence(
                    label="GRAPH_FRAGMENTATION",
                    severity="HIGH",
                    invariant_category="reachability",
                    affected_nodes=tuple(sorted(original_nodes)),
                    affected_edges=missing_edges,
                    details={
                        "reason": "reachable_pairs_not_preserved",
                        "reachability_preservation": reachability_preservation,
                        "original_reachable_pair_count": len(original_reach),
                        "reconstructed_reachable_pair_count": len(reconstructed_reach),
                    },
                )
            )
        if temporal_order_violation_rate > 0.0:
            failures.append(
                GraphFailureEvidence(
                    label="TEMPORAL_ORDER_VIOLATION",
                    severity="CRITICAL",
                    invariant_category="ordering",
                    affected_nodes=tuple(sorted(original_nodes & reconstructed_nodes)),
                    affected_edges=(),
                    details={
                        "reason": "deterministic_topological_order_inversions",
                        "temporal_order_violation_rate": temporal_order_violation_rate,
                    },
                )
            )
        if causal_preservation_score < 1.0:
            missing_causal = tuple(sorted(original_causal - reconstructed_causal))
            failures.append(
                GraphFailureEvidence(
                    label="CAUSAL_DEPENDENCY_LOSS",
                    severity="HIGH",
                    invariant_category="causality",
                    affected_nodes=(),
                    affected_edges=missing_causal,
                    details={
                        "reason": "causal_edges_not_preserved",
                        "causal_preservation_score": causal_preservation_score,
                        "missing_causal_edge_count": len(missing_causal),
                        "original_causal_edge_count": len(original_causal),
                    },
                )
            )

        return GraphComparisonResult(
            dependency_integrity_score=dependency_integrity_score,
            orphan_rate=orphan_rate,
            detached_dependency_rate=detached_dependency_rate,
            acyclicity_preserved=acyclicity_preserved,
            reachability_preservation=reachability_preservation,
            temporal_order_violation_rate=temporal_order_violation_rate,
            causal_preservation_score=causal_preservation_score,
            failures=tuple(failures),
        )

    def _incoming_counts(self, nodes: set[str], edges: set[tuple[str, str, str]]) -> dict[str, int]:
        counts = {node: 0 for node in nodes}
        for _, target, _ in edges:
            if target in counts:
                counts[target] += 1
        return counts

    def _adjacency(self, nodes: set[str], edges: set[tuple[str, str, str]]) -> dict[str, set[str]]:
        adjacency: dict[str, set[str]] = {node: set() for node in nodes}
        for source, target, _ in edges:
            if source in adjacency and target in adjacency:
                adjacency[source].add(target)
        return adjacency

    def _is_acyclic(self, nodes: set[str], edges: set[tuple[str, str, str]]) -> bool:
        return len(self._deterministic_topological_order(nodes, edges)) == len(nodes)

    def _deterministic_topological_order(self, nodes: set[str], edges: set[tuple[str, str, str]]) -> tuple[str, ...]:
        indegree = {node: 0 for node in nodes}
        adjacency = self._adjacency(nodes, edges)
        for source, targets in adjacency.items():
            if source not in indegree:
                continue
            for target in targets:
                indegree[target] += 1
        queue = deque(sorted(node for node, degree in indegree.items() if degree == 0))
        order: list[str] = []
        while queue:
            node = queue.popleft()
            order.append(node)
            for neighbor in sorted(adjacency[node]):
                indegree[neighbor] -= 1
                if indegree[neighbor] == 0:
                    queue.append(neighbor)
            queue = deque(sorted(queue))
        return tuple(order)

    def _reachable_pairs(self, nodes: set[str], edges: set[tuple[str, str, str]]) -> set[tuple[str, str]]:
        adjacency = self._adjacency(nodes, edges)
        reachable: set[tuple[str, str]] = set()
        for start in sorted(nodes):
            queue = deque(sorted(adjacency[start]))
            seen: set[str] = set()
            while queue:
                node = queue.popleft()
                if node in seen:
                    continue
                seen.add(node)
                reachable.add((start, node))
                for neighbor in sorted(adjacency[node]):
                    if neighbor not in seen:
                        queue.append(neighbor)
        return reachable

    def _temporal_violation_rate(self, original: DependencyGraph, reconstructed: DependencyGraph) -> float:
        original_nodes = {node.node_id for node in original.get_nodes()}
        reconstructed_nodes = {node.node_id for node in reconstructed.get_nodes()}
        shared_nodes = original_nodes & reconstructed_nodes
        if len(shared_nodes) < 2:
            return 0.0

        original_edges = {
            (edge.source, edge.target, edge.relation.value)
            for edge in original.get_edges()
            if edge.source in shared_nodes and edge.target in shared_nodes
        }
        reconstructed_edges = {
            (edge.source, edge.target, edge.relation.value)
            for edge in reconstructed.get_edges()
            if edge.source in shared_nodes and edge.target in shared_nodes
        }
        original_order = self._deterministic_topological_order(shared_nodes, original_edges)
        reconstructed_order = self._deterministic_topological_order(shared_nodes, reconstructed_edges)
        if len(original_order) != len(shared_nodes) or len(reconstructed_order) != len(shared_nodes):
            return 0.0

        original_index = {node: index for index, node in enumerate(original_order)}
        reconstructed_index = {node: index for index, node in enumerate(reconstructed_order)}
        total_pairs = 0
        inversions = 0
        ordered_nodes = sorted(shared_nodes)
        for i, first in enumerate(ordered_nodes):
            for second in ordered_nodes[i + 1 :]:
                original_diff = original_index[first] - original_index[second]
                reconstructed_diff = reconstructed_index[first] - reconstructed_index[second]
                total_pairs += 1
                if original_diff * reconstructed_diff < 0:
                    inversions += 1
        if total_pairs == 0:
            return 0.0
        return inversions / total_pairs
