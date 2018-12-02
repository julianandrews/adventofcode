import collections

import utils


def p1(data):
    two_counts = 0
    three_counts = 0
    for line in data:
        counts = set(collections.Counter(line).values())
        if 2 in counts:
            two_counts += 1
        if 3 in counts:
            three_counts += 1
    return two_counts * three_counts


def p2(data):
    for i, first in enumerate(data[:-1]):
        for second in data[i + 1:]:
            shared = [a for a, b in zip(first, second) if a == b]
            if len(shared) == len(first) - 1:
                return "".join(shared)


if __name__ == "__main__":
    data = utils.readstrings("data/input-02.txt")

    print("Part 1: %d" % p1(data))
    print("Part 2: %s" % p2(data))
