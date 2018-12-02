import itertools

import utils


def p1(data):
    return sum(data)


def p2(data):
    seen = set()
    for frequency in itertools.accumulate(itertools.cycle(data)):
        if frequency in seen:
            return frequency
        seen.add(frequency)


if __name__ == "__main__":
    data = utils.readints("data/input-01.txt")
    print("Part 1: %d" % p1(data))
    print("Part 2: %d" % p2(data))

