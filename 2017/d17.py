import collections

from utils import read_data


def next_after_insert(ticks, jump):
    spinlock = collections.deque()
    for i in range(ticks + 1):
        spinlock.rotate(-jump)
        spinlock.append(i)
    return spinlock[0]


def next_after_zero(ticks, jump):
    value = 0
    position = 0
    for i in range(1, ticks + 1):
        position = (position + jump) % i + 1
        if position == 1:
            value = i

    return value


def run_tests():
    assert next_after_insert(2017, 3) == 638
    assert next_after_zero(2017, 3) == 1226


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(17)
    jump = int(data.strip())
    print("Part 1: {}".format(next_after_insert(2017, jump)))
    print("Part 2: {}".format(next_after_zero(50_000_000, jump)))
