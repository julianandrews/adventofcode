from utils import read_data


def is_simple_candidate(number):
    adjacent_pair = False

    digits = [int(c) for c in str(number)]
    for d1, d2 in zip(digits, digits[1:]):
        if d2 < d1:
            return False
        if d1 == d2:
            adjacent_pair = True

    return adjacent_pair


def is_candidate(number):
    run_length = 1
    adjacent_pair = False

    digits = [int(c) for c in str(number)]
    for d1, d2 in zip(digits, digits[1:]):
        if d2 < d1:
            return False
        if d1 == d2:
            run_length += 1
        else:
            if run_length == 2:
                adjacent_pair = True
            run_length = 1

    return adjacent_pair or run_length == 2


def p1(start, end):
    return len([x for x in range(start, end + 1) if is_simple_candidate(x)])


def p2(start, end):
    return len([x for x in range(start, end + 1) if is_candidate(x)])


def run_tests():
    assert is_simple_candidate(111111)
    assert not is_simple_candidate(23450)
    assert not is_simple_candidate(123789)
    assert is_candidate(112233)
    assert not is_candidate(123444)
    assert is_candidate(111122)


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data()
    start, end = (int(x) for x in data.strip().split("-"))
    print("Part 1: {}".format(p1(start, end)))
    print("Part 2: {}".format(p2(start, end)))
