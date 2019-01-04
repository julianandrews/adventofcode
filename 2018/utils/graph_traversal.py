import heapq


class TraversalNode:
    """Class representing a single Node in the traversal of a graph.

    self.value  -- the value stored in the node.
    self.index  -- index for the order in which node was discovered.
    self.depth  -- depth of the node in the traversal tree.
    self.parent -- this node's parent node in the traversal tree.
    """
    def __init__(self, value, index, depth, parent):
        self.value = value
        self.index = index
        self.depth = depth
        self.parent = parent

    def get_path(self):
        """Yields the traversal path from this node back to start."""
        node = self
        while node:
            yield node.value
            node = node.parent

    def __lt__(self, other):
        return self.index < other.index


def graph_traversal(start, neighbors, node_key):
    """Returns a TraversalNode generator.

    Values in the graph must be unique.

    start     -- the value to begin the search from.
    neighbors -- a function from values to an iterable of neighbor values.
    node_key  -- a function which takes a TraversalNode and returns a value.
                 Discovered nodes will be visited in node_key order.
    """
    seen = set([start])
    start_node = TraversalNode(start, 0, 0, None)
    queue = [(node_key(start_node), start_node)]

    index = 0
    while queue:
        node = heapq.heappop(queue)[1]
        yield node
        for neighbor in neighbors(node.value):
            if neighbor not in seen:
                index += 1
                neighbor_node = TraversalNode(neighbor, index, node.depth + 1, node)
                heapq.heappush(queue, (node_key(neighbor_node), neighbor_node))
                seen.add(neighbor)


def bfs(start, neighbors, sort_key=None):
    """Returns a TraversalNode generator in breadth first order.

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
            return (-node.depth, node.index)
    else:
        def node_key(node):
            return (-node.depth, sort_key(node.value), node.index)

    return graph_traversal(start, neighbors, node_key)
