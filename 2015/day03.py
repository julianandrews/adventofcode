import itertools
import fileinput


class Santa:
    DIRECTIONS = {"^": (0, 1), ">": (1, 0), "<": (-1, 0), "v": (0, -1)}

    def __init__(self):
        self.position = (0, 0)

    def move(self, direction):
        self.position = tuple(map(
            sum,
            zip(self.position, self.DIRECTIONS[direction])
        ))


def find_visited(directions, santas):
    visited = {(0, 0)}

    for (direction, santa) in zip(data, itertools.cycle(santas)):
        santa.move(direction)
        visited.add(santa.position)

    return visited


def part1(data):
    visited = find_visited(data, [Santa()])

    return len(visited)


def part2(data):
    visited = find_visited(data, [Santa(), Santa()])

    return len(visited)


if __name__ == "__main__":
    data = next(fileinput.input()).strip()

    print("Part 1: %s" % part1(data))
    print("Part 2: %s" % part2(data))
