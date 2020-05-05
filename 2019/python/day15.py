import enum

from utils import read_data
from utils.direction import Direction
from utils.intcode import VM
from utils.graphs import bfs


class StatusCode(enum.Enum):
    HIT_WALL = 0
    MOVED = 1
    FOUND_OXYGEN = 2
    UNEXPLORED = 3

class ShipMap:
    def __init__(self):
        self.explorer_position = (0, 0)
        self.status_map = {self.explorer_position: StatusCode.MOVED}

    def __getitem__(self, position):
        return self.status_map[position]

    def __setitem__(self, position, status):
        self.status_map[position] = status

    def __contains__(self, position):
        return position in self.status_map

    def neighbors(self, position):
        x, y = position
        candidates = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        return [c for c in candidates if self.status_map.get(c) != StatusCode.HIT_WALL]

    def map_string(self):
        min_x = min(x for x, y in self.status_map)
        max_x = max(x for x, y in self.status_map)
        min_y = min(y for x, y in self.status_map)
        max_y = max(y for x, y in self.status_map)
        char_map = {
            StatusCode.MOVED: '·',
            StatusCode.HIT_WALL: '█',
            StatusCode.FOUND_OXYGEN: '$',
            StatusCode.UNEXPLORED: '▒',
        }

        def get_char(x, y):
            if (x, y) == self.explorer_position:
                return '@'
            return char_map[self.status_map.get((x, y), StatusCode.UNEXPLORED)]

        return "\n".join(
            "".join(get_char(x, y) for x in range(min_x, max_x + 1))
            for y in range(max_y, min_y - 1, -1)
        )

class ShipExplorer:
    def __init__(self, vm):
        self.vm = vm
        self.vm.inputs = self.inputs()
        self.route = []
        self.ship_map = ShipMap()
        self.next_input = None

    def inputs(self):
        while True:
            yield self.next_input

    @staticmethod
    def direction_input(direction):
        if direction == Direction.NORTH:
            return 1
        elif direction == Direction.SOUTH:
            return 2
        elif direction == Direction.WEST:
            return 3
        elif direction == Direction.EAST:
            return 4

    def try_move(self, direction):
        self.next_input = self.direction_input(direction)
        status_code = StatusCode(next(self.vm.outputs()))
        next_position = direction.next_position(self.ship_map.explorer_position)
        self.ship_map[next_position] = status_code
        if status_code != StatusCode.HIT_WALL:
            self.ship_map.explorer_position = next_position
            self.route.append(direction)

    def backtrack(self):
        direction = self.route.pop().reverse()
        self.next_input = self.direction_input(direction)
        status_code = StatusCode(next(self.vm.outputs()))
        self.ship_map.explorer_position = direction.next_position(self.ship_map.explorer_position)
        if status_code != self.ship_map[self.ship_map.explorer_position]:
            raise RuntimeError("Inconsistent map data.")

    def explore(self):
        while True:
            open_directions = [
                direction for direction in Direction
                if direction.next_position(self.ship_map.explorer_position) not in self.ship_map
            ]
            if open_directions:
                self.try_move(open_directions[0])
            else:
                if self.route:
                    self.backtrack()
                else:
                    break


def p1(ship_map):
    for node in bfs((0, 0), ship_map.neighbors):
        if ship_map[node.value] == StatusCode.FOUND_OXYGEN:
            return node.depth


def p2(ship_map):
    start_position = next(
        node.value for node in
        bfs((0, 0), ship_map.neighbors)
        if ship_map[node.value] == StatusCode.FOUND_OXYGEN
    )
    *_, last_node = bfs(start_position, ship_map.neighbors)
    return last_node.depth


if __name__ == "__main__":
    print("No tests run")

    data = read_data()
    program = [int(x) for x in data.strip().split(',')]
    vm = VM(program[:])
    ship_explorer = ShipExplorer(vm)
    ship_explorer.explore()

    print("Part 1: {}".format(p1(ship_explorer.ship_map)))
    print("Part 2: {}".format(p2(ship_explorer.ship_map)))
