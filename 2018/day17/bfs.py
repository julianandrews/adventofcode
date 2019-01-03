import heapq


class BFSNode:
    """Class representing a single Node in the traversal of a graph."""
    def __init__(self, value, depth, parent, sort_key=None):
        self.value = value
        self.depth = depth
        self.parent = parent
        self.sort_key = sort_key

    def get_path(self):
        """Yields the traversal path from this node back to start."""
        node = self
        while node:
            yield node.value
            node = node.parent

    def __lt__(self, other):
        return (self.depth, self.sort_key) < (other.depth, other.sort_key)


def bfs(start, neighbors, sort_key=None):
    """Returns a BFSNode generator in breadth first order.

    Values in the graph must be unique.

    start     -- the value to begin the search from.
    neighbors -- a function from values to an iterable of neighbor values.
    sort_key  -- optional sort key to determine traversal order for points
                 at equal depth.
    """
    sort_key = (lambda x: None) if sort_key is None else sort_key
    seen = set([start])
    queue = [BFSNode(start, 0, None, sort_key(start))]
    while queue:
        node = heapq.heappop(queue)
        yield node
        for neighbor in neighbors(node.value):
            if neighbor not in seen:
                neighbor_node = BFSNode(neighbor, node.depth + 1, node, sort_key(neighbor))
                heapq.heappush(queue, neighbor_node)
                seen.add(neighbor)
