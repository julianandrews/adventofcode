import collections
import fileinput
import itertools
import math


def get_trip_lengths(distances):
    trip_lengths = []
    for cities in itertools.permutations(distances.keys()):
        trip_lengths.append(
            sum(distances[a][b] for a, b in zip(cities, cities[1:]))
        )

    return trip_lengths


def part1(distances):
    trip_lengths = get_trip_lengths(distances)
    return min(trip_lengths)


def part2(distances):
    trip_lengths = get_trip_lengths(distances)
    return max(trip_lengths)


def parse_line(line):
    words = line.split()
    return (words[0], words[2], int(words[4]))


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    pairs = [parse_line(line) for line in lines]
    distances = collections.defaultdict(dict)
    for a, b, d in pairs:
        distances[a][b] = d
        distances[b][a] = d

    print("Part 1: %s" % part1(distances))
    print("Part 2: %s" % part2(distances))
