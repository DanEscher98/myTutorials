"""
Based on [Intro to A*](https://www.redblobgames.com/pathfinding/a-star/introduction.html)
"""

from pathfinder.breadth_fst import SimpleGraph, get_valid_graph

def main():
    data = get_valid_graph("data/graphs.json", 0)
    graph = SimpleGraph(data) if data else {}
    print(data)
    print("Hello world asdf")

if __name__ == "__main__":
    main()
