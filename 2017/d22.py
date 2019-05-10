import enum

from utils import read_data


class NodeState(enum.Enum):
    CLEAN = 0
    WEAKENED = 1
    INFECTED = 2
    FLAGGED = 3

    def next_state(self):
        return NodeState((self.value + 1) % len(NodeState))


@enum.unique
class Direction(enum.Enum):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3

    def turn_right(self):
        return Direction((self.value + 1) % len(Direction))

    def turn_left(self):
        return Direction((self.value - 1) % len(Direction))

    def reverse(self):
        return Direction((self.value + 2) % len(Direction))

    def advance(self, x, y):
        if self == Direction.NORTH:
            return (x, y - 1)
        elif self == Direction.EAST:
            return (x + 1, y)
        elif self == Direction.SOUTH:
            return (x, y + 1)
        elif self == Direction.WEST:
            return (x - 1, y)


class BaseInfectionMap:
    def __init__(self, infected_nodes):
        self.current_node = (0, 0)
        self.direction = Direction.NORTH
        self.infection_bursts = 0

    @classmethod
    def from_lines(cls, lines):
        nodes = set()
        cx, cy = len(lines[0]) // 2, len(lines) // 2
        for j, line in enumerate(lines):
            for i, c in enumerate(line):
                if c == "#":
                    nodes.add((i - cx, j - cy))

        return cls(nodes)

    def tick(self):
        raise NotImplementedError


class SimpleInfectionMap(BaseInfectionMap):
    def __init__(self, infected_nodes):
        super().__init__(self)
        self.infected_nodes = set(infected_nodes)

    def diagram(self):
        min_x = min(n[0] for n in self.infected_nodes)
        min_y = min(n[1] for n in self.infected_nodes)
        max_x = max(n[0] for n in self.infected_nodes)
        max_y = max(n[1] for n in self.infected_nodes)
        rows = []
        for y in range(min_y, max_y + 1):
            rows.append(" ".join(
                "." if (x, y) in self.infected_nodes else "#"
                for x in range(min_x, max_x + 1)
            ))

        return "\n".join(rows)

    def tick(self):
        if self.current_node in self.infected_nodes:
            self.direction = self.direction.turn_right()
            self.infected_nodes.remove(self.current_node)
        else:
            self.direction = self.direction.turn_left()
            self.infected_nodes.add(self.current_node)
            self.infection_bursts += 1
        self.current_node = self.direction.advance(*self.current_node)


class ComplexInfectionMap(BaseInfectionMap):
    def __init__(self, infected_nodes):
        super().__init__(self)
        self.node_states = {}
        for node in infected_nodes:
            self.node_states[node] = NodeState.INFECTED

    def tick(self):
        node_state = self.get_node_state()
        if node_state == NodeState.CLEAN:
            self.direction = self.direction.turn_left()
        elif node_state == NodeState.WEAKENED:
            self.infection_bursts += 1
        elif node_state == NodeState.INFECTED:
            self.direction = self.direction.turn_right()
        else:
            self.direction = self.direction.reverse()
        self.cycle_node_state()
        self.current_node = self.direction.advance(*self.current_node)

    def cycle_node_state(self):
        self.node_states[self.current_node] = self.get_node_state().next_state()

    def get_node_state(self):
        return self.node_states.get(self.current_node, NodeState.CLEAN)

    def state(self):
        return tuple([self.current_node, self.direction, frozenset(self.node_states.items())])


def infection_bursts(infection_map, time):
    for _ in range(time):
        infection_map.tick()

    return infection_map.infection_bursts


def run_tests():
    map_data = [
        "..#",
        "#..",
        "...",
    ]

    assert infection_bursts(SimpleInfectionMap.from_lines(map_data), 70) == 41
    assert infection_bursts(SimpleInfectionMap.from_lines(map_data), 10000) == 5_587
    assert infection_bursts(ComplexInfectionMap.from_lines(map_data), 100) == 26
    assert infection_bursts(ComplexInfectionMap.from_lines(map_data), 10_000_000) == 2_511_944


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(22)
    lines = data.strip().split("\n")
    simple_map = SimpleInfectionMap.from_lines(lines)
    complex_map = ComplexInfectionMap.from_lines(lines)

    print("Part 1: {}".format(infection_bursts(simple_map, 10_000)))
    print("Part 2: {}".format(infection_bursts(complex_map, 10_000_000)))
