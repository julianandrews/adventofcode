import fileinput


class LightGrid:
    def __init__(self, rows):
        self.rows = [list(line) for line in lines]

    def should_live(self, x, y):
        cell_alive = self.rows[y][x] == "#"
        neighbor_count = 0
        for j in range(y - 1, y + 2):
            if 0 <= j < len(self.rows):
                for i in range(x - 1, x + 2):
                    if 0 <= i < len(self.rows[j]):
                        if self.rows[j][i] == "#":
                            neighbor_count += 1
        if cell_alive:
            neighbor_count -= 1

        return neighbor_count == 3 or (neighbor_count == 2 and cell_alive)

    def tick(self):
        self.rows = [[
            "#" if self.should_live(x, y) else "."
            for x in range(len(self.rows[0]))
        ] for y in range(len(self.rows))]

    def run(self, t):
        for i in range(t):
            self.tick()

    def light_count(self):
        return sum(1 for row in self.rows for c in row if c == "#")

    def __str__(self):
        return "\n".join("".join(row) for row in self.rows)


class RiggedLightGrid(LightGrid):
    def __init__(self, rows):
        super().__init__(rows)
        self.stuck = {
            (0, 0),
            (0, len(self.rows) - 1),
            (len(self.rows[0]) - 1, 0),
            (len(self.rows[0]) - 1, len(self.rows) - 1),
        }
        for x, y in self.stuck:
            self.rows[y][x] = "#"

    def should_live(self, x, y):
        return (x, y) in self.stuck or super().should_live(x, y)


def part1(grid):
    grid = LightGrid(lines)
    grid.run(100)
    return grid.light_count()


def part2(grid):
    grid = RiggedLightGrid(lines)
    grid.run(100)
    return grid.light_count()


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]

    print("Part 1: %s" % part1(lines))
    print("Part 2: %s" % part2(lines))
