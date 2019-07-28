import fileinput


def part1(data):
    floor = 0
    for c in data:
        floor += 1 if c == "(" else -1

    return floor


def part2(data):
    floor = 0
    for i, c in enumerate(data):
        floor += 1 if c == "(" else -1
        if floor == -1:
            return i + 1

    return


if __name__ == "__main__":
    data = next(fileinput.input()).strip()

    print("Part 1: %s" % part1(data))
    print("Part 2: %s" % part2(data))
