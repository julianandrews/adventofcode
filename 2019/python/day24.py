import collections

from utils import read_data


class BugLayer:
    def __init__(self, value=0):
        self.value = value

    def __str__(self):
        return "\n".join(
            "".join("#" if self.bug_at(x, y) else "." for x in range(5))
            for y in range(5)
        )

    def __hash__(self):
        return self.value

    def __eq__(self, other):
        return self.value == other.value

    def is_empty(self):
        return self.value == 0

    def borders_lower(self):
        return bool(self.value & 141440)

    def borders_upper(self):
        return bool(self.value & 33080895)

    @property
    def bug_count(self):
        return bin(self.value).count("1")

    def bug_at(self, x, y):
        if x < 0 or x > 4 or y < 0 or y > 4:
            raise ValueError(f"Coordinates {(x, y)} out of bounds!")
        return bool(self.value & (1 << (x + 5 * y)))

    @classmethod
    def from_string(cls, data):
        value = 0
        s = data.replace(" ", "").replace("\n", "")
        if len(s) != 25:
            raise ValueError("Invalid input data")
        if s[12] != ".":
            raise ValueError("Unexpected bug in center")

        for i, c in enumerate(s):
            if c == "#":
                value ^= 1 << i
        return cls(value)


class SimpleBugMaze:
    NEIGHBOR_MASKS = [
        34, 69, 138, 276, 520,
        1089, 2210, 4420, 8840, 16656,
        34848, 70720, 141440, 282880, 532992,
        1115136, 2263040, 4526080, 9052160, 17055744,
        2129920, 5308416, 10616832, 21233664, 8912896,
    ]

    def __init__(self, data):
        self.layer = BugLayer.from_string(data)

    def __hash__(self):
        return hash(self.layer)

    def __eq__(self, other):
        return self.layer == other.layer

    @property
    def biodiversity_rating(self):
        return self.layer.value

    def neighbor_count(self, i):
        mask = self.NEIGHBOR_MASKS[i]
        return bin(self.layer.value & mask).count("1")

    def step(self):
        new_value = 0
        for i in range(25):
            bit = 1 << i
            neighbors = self.neighbor_count(i)
            if neighbors == 1 or (neighbors == 2 and not self.layer.value & bit):
                new_value ^= bit
        self.layer.value = new_value


class BugMaze:
    NEIGHBOR_MASKS = {
        -1: [
            0, 0, 0, 0, 0,
            0, 0, 31, 0, 0,
            0, 1082401, 0, 17318416, 0,
            0, 0, 32505856, 0, 0,
            0, 0, 0, 0, 0,
        ],
        0: [
            34, 69, 138, 276, 520,
            1089, 2210, 324, 8840, 16656,
            34848, 66624, 141440, 278784, 532992,
            1115136, 2263040, 4521984, 9052160, 17055744,
            2129920, 5308416, 10616832, 21233664, 8912896
        ],
        1: [
            2176, 128, 128, 128, 8320,
            2048, 0, 0, 0, 8192,
            2048, 0, 0, 0, 8192,
            2048, 0, 0, 0, 8192,
            133120, 131072, 131072, 131072, 139264
        ],
    }

    def __init__(self, data):
        self.layers = {
            -1: BugLayer(),
            0: BugLayer.from_string(data),
            1: BugLayer(),
        }

    def __str__(self):
        return "\n\n".join(
            f"Depth {depth}:\n{layer}" for depth, layer in sorted(self.layers.items())
        )

    def step(self):
        new_layers = {}
        for depth, layer in self.layers.items():
            new_layers[depth] = self.step_layer(depth)

        min_depth = min(new_layers.keys())
        if new_layers[min_depth].borders_lower():
            new_layers[min_depth - 1] = BugLayer()
        max_depth = max(new_layers.keys())
        if new_layers[max_depth].borders_lower():
            new_layers[max_depth + 1] = BugLayer()

        self.layers = new_layers

    def step_layer(self, depth):
        layer = self.layers[depth]
        value = 0
        for i in range(25):
            if i == 12:
                continue
            bit = 1 << i
            neighbors = self.neighbor_count(depth, i)
            if neighbors == 1 or (neighbors == 2 and not layer.value & bit):
                value ^= bit
        return BugLayer(value)

    def neighbor_count(self, depth, i):
        count = 0
        for offset in self.NEIGHBOR_MASKS:
            mask = self.NEIGHBOR_MASKS[offset][i]
            layer = self.layers.get(depth + offset, BugLayer())
            count += bin(layer.value & mask).count("1")
        return count

    def bug_count(self):
        return sum(layer.bug_count for layer in self.layers.values())


def p1(data):
    maze = SimpleBugMaze(data)
    seen = set()
    while maze.layer not in seen:
        seen.add(maze.layer)
        maze.step()
    return maze.biodiversity_rating


def p2(data):
    maze = BugMaze(data)
    for _ in range(200):
        maze.step()
    return maze.bug_count()


def run_tests():
    data = """
        ....#
        #..#.
        #..##
        ..#..
        #....
    """
    maze = SimpleBugMaze(data)
    for _ in range(4):
        maze.step()
    expected = SimpleBugMaze("""
        ####.
        ....#
        ##..#
        .....
        ##...
    """)
    assert maze == expected
    assert p1(data) == 2129920

    maze = BugMaze(data)
    for _ in range(10):
        maze.step()
    assert maze.bug_count() == 99


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data()
    print("Part 1: {}".format(p1(data)))
    print("Part 2: {}".format(p2(data)))
