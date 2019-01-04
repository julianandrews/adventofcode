import heapq


class GraphNode:
    """Class representing a single Node in the traversal of a graph."""
    def __init__(self, value, depth, parent):
        self.value = value
        self.depth = depth
        self.parent = parent

    def get_path(self):
        """Yields the traversal path from this node back to start."""
        node = self
        while node:
            yield node.value
            node = node.parent


def graph_traversal(start, neighbors, node_key):
    """Returns a GraphNode generator.

    Values in the graph must be unique.

    start     -- the value to begin the search from.
    neighbors -- a function from values to an iterable of neighbor values.
    node_key  -- a function which takes a GraphNode and returns a value.
                 Discovered nodes will be visited in node_key order.
    """
    seen = set([start])
    start_node = GraphNode(start, 0, None)
    queue = [(node_key(start_node), start_node)]

    while queue:
        node = heapq.heappop(queue)[1]
        yield node
        for neighbor in neighbors(node.value):
            if neighbor not in seen:
                neighbor_node = GraphNode(neighbor, node.depth + 1, node)
                heapq.heappush(queue, (node_key(neighbor_node), neighbor_node))
                seen.add(neighbor)


def bfs(start, neighbors, sort_key=None):
    """Returns a GraphNode generator in breadth first order.

    Values in the graph must be unique.

    start     -- the value to begin the search from.
    neighbors -- a function from values to an iterable of neighbor values.
    sort_key  -- optional sort key to determine traversal order for points
                 at equal depth.
    """

    if sort_key is None:
        def node_key(node):
            return node.depth
    else:
        def node_key(node):
            return (node.depth, sort_key(node.value))

    return graph_traversal(start, neighbors, node_key)


def dfs(start, neighbors, sort_key=None):
    """Returns a BFSNode generator in breadth first order.

    Values in the graph must be unique.

    start     -- the value to begin the search from.
    neighbors -- a function from values to an iterable of neighbor values.
    sort_key  -- optional sort key to determine traversal order for points
                 at equal depth.
    """

    if sort_key is None:
        def node_key(node):
            return -node.depth
    else:
        def node_key(node):
            return (-node.depth, sort_key(node.value))

    return graph_traversal(start, neighbors, node_key)
