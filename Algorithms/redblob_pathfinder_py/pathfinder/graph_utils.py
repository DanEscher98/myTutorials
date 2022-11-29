from typing import Optional, Any, List, Dict, TypeAlias, Tuple, Iterator
from abc import ABC, abstractmethod
import json
import collections


Location: TypeAlias = str
GridCell: TypeAlias = Tuple[int, int]
Node: TypeAlias = Location | GridCell

NodeFrom: TypeAlias = Dict[Node, Optional[Node]]
GPath: TypeAlias = Optional[List[Node]]

AdjacencyMatrix: TypeAlias = Any  # TODO
IncidenceMatrix: TypeAlias = Any  # TODO
AdjacencyList: TypeAlias = Dict[Location, List[Location]]


class Queue:
    def __init__(self):
        self.elements = collections.deque()

    def empty(self) -> bool:
        return not self.elements

    def put(self, x: Node):
        self.elements.append(x)

    def get(self) -> Node:
        return self.elements.popleft()


class Graph(ABC):
    @abstractmethod
    def neighbours(self, loc: Node) -> Iterator[Node]:
        pass


class SimpleGraph(Graph):
    def __init__(self, edges: AdjacencyList):
        self.edges: AdjacencyList = edges

    def neighbours(self, loc: Location) -> List[Location]:
        try:
            return self.edges[loc]
        except KeyError:
            raise SystemExit(f"Node: {loc} not found")


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

    def neighbours(self, loc: GridCell) -> Iterator[GridCell]:
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


def get_valid_graph(file_path: str, id: int) -> Optional[AdjacencyList]:
    with open(file_path) as file:
        data = json.load(file)[id]["edges"]
    # TODO: add check functionality
    return data
