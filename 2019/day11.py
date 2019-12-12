import enum

from utils import read_data
from utils.intcode import VM


class Direction(enum.Enum):
    UP = 0
    RIGHT = 1
    DOWN = 2
    LEFT = 3

    def turn_left(self):
        return Direction((self.value - 1) % 4)

    def turn_right(self):
        return Direction((self.value + 1) % 4)


def pairs(seq):
    try:
        while True:
            yield (next(seq), next(seq))
    except StopIteration:
        pass


class Robot:
    def __init__(self, vm):
        self.vm = vm
        self.vm.inputs = self.inputs()
        self.painted_panels = set()
        self.x = 0
        self.y = 0
        self.direction = Direction.UP

    def inputs(self):
        while True:
            yield (self.x, self.y) in self.painted_panels

    def move(self):
        if self.direction == Direction.UP:
            self.y += 1
        elif self.direction == Direction.RIGHT:
            self.x += 1
        if self.direction == Direction.DOWN:
            self.y -= 1
        elif self.direction == Direction.LEFT:
            self.x -= 1

    def paint_instructions(self):
        for paint_white, turn_right in pairs(self.vm.outputs()):
            paint_location = (self.x, self.y)
            if paint_white:
                self.painted_panels.add((self.x, self.y))
            else:
                self.painted_panels.discard((self.x, self.y))
            if turn_right:
                self.direction = self.direction.turn_right()
            else:
                self.direction = self.direction.turn_left()
            self.move()
            yield paint_location, (self.x, self.y), paint_white

    def panel_string(self):
        min_x = min(l[0] for l in self.painted_panels)
        max_x = max(l[0] for l in self.painted_panels)
        min_y = min(l[1] for l in self.painted_panels)
        max_y = max(l[1] for l in self.painted_panels)

        return "\n".join(
            "".join(
                "█" if (x, y) in self.painted_panels else " "
                for x in range(min_x, max_x + 1))
            for y in range(min_y, max_y + 1))


def p1(program):
    robot = Robot(VM(program[:]))
    painted_locations = set()
    for paint_location, move_location, paint_white in robot.paint_instructions():
        if paint_white:
            painted_locations.add(paint_location)
    return len(painted_locations)


def p2(robot):
    robot = Robot(VM(program[:]))
    robot.painted_panels.add((0, 0))
    for _ in robot.paint_instructions():
        pass
    return "\n".join(reversed(robot.panel_string().split("\n")))


def run_tests():
    class FakeVM:
        def outputs(self):
            yield from [1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0]

    robot = Robot(FakeVM())
    instructions = robot.paint_instructions()
    assert next(instructions) == ((0, 0), (-1, 0), 1)
    assert next(instructions) == ((-1, 0), (-1, -1), 0)
    assert next(instructions) == ((-1, -1), (0, -1), 1)
    assert next(instructions) == ((0, -1), (0, 0), 1)
    assert next(instructions) == ((0, 0), (1, 0), 0)
    assert next(instructions) == ((1, 0), (1, 1), 1)
    assert next(instructions) == ((1, 1), (0, 1), 1)


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(11)
    program = [int(x) for x in data.strip().split(',')]
    print("Part 1: {}".format(p1(program)))
    print("Part 2: {}".format("\n" + p2(program)))
