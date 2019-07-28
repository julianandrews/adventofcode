import fileinput
import itertools


def look_and_say(value):
    groups = ["%s%s" % (len(list(g)), k) for k, g in itertools.groupby(value)]
    return "".join(groups)


def part1(value):
    for i in range(40):
        value = look_and_say(value)

    return len(value)


def part2(value):
    for i in range(50):
        value = look_and_say(value)

    return len(value)


if __name__ == "__main__":
    data = next(fileinput.input()).strip()

    print("Part 1: %s" % part1(data))
    print("Part 2: %s" % part2(data))
