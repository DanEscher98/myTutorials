from typing import List, TypeVar, Optional, Any
import json
from abc import ABC, abstractmethod

Location = TypeVar('Location')
class Graph(ABC):
    @abstractmethod
    def neighbors(self, id: Location) -> list[Location]:
        pass

class SimpleGraph(Graph):
    def __init__(self, graph_data: dict):
        self.edges: dict[Location, List[Location]] = graph_data

    def neighbors(self, id: Location) -> List[Location]:
        return self.edges[id]


def get_valid_graph(file_path: str, id: int) -> Optional[Any]:
    with open(file_path) as file:
        data = json.load(file)[id]["edges"]
    # TODO: data validation
    return data


