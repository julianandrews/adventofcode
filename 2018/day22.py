import collections
import enum
import fileinput

from utils.astar import astar


class Gear(enum.Enum):
    NONE = 0
    TORCH = 1
    CLIMBING_GEAR = 2


CavernState = collections.namedtuple("CavernState", ["x", "y", "gear"])


class RegionType(enum.Enum):
    ROCKY = 0
    WET = 1
    NARROW = 2

    @classmethod
    def from_erosion_level(cls, erosion_level):
        return cls(erosion_level % 3)

    def tile(self):
        if self == RegionType.ROCKY:
            return "."
        elif self == RegionType.WET:
            return "="
        else:
            return "|"

    def risk_level(self):
        return self.value

    def allowed_gear(self):
        if self == RegionType.ROCKY:
            return {Gear.TORCH, Gear.CLIMBING_GEAR}
        elif self == RegionType.WET:
            return {Gear.CLIMBING_GEAR, Gear.NONE}
        else:
            return {Gear.NONE, Gear.TORCH}


class Caverns:
    def __init__(self, depth, target):
        self.depth = depth
        self.target = target
        self._erosion_levels = {}

    def geologic_index(self, x, y):
        if (x, y) in {(0, 0), self.target}:
            return 0
        elif y == 0:
            return 16807 * x
        elif x == 0:
            return 48271 * y
        else:
            return self.erosion_level(x - 1, y) * self.erosion_level(x, y - 1)

    def erosion_level(self, x, y):
        if (x, y) not in self._erosion_levels:
            self._erosion_levels[(x, y)] = (self.geologic_index(x, y) + self.depth) % 20183

        return self._erosion_levels[(x, y)]

    def region_type(self, x, y):
        return RegionType.from_erosion_level(self.erosion_level(x, y))

    def risk_level(self):
        return sum(
            self.region_type(x, y).risk_level()
            for y in range(self.target[1] + 1)
            for x in range(self.target[0] + 1)
        )

    def get_neighbors(self, state):
        current_region = self.region_type(state.x, state.y)
        for gear in current_region.allowed_gear():
            if gear != state.gear:
                yield CavernState(state.x, state.y, gear)

        adjacent = (
            (state.x - 1, state.y),
            (state.x + 1, state.y),
            (state.x, state.y - 1),
            (state.x, state.y + 1)
        )
        for x, y in adjacent:
            if x >= 0 and y >= 0:
                destination = self.region_type(x, y)
                if state.gear in destination.allowed_gear():
                    yield(CavernState(x, y, state.gear))

    def __str__(self):
        lines = []
        for y in range(self.target[1] + 6):
            lines.append([self.region_type(x, y).tile() for x in range(self.target[0] + 6)])

        return "\n".join("".join(line) for line in lines)


def p1(depth, target, debug=False):
    caverns = Caverns(depth, target)
    if debug:
        print(caverns)
    return caverns.risk_level()


def p2(depth, target, debug=False):
    caverns = Caverns(depth, target)
    start = CavernState(0, 0, Gear.TORCH)
    end = CavernState(target[0], target[1], Gear.TORCH)

    path, distance = astar(
        start,
        lambda a: a == end,
        caverns.get_neighbors,
        lambda a, b: 7 if a.gear != b.gear else 1,
        lambda a: abs(a.x - end.x) + abs(a.y - end.y) + (7 if a.gear != end.gear else 0)
    )

    if debug:
        print(caverns)
        for state in path:
            print("%s, %s:  %s" % (state.x, state.y, state.gear.name))

    return distance


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    depth = int(lines[0].split(": ")[1])
    target = tuple(int(x) for x in lines[1].split(": ")[1].split(","))
    print(p1(depth, target))
    print(p2(depth, target))
