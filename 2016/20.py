from utils import lines, read_data


def unblocked(data, max_value):
    ranges = sorted(tuple(map(int, line.split('-'))) for line in lines(data))
    ranges.append((max_value + 1, max_value + 1))

    i = 0
    for bottom, top in ranges:
        yield from range(i, bottom)
        i = max(i, top + 1)


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

    print(next(unblocked(data, 4294967295)))
    print(len(list(unblocked(data, 4294967295))))
