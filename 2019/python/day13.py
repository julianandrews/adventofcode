import enum

from utils import read_data
from utils.intcode import VM


def triples(it):
    try:
        while True:
            a = next(it)
            b = next(it)
            c = next(it)
            yield (a, b, c)
    except StopIteration:
        pass


class Tile(enum.Enum):
    EMPTY = 0
    WALL = 1
    BLOCK = 2
    PADDLE = 3
    BALL = 4

    def __str__(self):
        if self == self.EMPTY:
            return " "
        elif self == self.WALL:
            return "┼"
        elif self == self.BLOCK:
            return "█"
        elif self == self.PADDLE:
            return "▄"
        elif self == self.BALL:
            return "°"


class ArcadeMachine:
    def __init__(self, vm):
        self.grid = {}
        self.score = 0
        self.vm = vm
        self.vm.inputs = self.ai()
        self.last_ball_location = None
        self.last_paddle_location = None

    def run(self):
        for x, y, tile_id in triples(self.vm.outputs()):
            if x == -1 and y == 0:
                self.score = tile_id
                continue
            tile = Tile(tile_id)
            if tile == Tile.BALL:
                self.last_ball_location = x
            elif tile == Tile.PADDLE:
                self.last_paddle_location = x
            self.grid[(x, y)] = tile

    def ai(self):
        while True:
            offset = self.last_ball_location - self.last_paddle_location
            yield 0 if offset == 0 else (1 if offset > 0 else -1)

    def __str__(self):
        min_x = min(x for x, y in self.grid)
        max_x = max(x for x, y in self.grid)
        min_y = min(y for x, y in self.grid)
        max_y = max(y for x, y in self.grid)

        return "\n".join(
            "".join(str(self.grid.get((x, y), " ")) for x in range(min_x, max_x + 1))
            for y in range(min_y, max_y + 1)
        )


def p1(program):
    vm = VM(program)
    machine = ArcadeMachine(vm)
    machine.run()

    return sum(1 for tile in machine.grid.values() if tile == Tile.BLOCK)


def p2(program):
    vm = VM(program)
    vm.set_memory(0, 2)
    machine = ArcadeMachine(vm)
    machine.run()

    return machine.score


def run_tests():
    class FakeVM:
        def outputs(self):
            yield from [1, 2, 3, 6, 5, 4]

    machine = ArcadeMachine(FakeVM())
    machine.run()
    assert machine.grid[(1, 2)] == Tile.PADDLE
    assert machine.grid[(6, 5)] == Tile.BALL
    assert len(machine.grid) == 2


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data()
    program = [int(x) for x in data.strip().split(',')]
    print("Part 1: {}".format(p1(program)))
    print("Part 2: {}".format(p2(program)))
