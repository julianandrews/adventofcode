from utils import AStar, lines, read_data


class Node(object):
    def __init__(self, line):
        words = line.split()
        self.used = int(words[2][:-1])
        self.avail = int(words[3][:-1])
        self.x = int(words[0].split('-')[1][1:])
        self.y = int(words[0].split('-')[2][1:])

    def __str__(self):
        return "{}, {}".format(self.x, self.y)

    def __repr__(self):
        return "Node({}, {})".format(self.x, self.y)

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y


class Grid(object):
    def __init__(self, data):
        self.nodes = {}
        for line in lines(data)[2:]:
            node = Node(line)
            self.nodes[(node.x, node.y)] = node
        self.max_x = max(node.x for node in self.nodes.values())
        self.max_y = max(node.y for node in self.nodes.values())

    def __getitem__(self, location):
        return self.nodes[location]

    def empty_nodes(self):
        return [node for node in self.nodes.values() if node.used == 0]

    def neighbors(self, x, y):
        for (u, v) in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
            if 0 <= u <= self.max_x and 0 <= v <= self.max_y:
                neighbor = self.nodes[(u, v)]
                if not neighbor.used > 92:
                    yield (neighbor.x, neighbor.y)


class GridAStar(AStar):
    def __init__(self, data):
        self.grid = Grid(data)
        empty_nodes = self.grid.empty_nodes()
        assert len(empty_nodes) == 1
        super(GridAStar, self).__init__(
            ((self.grid.max_x, 0), (empty_nodes[0].x, empty_nodes[0].y))
        )

    def get_neighbors(self, state):
        data, empty = state
        for neighbor in self.grid.neighbors(*empty):
            if neighbor == data:
                yield (empty, neighbor)
            else:
                yield (data, neighbor)

    def remaining_distance_heuristic(self, state):
        return state[0][0] + state[0][1] + abs(state[1][0] - state[0][0]) + abs(state[1][1] - state[0][1]) - 1

    def get_edge_weight(self, state, neighbor):
        return 1

    def is_end(self, state):
        return state[0][0] == state[0][1] == 0


def viable_pairs(data):
    nodes = [Node(line) for line in lines(data)[2:]]
    by_used = sorted(nodes, key=lambda node: node.used)
    by_avail = sorted(nodes, key=lambda node: node.avail)
    pairs = set()
    i = 0
    for node in by_used:
        if node.used != 0:
            while i < len(by_avail):
                if by_avail[i].avail >= node.used:
                    break
                i += 1
            for other_node in by_avail[i:]:
                pairs.add(tuple([node, other_node]))
    return pairs


if __name__ == '__main__':
    data = read_data(22)

    print(len(viable_pairs(data)))
    path = GridAStar(data)()
    print(len(path))
