from typing import (
    Optional, Any, List, Dict, TypeAlias, Tuple, Iterable
)
from abc import ABC, abstractmethod
import json
import collections
import heapq
import math


Location: TypeAlias = str
GridCell: TypeAlias = Tuple[int, int]
Node: TypeAlias = Location | GridCell
IterNodes: TypeAlias = Iterable[Node]
Matrix: TypeAlias = List[List[float]]
# IterNode: TypeAlias = Iterable[Node] | Dict[]

NodeFrom: TypeAlias = Dict[Node, Optional[Node]]
NodeWeightsFrom: TypeAlias = Tuple[NodeFrom, Dict[Node, float]]
GPath: TypeAlias = Optional[IterNodes]

AdjacencyList: TypeAlias = Dict[Location, IterNodes]
AdjacencyMatrix: TypeAlias = Tuple[IterNodes, Matrix]  # TODO
AdjacencyWeights: TypeAlias = Dict[Node, Dict[Node, float]]
IncidenceMatrix: TypeAlias = Any  # TODO


class Queue:
    def __init__(self):
        self.elements = collections.deque()

    def empty(self) -> bool:
        return not self.elements

    def put(self, x: Node):
        self.elements.append(x)

    def get(self) -> Node:
        return self.elements.popleft()


class PriorityQueue:
    def __init__(self):
        self.elements: List[Tuple[float, Node]] = []

    def empty(self):
        return not self.elements

    def put(self, item: Node, priority: float):
        heapq.heappush(self.elements, (priority, item))

    def get(self) -> Node:
        return heapq.heappop(self.elements)[1]


class Graph(ABC):
    @abstractmethod
    def neighbours(self, loc: Node) -> Iterable[Node]:
        pass


class SimpleGraph(Graph):
    def __init__(self, edges: AdjacencyList):
        self.edges: AdjacencyList = edges

    def neighbours(self, loc: Location) -> IterNodes:
        try:
            return self.edges[loc]
        except KeyError:
            raise SystemExit(f"Node: {loc} not found")


class WeightedGraph(Graph):
    def __init__(self, node_labels, matrix):
        self.node_labels: List[Node] = node_labels
        self.edges: Matrix = matrix
        self.__weighted_edges: Dict[Node, Any] = dict(map(
            lambda node: (node, dict(self.__neighbours(node))),
            self.node_labels))

    def __node2idx(self, loc: Node) -> int:
        return self.node_labels.index(loc)

    def __neighbours(self, loc: Node) -> Iterable[Tuple[Node, float]]:
        row = self.edges[self.__node2idx(loc)]
        for (node, weight) in zip(self.node_labels, row):
            if not math.isnan(weight):
                yield node, weight

    def cost(self, from_node: Node, to_node: Node) -> Optional[float]:
        return self.__weighted_edges[from_node][to_node]

    def neighbours(self, loc: Node) -> Iterable[Node]:
        return self.__weighted_edges[loc].keys()


class SquareGrid(Graph):
    def __init__(self, width: int, height: int, walls: List[GridCell] = []):
        self.width = width
        self.height = height
        self.walls: List[GridCell] = walls

    def __in_bounds(self, loc: GridCell) -> bool:
        (x, y) = loc
        return 0 <= x < self.width and 0 <= y < self.height

    def __passable(self, loc: GridCell) -> bool:
        return loc not in self.walls

    def neighbours(self, loc: GridCell) -> Iterable[GridCell]:
        (x, y) = loc
        neighbours = [(x + 1, y), [x - 1, y], (x, y - 1), (x, y + 1)]
        if (x + y) % 2 == 0:
            neighbours.reverse()
        results = filter(self.__in_bounds, neighbours)
        results = filter(self.__passable, results)
        return results


class GridWithWeight(SquareGrid):
    def __init__(self, width: int, height: int):
        super().__init__(width, height)
        self.weights: dict[GridCell, float] = {}

    def cost(self, _: GridCell, to_node: GridCell) -> float:
        try:
            return self.weights.get(to_node, 1)
        except KeyError:
            raise SystemExit(f"Node: {to_node} not found")


def get_graph(
        file_path: str, id: int) -> Graph:
    with open(file_path) as file:
        data = json.load(file)[id]  # TODO: catch error if invalid json

    match data["type"]:
        case "AdjacencyList":
            try:
                edges = get_graph_adjacency_list(data["edges"])
            except Exception as err:
                raise SystemExit(f"Get graph Error: {err}")
            return SimpleGraph(edges)
        case "AdjacencyMatrix":
            try:
                nodes, matrix = get_graph_adjacency_matrix(data["edges"])
            except Exception as err:
                raise SystemExit(f"Get graph Error: {err}")
            return WeightedGraph(nodes, matrix)
        case _:
            raise SystemExit("Invalid graph type")


def get_graph_adjacency_list(data) -> AdjacencyList:
    # TODO: add check functionality
    return data


def get_graph_adjacency_matrix(data) -> AdjacencyMatrix:
    def get_value(node, idx) -> float:
        if not (weight := data[node][idx]):
            weight = 'nan'
        try:
            return float(weight)
        except ValueError:
            raise SystemExit(f"Value: {weight} isn't float nor null")

    nodes = data["Nodes"]

    matrix = [[get_value(node, idx)
               for idx in range(len(nodes))] for node in nodes]

    return (nodes, matrix)


def __printmatrix(matrix):
    print('\n'.join([' '.join(['{:4}'.format(item)
                    for item in row])
                    for row in matrix]), end="\n\n")
