import functools
import math

from utils import read_data


def transit_steps(n):
    # largest odd-square strictly smaller than n
    s = max(((int(math.sqrt(n - 1)) + 1) // 2) * 2 - 1, 0)
    side_ix = (n - s**2 - 1) % (4 * (s + 2) - 4)
    remainder = side_ix % (s + 1)
    edge_steps = abs(s // 2 - remainder)
    return (s + 1) // 2 + edge_steps


def neighbors(x, y):
    for z in (x - 1, x, x + 1):
        for w in (y - 1, y, y + 1):
            if (x, y) != (z, w):
                yield (z, w)


def values_written():
    grid = {}
    x, y = 0, 0
    dx, dy = 0, -1
    grid[(x, y)] = 1
    while True:
        n_dx, n_dy = -dy, dx  # velocity left of current direction
        if (x + n_dx, y + n_dy) not in grid:
            dx, dy = n_dx, n_dy
        x, y = x + dx, y + dy
        grid[(x, y)] = sum(grid.get(point, 0) for point in neighbors(x, y))
        yield grid[(x, y)]


def first_greater_than(n):
    for value in values_written():
        if value > n:
            return value


def run_tests():
    assert transit_steps(1) == 0
    assert transit_steps(12) == 3
    assert transit_steps(23) == 2
    assert transit_steps(1024) == 31
    assert first_greater_than(302) == 304
    assert first_greater_than(363) == 747


if __name__ == "__main__":
    data = int(read_data(3).strip())
    run_tests()
    print("All tests passed")
    print("Part 1: {}".format(transit_steps(data)))
    print("Part 2: {}".format(first_greater_than(data)))
