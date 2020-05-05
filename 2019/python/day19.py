from utils import read_data

from utils.intcode import VM


class TractorBeam:
    def __init__(self, program):
        self.program = program

    def is_active(self, x, y):
        if x < 0 or y < 0:
            return False

        def inputs():
            yield x
            yield y
        vm = VM(self.program[:], inputs=inputs())
        return bool(next(vm.outputs()))

    def lower_left_edge(self):
        x, y = 0, 0
        while True:
            yield x, y
            if self.is_active(x, y + 1):
                y += 1
            elif self.is_active(x + 1, y):
                x += 1
            else:
                x = 0
                y += 1
                while not self.is_active(x, y):
                    x += 1
                    if x > y:
                        x = 0
                        y += 1

    def beam_string(self, height=50, width=50):
        tiles = []
        for y in range(height):
            tiles.append(
                "".join("#" if self.is_active(x, y) else "." for x in range(width))
            )
        return "\n".join(tiles)


def p1(program):
    tractor_beam = TractorBeam(program)
    return sum(1 for x in range(50) for y in range(50) if tractor_beam.is_active(x, y))


def p2(program):
    tractor_beam = TractorBeam(program)
    for x, y in tractor_beam.lower_left_edge():
        if tractor_beam.is_active(x + 99, y - 99):
            return 10000 * x + y - 99


def run_tests(program):
    tractor_beam = TractorBeam(program)
    assert tractor_beam.is_active(-1, 0) == 0
    assert tractor_beam.is_active(0, 0) == 1
    assert tractor_beam.is_active(1, 0) == 0
    assert tractor_beam.is_active(10, 14) == 1
    assert tractor_beam.is_active(9, 14) == 0

if __name__ == "__main__":
    data = read_data()
    program = [int(x) for x in data.strip().split(',')]
    run_tests(program)
    print("All tests passed")

    print("Part 1: {}".format(p1(program)))
    print("Part 2: {}".format(p2(program)))
