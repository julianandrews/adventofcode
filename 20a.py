def least_unblocked(data):
    lines = [line.strip() for line in data.strip().split('\n')]
    ranges = sorted(tuple(map(int, line.split('-'))) for line in lines)
    lowest = 4294967296
    highest = 0
    for a, b in ranges:
        if a > highest + 1:
            return highest + 1
        lowest = min(lowest, a)
        highest = max(highest, b)


if __name__ == '__main__':
    assert least_unblocked(
        """
        5-8
        0-2
        4-7
        """
    ) == 3
    print("All tests passed")

    with open('data/d20.txt') as f:
        data = f.read()
    print(least_unblocked(data))
