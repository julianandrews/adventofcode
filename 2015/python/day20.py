import fileinput
import math


def part1(n):
    target = math.ceil(n / 10.0)
    counts = [0] * (target + 1)

    for elf in range(1, target + 1):
        value = elf
        while value < target + 1:
            counts[value] += elf
            value += elf
    for i, count in enumerate(counts):
        if count > target:
            return i


def part2(n):
    target = math.ceil(n / 11.0)
    counts = [0] * (target + 1)
    for elf in range(1, target + 1):
        for multiplier in range(1, 51):
            value = multiplier * elf
            if value > target + 1:
                break
            counts[value] += elf
    for i, count in enumerate(counts):
        if count > target:
            return i


if __name__ == "__main__":
    n = int(next(fileinput.input()).strip())

    print("Part 1: %s" % part1(n))
    print("Part 2: %s" % part2(n))
