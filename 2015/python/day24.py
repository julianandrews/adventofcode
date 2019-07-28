import fileinput
import functools


def subsets_with_sum(items, total):
    items = sorted(items)

    @functools.lru_cache(maxsize=None)
    def q(i, s):
        if i == 0:
            return items[i] == s
        elif s == 0:
            return True
        else:
            return items[i] == s or q(i - 1, s - items[i]) or q(i - 1, s)

    def p(i, s, r):
        if s == 0:
            yield r.copy()
        elif i == 0:
            if items[0] == s:
                x = r.copy()
                x.add(s)
                yield x
        else:
            if q(i - 1, s):
                yield from p(i - 1, s, r.copy())
            if s >= items[i] and q(i - 1, s - items[i]):
                x = r.copy()
                x.add(items[i])
                yield from p(i - 1, s - items[i], x)

    yield from p(len(items) - 1, total, set())


def entanglement(weights):
    return functools.reduce(int.__mul__, weights, 1)


def part1(weights):
    total = sum(weights) // 3
    possible_subsets = sorted(
        subsets_with_sum(weights, total),
        key=lambda s: (len(s), entanglement(s))
    )
    for a in possible_subsets:
        for b in subsets_with_sum(set(weights) - a, total):
            return entanglement(a)


def part2(weights):
    total = sum(weights) // 4
    possible_subsets = sorted(
        subsets_with_sum(weights, total),
        key=lambda s: (len(s), entanglement(s))
    )
    for a in possible_subsets:
        remaining = set(weights) - a
        for b in subsets_with_sum(remaining, total):
            for c in subsets_with_sum(remaining - b, total):
                return entanglement(a)


if __name__ == "__main__":
    test_weights = list(range(1, 6)) + list(range(7, 12))
    assert part1(test_weights) == 99
    assert part2(test_weights) == 44
    print("All tests passed")

    lines = [line.strip() for line in fileinput.input()]
    weights = [int(line) for line in lines]

    print("Part 1: %s" % part1(weights))
    print("Part 2: %s" % part2(weights))
