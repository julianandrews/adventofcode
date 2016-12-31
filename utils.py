from collections import namedtuple
from fractions import gcd
try:
    from queue import Queue
except ImportError:
    from Queue import Queue


def read_data(n):
    with open('data/d{}.txt'.format(n)) as f:
        data = f.read()
    return data


def lines(data):
    return [line.strip() for line in data.strip().split('\n')]


def lcm(a, b):
    return (a * b) // gcd(a, b)


class AStar(object):
    def __init__(self, starting_node):
        self.closed_nodes = set()
        self.open_nodes = set([starting_node])
        self.distances = {starting_node: 0}
        self.estimated_distances = {starting_node: 0}
        self.parents = {}

    def get_neighbors(self, node):
        raise NotImplementedError

    def remaining_distance_heuristic(self, node):
        raise NotImplementedError

    def get_edge_weight(self, node, neighbor):
        raise NotImplementedError

    def is_end(self, node):
        raise NotImplementedError

    def get_next_node(self):
        node = min(self.open_nodes, key=lambda node: self.estimated_distances[node])
        self.open_nodes.remove(node)
        return node

    def lower_bound_estimate(self, node):
        return self.distances[node] + self.remaining_distance_heuristic(node)

    def reconstruct_path(self, node):
        path = []
        while node in self.parents:
            path.append(node)
            node = self.parents[node]
        return path

    def __call__(self):
        while self.open_nodes:
            node = self.get_next_node()
            self.closed_nodes.add(node)
            if self.is_end(node):
                return self.reconstruct_path(node)
            for neighbor in self.get_neighbors(node):
                new_distance = self.distances[node] + self.get_edge_weight(node, neighbor)

                if neighbor in self.closed_nodes:
                    continue
                elif neighbor not in self.open_nodes:
                    self.open_nodes.add(neighbor)

                if self.distances.get(neighbor) is None or new_distance < self.distances[neighbor]:
                    self.distances[neighbor] = new_distance
                    self.estimated_distances[neighbor] = self.lower_bound_estimate(neighbor)
                    self.parents[neighbor] = node
        return None


BFSNode = namedtuple('BFSNode', ('value', 'depth', 'parent'))


def bfs(start, get_neighbors):
    seen = set([start])
    queue = Queue()
    queue.put(BFSNode(start, 0, None))
    while not queue.empty():
        node = queue.get()
        yield node
        for neighbor in get_neighbors(node.value):
            if neighbor not in seen:
                queue.put(BFSNode(neighbor, node.depth + 1, node))
                seen.add(neighbor)
