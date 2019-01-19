import collections
import fileinput
import itertools


def happy_sum(people, happy_map):
    neighbors = itertools.cycle(people)
    next(neighbors)
    return sum(
        happy_map[a][b] + happy_map[b][a]
        for a, b in zip(people, neighbors)
    )


def part1(happy_map):
    return max(
        happy_sum(people, happy_map)
        for people in itertools.permutations(happy_map.keys())
    )


def part2(happy_map):
    happy_map["Me"] = {}
    for person in happy_map.keys():
        happy_map["Me"][person] = 0
        happy_map[person]["Me"] = 0
    return max(
        happy_sum(people, happy_map)
        for people in itertools.permutations(happy_map.keys())
    )



if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]

    happy_map = collections.defaultdict(dict)
    for line in lines:
        words = line[:-1].split()
        happy_map[words[0]][words[-1]] = (1 if words[2] == "gain" else -1) * int(words[3])

    print("Part 1: %s" % part1(happy_map))
    print("Part 2: %s" % part2(happy_map))
