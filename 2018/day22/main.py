import enum
import fileinput


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

    def __str__(self):
        lines = []
        for y in range(self.target[1] + 1):
            lines.append([self.region_type(x, y).tile() for x in range(self.target[0] + 1)])

        return "\n".join("".join(line) for line in lines)


def p1(depth, target):
    caverns = Caverns(depth, target)
    return caverns.risk_level()


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    depth = int(lines[0].split(": ")[1])
    target = tuple(int(x) for x in lines[1].split(": ")[1].split(","))
    print(p1(depth, target))
