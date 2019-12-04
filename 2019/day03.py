import enum
import utils


class Direction(enum.Enum):
    UP = "U"
    DOWN = "D"
    LEFT = "L"
    RIGHT = "R"


class Wire:
    def __init__(self, instructions):
        self.signal_distances = {}
        signal_distance = 0
        x = y = 0
        for instruction in instructions:
            direction = Direction(instruction[0])
            distance = int(instruction[1:])
            for i in range(distance):
                if direction == Direction.UP:
                    y += 1
                elif direction == Direction.DOWN:
                    y -= 1
                if direction == Direction.RIGHT:
                    x += 1
                elif direction == Direction.LEFT:
                    x -= 1
                signal_distance += 1
                if (x, y) not in self.signal_distances:
                    self.signal_distances[(x, y)] = signal_distance

    @property
    def points(self):
        return self.signal_distances.keys()


def p1(wire1, wire2):
    intersections = wire1.points & wire2.points
    return min(abs(x) + abs(y) for x, y in intersections)


def p2(wire1, wire2):
    intersections = wire1.points & wire2.points
    return min(wire1.signal_distances[p] + wire2.signal_distances[p] for p in intersections)


if __name__ == "__main__":
    wire_a1 = Wire(["R8", "U5", "L5", "D3"])
    wire_a2 = Wire(["U7", "R6", "D4", "L4"])
    assert p1(wire_a1, wire_a2) == 6
    assert p2(wire_a1, wire_a2) == 30

    wire_b1 = Wire(['R75', 'D30', 'R83', 'U83', 'L12', 'D49', 'R71', 'U7', 'L72'])
    wire_b2 = Wire(['U62', 'R66', 'U55', 'R34', 'D71', 'R55', 'D58', 'R83'])
    assert p1(wire_b1, wire_b2) == 159
    assert p2(wire_b1, wire_b2) == 610

    wire_c1 = Wire(['R98', 'U47', 'R26', 'D63', 'R33', 'U87', 'L62', 'D20', 'R33', 'U53', 'R51'])
    wire_c2 = Wire(['U98', 'R91', 'D20', 'R16', 'D67', 'R40', 'U7', 'R15', 'U6', 'R7'])
    assert p1(wire_c1, wire_c2) == 135
    assert p2(wire_c1, wire_c2) == 410

    data = utils.read_data(3)
    wires = [Wire(line.split(",")) for line in data.strip().split("\n")]
    print("Part 1: %d" % p1(*wires))
    print("Part 2: %d" % p2(*wires))
