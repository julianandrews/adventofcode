import collections
import fileinput


class ForestMap:
    def __init__(self, lines):
        self.grid = tuple(tuple(c for c in line) for line in lines)
        self.time = 0

    def __str__(self):
        return "\n".join("".join(row) for row in self.grid)

    def tick(self):
        self.grid = tuple(
            tuple(self.new_value(x, y) for x in range(len(row)))
            for y, row in enumerate(self.grid)
        )
        self.time += 1

    def new_value(self, x, y):
        current_value = self.grid[y][x]
        neighbor_values = [self.grid[w][v] for v, w in self.neighbors(x, y)]
        counts = collections.Counter(neighbor_values)
        if current_value == "." and counts["|"] >= 3:
            return "|"
        elif current_value == "|" and counts["#"] >= 3:
            return "#"
        elif current_value == "#" and (counts["|"] == 0 or counts["#"] == 0):
            return "."
        else:
            return current_value

    def neighbors(self, x, y):
        return [
            (x + i, y + j) for i in (-1, 0, 1) for j in (-1, 0, 1)
            if (i or j)
            and x + i >= 0 and x + i < len(self.grid[0])
            and y + j >= 0 and y + j < len(self.grid)
        ]

    def total_resource_count(self):
        counts = collections.Counter(str(self))
        return counts["|"] * counts["#"]


def p1(lines):
    forest_map = ForestMap(lines)
    for i in range(10):
        forest_map.tick()

    return forest_map.total_resource_count()


def p2(lines):
    forest_map = ForestMap(lines)
    seen = {}
    while forest_map.grid not in seen:
        seen[forest_map.grid] = forest_map.time
        forest_map.tick()

    period = forest_map.time - seen[forest_map.grid]
    cycles, remainder = divmod(1000000000 - forest_map.time, period)
    for i in range(remainder):
        forest_map.tick()

    return forest_map.total_resource_count()


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    print(p1(lines))
    print(p2(lines))
