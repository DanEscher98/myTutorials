from pathfinder.graph_utils import (
    Graph,
    Queue,
    Node,
    NodeFrom,
    GPath,
    WeightedGraph,
    PriorityQueue,
    NodeWeightsFrom,
)


def breadth_fst(graph: Graph, start: Node, goal: Node) -> NodeFrom:
    frontier = Queue()
    frontier.put(start)
    came_from: NodeFrom = {}
    came_from[start] = None

    while not frontier.empty():
        current: Node = frontier.get()
        if current == goal:  # greedy exit
            break

        for next in graph.neighbours(current):
            if next not in came_from:
                frontier.put(next)
                came_from[next] = current

    return came_from


def dijkstra_search(
        graph: WeightedGraph, start: Node, goal: Node) -> NodeWeightsFrom:
    frontier = PriorityQueue()
    frontier.put(start, 0)
    came_from: NodeFrom = {start: None}
    cost_so_far: dict[Node, float] = {start: 0}

    while not frontier.empty():
        current: Node = frontier.get()
        if current == goal:
            break
        for next in graph.neighbours(current):
            if not (next_cost := graph.cost(current, next)):
                raise SystemExit(f"Nodes not connected: {current} - {next}")
            new_cost = cost_so_far[current] + next_cost
            if next not in cost_so_far or new_cost < cost_so_far[next]:
                cost_so_far[next] = new_cost
                frontier.put(next, new_cost)
                came_from[next] = current

    return came_from, cost_so_far


def get_final_cost(path: GPath, cost_so_far) -> float:
    if not path:
        raise SystemExit("Empty Path")

    cost = 0.0
    for node in path:
        cost += cost_so_far[node]
    return cost


def reconstruct_path(came_from: NodeFrom, start: Node, goal: Node) -> GPath:
    path = [goal]
    counter = 0

    try:
        step = came_from[goal]
    except KeyError:
        return None

    while step:
        path.append(step)
        counter += 1
        if step == start:
            break
        if counter > len(came_from):
            return None
        step = came_from[step]

    path.reverse()
    return path
