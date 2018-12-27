import heapq


class BFSNode:
    def __init__(self, value, depth, parent, value_key=None):
        self.value = value
        self.depth = depth
        self.parent = parent
        self.value_key = value_key

    def get_path(self):
        path = [self.value]
        node = self
        while node.parent is not None:
            node = node.parent
            path.append(node.value)

        return list(reversed(path))

    def __lt__(self, other):
        return (self.depth, self.value_key) < (other.depth, other.value_key)


def bfs(start, get_neighbors, value_key_func=None):
    seen = set([start])
    queue = [BFSNode(start, 0, None, value_key_func(start))]
    while queue:
        node = heapq.heappop(queue)
        yield node
        for neighbor in get_neighbors(node.value):
            if neighbor not in seen:
                heapq.heappush(queue, BFSNode(neighbor, node.depth + 1, node, value_key_func(neighbor)))
                seen.add(neighbor)
