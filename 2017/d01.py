from utils import read_data


def checksum(digits):
    result = 0
    for i, digit in enumerate(digits):
        if digit == digits[(i + 1) % len(digits)]:
            result += digit

    return result


def fancy_checksum(digits):
    assert len(digits) % 2 == 0

    result = 0
    for i, digit in enumerate(digits):
        if digit == digits[(i + len(digits) // 2) % len(digits)]:
            result += digit

    return result


def run_tests():
    assert checksum([1, 1, 2, 2]) == 3
    assert checksum([1, 1, 1, 1]) == 4
    assert checksum([1, 2, 3, 4]) == 0
    assert checksum([9, 1, 2, 1, 2, 1, 2, 9]) == 9
    assert fancy_checksum([1, 2, 1, 2]) == 6
    assert fancy_checksum([1, 2, 2, 1]) == 0
    assert fancy_checksum([1, 2, 3, 4, 2, 5]) == 4
    assert fancy_checksum([1, 2, 3, 1, 2, 3]) == 12
    assert fancy_checksum([1, 2, 1, 3, 1, 4, 1, 5]) == 4


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    digits = [int(c) for c in read_data(1).strip()]
    print("Part 1: {}".format(checksum(digits)))
    print("Part 2: {}".format(fancy_checksum(digits)))
