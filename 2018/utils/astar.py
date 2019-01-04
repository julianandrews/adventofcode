def astar(start, is_end, get_neighbors, get_edge_weight, heuristic):
    closed_nodes = set()
    open_nodes = set([start])
    distances = {start: 0}
    estimated_distances = {start: 0}
    parents = {}

    while open_nodes:
        node = min(open_nodes, key=lambda node: estimated_distances[node])
        open_nodes.remove(node)
        closed_nodes.add(node)
        if is_end(node):
            distance = distances[node]
            path = []
            while node in parents:
                path.append(node)
                node = parents[node]
            path.append(start)
            return list(reversed(path)), distance
        for neighbor in get_neighbors(node):
            new_distance = distances[node] + get_edge_weight(node, neighbor)

            if neighbor not in closed_nodes:
                open_nodes.add(neighbor)

                if distances.get(neighbor) is None or new_distance < distances[neighbor]:
                    distances[neighbor] = new_distance
                    estimated_distances[neighbor] = distances[neighbor] + heuristic(neighbor)
                    parents[neighbor] = node
    return None
