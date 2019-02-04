from utils import get_lines, read_data


def jumps_to_exit(jumps):
    jumps = jumps[:]
    i = 0
    steps = 0
    while 0 <= i < len(jumps):
        jumps[i] += 1
        i += jumps[i] - 1
        steps += 1

    return steps


def strange_jumps_to_exit(jumps):
    jumps = jumps[:]
    i = 0
    steps = 0
    while 0 <= i < len(jumps):
        jump = jumps[i]
        jumps[i] += 1 if jump < 3 else -1
        i += jump
        steps += 1

    return steps


def run_tests():
    assert jumps_to_exit([0, 3, 0, 1, -3]) == 5
    assert strange_jumps_to_exit([0, 3, 0, 1, -3]) == 10


if __name__ == "__main__":
    jumps = [int(line) for line in get_lines(read_data(5))]
    run_tests()
    print("All tests passed")
    print("Part 1: {}".format(jumps_to_exit(jumps)))
    print("Part 2: {}".format(strange_jumps_to_exit(jumps)))
