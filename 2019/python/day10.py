import itertools
import math

from utils import read_data


class AsteroidField:
    def __init__(self, s):
        self.grid = [[c for c in line.strip()] for line in s.strip().split("\n")]
        if not self.grid or not all(len(line) == len(self.grid[0]) for line in self.grid):
            raise ValueError("Invalid grid")

    @property
    def width(self):
        return len(self.grid[0])

    @property
    def height(self):
        return len(self.grid)

    def has_asteroid(self, x, y):
        return self.grid[y][x] == "#"

    def get_directions(self, location):
        directions = set()
        for x in range(self.width):
            dx = x - location[0]
            for y in range(self.height):
                dy = y - location[1]
                denom = math.gcd(dx, dy) or 1
                if dx or dy:
                    directions.add((dx // denom, dy // denom))

        return sorted(
            directions,
            key=lambda d: (-math.atan2(*d) + math.pi) % (2 * math.pi)
        )

    def first_visible_asteroid(self, location, direction):
        x, y = location
        dx, dy = direction
        while True:
            x += dx
            y += dy
            if x < 0 or x >= self.width or y < 0 or y >= self.height:
                break
            if self.has_asteroid(x, y):
                return (x, y)

    def visible_count(self, location):
        return sum(
            1 for direction in self.get_directions(location)
            if self.first_visible_asteroid(location, direction))

    def monitoring_station(self):
        return max(
            (
                (x, y)
                for x in range(self.height)
                for y in range(self.width)
                if self.has_asteroid(x, y)
            ),
            key=lambda location: self.visible_count(location))

    def __str__(self):
        return "\n".join("".join(line) for line in self.grid)


def nth_asteroid(field, location, n):
    count = 0
    for direction in itertools.cycle(field.get_directions(location)):
        asteroid_coords = field.first_visible_asteroid(location, direction)
        if asteroid_coords is not None:
            x, y = asteroid_coords
            field.grid[y][x] = "."
            count += 1
            if count == n:
                return asteroid_coords


def p1(field):
    return field.visible_count(field.monitoring_station())


def p2(field):
    coords = nth_asteroid(field, field.monitoring_station(), 200)
    return 100 * coords[0] + coords[1]


def run_tests():
    field = AsteroidField("""
        .#..#
        .....
        #####
        ....#
        ...##
    """)
    assert field.monitoring_station() == (3, 4)
    assert field.visible_count((3, 4)) == 8
    field = AsteroidField("""
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####
    """)
    assert field.monitoring_station() == (5, 8)
    assert field.visible_count((5, 8)) == 33
    field = AsteroidField("""
        .#....#####...#..
        ##...##.#####..##
        ##...#...#.#####.
        ..#.........###..
        ..#.#.....#....##
    """)
    assert nth_asteroid(field, (8, 3), 8) == (11, 2)


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data()
    field = AsteroidField(data)
    print("Part 1: {}".format(p1(field)))
    print("Part 2: {}".format(p2(field)))
