import fileinput


def arithmetic_sum(start, end):
    n = end - start + 1
    a = start
    return n * (2 * a + n - 1) // 2


def index(row, column):
    return arithmetic_sum(0, column) + arithmetic_sum(column, column + row - 2)


def next_code(code):
    return (code * 252533) % 33554393


def code(row, column):
    code = 20151125
    for i in range(index(row, column) - 1):
        code = next_code(code)

    return code


def part1(row, column):
    return code(row, column)


if __name__ == "__main__":
    assert arithmetic_sum(0, 6) == 21
    assert arithmetic_sum(3, 5) == 12
    assert index(4, 3) == 18
    assert index(4, 1) == 7
    assert index(2, 5) == 20
    assert next_code(20151125) == 31916031
    assert code(1, 1) == 20151125
    assert code(5, 2) == 17552253
    assert code(2, 6) == 4041754
    print("All tests passed")

    words = next(fileinput.input()).split()
    row, column = int(words[15][:-1]), int(words[17][:-1])
    print("Part 1: %s" % part1(row, column))
    print("All Done!")
