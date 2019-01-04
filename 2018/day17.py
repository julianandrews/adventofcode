import fileinput

from utils.graphs import bfs


class GroundChart:
    def __init__(self, clay_points):
        self.left_edge = min(500, min(x for (x, y) in clay_points)) - 1
        width = max(max(x for (x, y) in clay_points), 500) - self.left_edge + 2
        self.min_y = min(y for (x, y) in clay_points)
        depth = max(y for (x, y) in clay_points) + 1

        self.grid = [
            ["#" if (x + self.left_edge, y) in clay_points else "." for x in range(width)]
            for y in range(depth)
        ]
        self.fountains = {(500 - self.left_edge, 0)}
        self.flow_front = self.fountains.copy()

    def tick(self):
        new_flows = self.flow()
        changed = bool(new_flows)
        self.settle(new_flows)

        return changed

    def flow(self):
        new_flows = set()
        for x, y in list(self.flow_front):
            below = self.tile_value(x, y + 1)
            if below == ".":
                changed = True
                self.flow_front.add((x, y + 1))
            elif below in "#~":
                if self.tile_value(x - 1, y) == ".":
                    changed = True
                    self.flow_front.add((x - 1, y))
                if self.tile_value(x + 1, y) == ".":
                    changed = True
                    self.flow_front.add((x + 1, y))
            self.grid[y][x] = "|"
            self.flow_front.remove((x, y))
            new_flows.add((x, y))

        return new_flows

    def settle(self, new_flows):
        def get_neighbors(point):
            x, y = point
            return [
                p for p in [(x - 1, y), (x + 1, y)]
                if self.grid[p[1]][p[0]] == "|"
            ]

        def is_good(point):
            x, y = point
            return self.tile_value(x, y + 1) in "#~" \
                and self.tile_value(x - 1, y) in "#~|" \
                and self.tile_value(x + 1, y) in "#~|"

        while new_flows:
            connected_group = {node.value for node in bfs(new_flows.pop(), get_neighbors)}
            new_flows -= connected_group

            if all(is_good(p) for p in connected_group):
                for x, y in connected_group:
                    self.grid[y][x] = "~"
                    if self.grid[y - 1][x] == "|":
                        self.flow_front.add((x, y - 1))

    def tile_value(self, x, y):
        return " " if y >= len(self.grid) else self.grid[y][x]

    def __str__(self):
        return "\n".join(
            "".join(self.tile_value(x, y) for x, t in enumerate(row))
            for y, row in enumerate(self.grid)
        )


def points_for_line(line):
    fixed_part, range_part = line.split(", ")
    a = int(fixed_part[2:])
    start, end = (int(x) for x in range_part[2:].split(".."))
    if fixed_part[0] == "x":
        return [(a, b) for b in range(start, end + 1)]
    else:
        return [(b, a) for b in range(start, end + 1)]


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    clay_points = set(sum(map(points_for_line, lines), []))
    chart = GroundChart(clay_points)
    changing = True
    while changing:
        changing = chart.tick()

    water_count = len([1 for row in chart.grid for c in row if c == "~"])
    flow_count = len([1 for row in chart.grid for c in row if c == "|"])
    print(flow_count + water_count - chart.min_y)
    print(water_count)
