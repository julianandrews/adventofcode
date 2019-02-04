import itertools

from utils import read_data, get_lines


def checksum(data):
    return sum(max(row) - min(row) for row in data)


def fancy_checksum(data):
    total = 0
    for row in data:
        for pair in itertools.combinations(row, 2):
            a, b = sorted(pair)
            if b % a == 0:
                total += b // a

    return total


def run_tests():
    assert checksum([
        [5, 1, 9, 5],
        [7, 5, 3],
        [2, 4, 6, 8],
    ]) == 18
    assert fancy_checksum([
        [5, 9, 2, 8],
        [9, 4, 7, 3],
        [3, 8, 6, 5],
    ]) == 9


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = [
        [int(cell) for cell in line.split()]
        for line in get_lines(read_data(2))
    ]
    print("Part 1: {}".format(checksum(data)))
    print("Part 2: {}".format(fancy_checksum(data)))
