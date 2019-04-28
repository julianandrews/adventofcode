from itertools import islice

from utils import read_data

FACTOR_A = 16807
FACTOR_B = 48271
DIVISOR = 2147483647


def count_matches(generator_a, generator_b, number):
    mask = (1 << 16) - 1
    pairs = zip(generator_a, generator_b)

    return sum(
        1
        for (a, b) in islice(pairs, number)
        if a & mask == b & mask
    )


def count_simple_matches(a, b, number):
    generator_a = generator(FACTOR_A, a)
    generator_b = generator(FACTOR_B, b)

    return count_matches(generator_a, generator_b, number)


def count_picky_matches(a, b, number):
    generator_a = filter(lambda x: x % 4 == 0, generator(FACTOR_A, a))
    generator_b = filter(lambda x: x % 8 == 0, generator(FACTOR_B, b))

    return count_matches(generator_a, generator_b, number)


def generator(factor, value):
    while True:
        value = (value * factor) % DIVISOR
        yield value


def run_tests():
    assert list(islice(generator(FACTOR_A, 65), 5)) == [
        1092455, 1181022009, 245556042, 1744312007, 1352636452
    ]
    assert list(islice(generator(FACTOR_B, 8921), 5)) == [
        430625591, 1233683848, 1431495498, 137874439, 285222916
    ]
    assert count_simple_matches(65, 8921, 40_000_000) == 588
    assert count_picky_matches(65, 8921, 5_000_000) == 309


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(15)
    a, b = (int(line.split()[-1]) for line in data.strip().split("\n"))

    print("Part 1: {}".format(count_simple_matches(a, b, 40_000_000)))
    print("Part 2: {}".format(count_picky_matches(a, b, 5_000_000)))
