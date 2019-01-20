import collections
import fileinput
import functools
import itertools


Ingredient = collections.namedtuple(
    "Ingredient",
    ["name", "capacity", "durability", "flavor", "texture", "calories"]
)


def partitions(n, k, l=1):
    if k < 1:
        raise StopIteration
    if k == 1:
        if n >= l:
            yield (n,)
        raise StopIteration
    for i in range(l, n + 1):
        for result in partitions(n - i, k - 1, i):
            yield (i, ) + result


def property_totals(ingredients, parts):
    property_totals = [0] * 5
    for ingredient, count in zip(ingredients, parts):
        for i, value in enumerate(ingredient[1:]):
            property_totals[i] += value * count
    return [max(s, 0) for s in property_totals]


def total_score(ingredients, parts):
    partial_scores = property_totals(ingredients, parts)[:-1]
    return functools.reduce(int.__mul__, partial_scores, 1)


def part1(ingredients):
    scores = []
    for part in partitions(100, len(ingredients)):
        for perm in itertools.permutations(part):
            scores.append(total_score(ingredients, perm))

    return max(scores)


def part2(ingredients):
    scores = []
    for part in partitions(100, len(ingredients)):
        for perm in itertools.permutations(part):
            totals = property_totals(ingredients, perm)
            if totals[-1] == 500:
                scores.append(total_score(ingredients, perm))

    return max(scores)


def parse_line(line):
    cleaned = line.replace(',', '').replace(":", "")
    words = cleaned.split()
    return Ingredient(words[0], *map(int, words[2::2]))


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    ingredients = [parse_line(line) for line in lines]

    print("Part 1: %s" % part1(ingredients))
    print("Part 2: %s" % part2(ingredients))
