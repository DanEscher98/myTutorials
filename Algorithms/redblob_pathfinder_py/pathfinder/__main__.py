"""
Based on [Intro to A*](https://www.redblobgames.com/pathfinding)
"""

import sys
from pathfinder.graph_utils import get_valid_graph, SimpleGraph
from pathfinder.breadth_fst import breadth_fst, reconstruct_path


def main():
    id = sys.argv[1]
    (start, end) = ("e", "8")

    if data := get_valid_graph("data/graphs.json", int(id)):
        graph = SimpleGraph(data)
    else:
        raise Exception("Bad data")

    path = breadth_fst(graph, start, end)
    print("Path: {}".format(reconstruct_path(path, start, end)))


if __name__ == "__main__":
    main()
