from utils import lines, read_data


def unblocked(data, max_value):
    lowest = max_value + 1
    highest = 0
    ranges = sorted(tuple(map(int, line.split('-'))) for line in lines(data))
    ranges.append((max_value + 1, max_value + 1))

    for a, b in ranges:
        if a > highest + 1:
            for x in range(highest + 1, a):
                yield x
        lowest = min(lowest, a)
        highest = max(highest, b)


if __name__ == '__main__':
    data = read_data(20)

    assert list(unblocked(
        """
        5-8
        0-2
        4-7
        """,
        9,
    )) == [3, 9]
    print("All tests passed")

    print(unblocked(data, 4294967295).next())
    print(len(list(unblocked(data, 4294967295))))
