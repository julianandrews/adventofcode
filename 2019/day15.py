import enum

from utils import read_data
from utils.direction import Direction
from utils.intcode import VM
from utils.graphs import bfs


class StatusCode(enum.Enum):
    HIT_WALL = 0
    MOVED = 1
    FOUND_OXYGEN = 2


class Robot:
    def __init__(self, vm):
        self.vm = vm
        self.vm.inputs = self.inputs()
        self.position = (0, 0)
        self.route = []
        self.ship_map = {(0, 0): StatusCode.MOVED}
        self.next_input = None
        self.explored = False

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
        next_position = direction.next_position(self.position)
        self.ship_map[next_position] = status_code
        if status_code != StatusCode.HIT_WALL:
            self.position = next_position
            self.route.append(direction)

    def backtrack(self):
        direction = self.route.pop().reverse()
        self.next_input = self.direction_input(direction)
        status_code = StatusCode(next(self.vm.outputs()))
        self.position = direction.next_position(self.position)
        if status_code != self.ship_map[self.position]:
            raise RuntimeError("Inconsistent map data.")

    def explore(self):
        while not self.explored:
            open_directions = [
                direction for direction in Direction
                if direction.next_position(self.position) not in self.ship_map
            ]
            if open_directions:
                self.try_move(open_directions[0])
            else:
                if self.route:
                    self.backtrack()
                else:
                    self.explored = True
                    break

    def map_neighbors(self, position):
        if not self.explored:
            raise RuntimeError("Must call self.explore() first")
        x, y = position
        candidates = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        return [c for c in candidates if self.ship_map.get(c) != StatusCode.HIT_WALL]

    def map_string(self):
        if not self.explored:
            raise RuntimeError("Must call self.explore() first")
        min_x = min(x for x, y in self.ship_map)
        max_x = max(x for x, y in self.ship_map)
        min_y = min(y for x, y in self.ship_map)
        max_y = max(y for x, y in self.ship_map)
        char_map = {
            StatusCode.MOVED: '·', StatusCode.HIT_WALL: '█', StatusCode.FOUND_OXYGEN: '$'
        }

        def get_char(x, y):
            if (x, y) == self.position:
                return "@"
            else:
                return char_map[self.ship_map.get((x, y), StatusCode.HIT_WALL)]

        return "\n".join(
            "".join(get_char(x, y) for x in range(min_x, max_x + 1))
            for y in range(min_y, max_y + 1)
        )


def p1(robot):
    for node in bfs((0, 0), robot.map_neighbors):
        if robot.ship_map[node.value] == StatusCode.FOUND_OXYGEN:
            return node.depth


def p2(robot):
    start_position = next(
        node.value for node in
        bfs((0, 0), robot.map_neighbors)
        if robot.ship_map[node.value] == StatusCode.FOUND_OXYGEN
    )
    *_, last_node = bfs(start_position, robot.map_neighbors)
    return last_node.depth


if __name__ == "__main__":
    print("No tests run")

    data = read_data(15)
    program = [int(x) for x in data.strip().split(',')]
    vm = VM(program[:])
    robot = Robot(vm)
    robot.explore()
    print("Part 1: {}".format(p1(robot)))
    print("Part 2: {}".format(p2(robot)))
