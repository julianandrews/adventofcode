import enum

from utils import read_data


class Direction(enum.Enum):
    UP = 1
    RIGHT = 2
    DOWN = 3
    LEFT = 4

    def continuations(self):
        yield self
        if self in (Direction.UP, Direction.DOWN):
            yield Direction.RIGHT
            yield Direction.LEFT
        elif self in (Direction.RIGHT, Direction.LEFT):
            yield Direction.UP
            yield Direction.DOWN


class Map:
    OFFSETS = {
        Direction.UP: (0, -1),
        Direction.RIGHT: (1, 0),
        Direction.DOWN: (0, 1),
        Direction.LEFT: (-1, 0),
    }

    def __init__(self, diagram):
        self.diagram = diagram

    def walk(self):
        direction = Direction.DOWN
        position = self.path_start()
        while True:
            yield position, direction, self.char_at(position)
            for d in direction.continuations():
                p = self.move(position, d)
                if self.char_at(p) != " ":
                    position, direction = p, d
                    break
            else:
                break

    def path_start(self):
        return self.diagram[0].index("|"), 0

    def move(self, position, direction):
        return tuple(a + b for a, b in zip(position, self.OFFSETS[direction]))

    def char_at(self, position):
        try:
            return self.diagram[position[1]][position[0]]
        except IndexError:
            return " "


def path_letters(diagram):
    route_map = Map(diagram)
    return "".join(c for _, _, c in route_map.walk() if c.isalpha())


def path_length(diagram):
    route_map = Map(diagram)
    return sum(1 for _ in route_map.walk())


def run_tests():
    diagram = [
        "     |         ",
        "     |  +--+   ",
        "     A  |  C   ",
        " F---|----E|--+",
        "     |  |  |  D",
        "     +B-+  +--+",
    ]

    assert path_letters(diagram) == "ABCDEF"
    assert path_length(diagram) == 38


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(19)
    diagram = data.split("\n")
    print("Part 1: {}".format(path_letters(diagram)))
    print("Part 2: {}".format(path_length(diagram)))
