"""
Based on [Intro to A*](https://www.redblobgames.com/pathfinding)
"""

import sys
from pathfinder.graph_utils import get_graph, WeightedGraph, SimpleGraph
from pathfinder.breadth_fst import (
    breadth_fst,
    reconstruct_path,
    dijkstra_search,
    get_final_cost,
)


def main():
    id = sys.argv[1]
    (start, end) = ("1", "20")

    graph = get_graph("data/graphs.json", int(id))

    if isinstance(graph, WeightedGraph):
        node_from, cost_so_far = dijkstra_search(graph, start, end)
        path = reconstruct_path(node_from, start, end)
        cost = get_final_cost(path, cost_so_far)
        print(f"Dijkstra =>\t{cost=:.2f}\n\t\t{path=}", end="\n\n")

        node_from = breadth_fst(graph, start, end)
        path = reconstruct_path(node_from, start, end)
        cost = get_final_cost(path, cost_so_far)
        print(f"Breadth_1st =>\t{cost=:.2f}\n\t\t{path=}")

    elif isinstance(graph, SimpleGraph):
        node_from = breadth_fst(graph, start, end)
        path = reconstruct_path(node_from, start, end)
        print(f"Breadth_1st => {path=}")

    else:
        raise SystemExit("Invalid graph type")


if __name__ == "__main__":
    main()
