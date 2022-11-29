from pathfinder.graph_utils import Graph, Queue, Node, NodeFrom, GPath


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
