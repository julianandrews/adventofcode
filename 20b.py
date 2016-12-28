def least_unblocked(data, max_value):
    lines = [line.strip() for line in data.strip().split('\n')]
    ranges = sorted(tuple(map(int, line.split('-'))) for line in lines)
    lowest = max_value + 1
    highest = 0
    unblocked = 0
    for a, b in ranges:
        if a > highest + 1:
            unblocked += a - highest - 1
        lowest = min(lowest, a)
        highest = max(highest, b)
    unblocked += max_value - highest
    return unblocked

if __name__ == '__main__':
    assert least_unblocked(
        """
        5-8
        0-2
        4-7
        """
    , 9) == 2
    print("All tests passed")

    with open('data/d20.txt') as f:
        data = f.read()
    print(least_unblocked(data, 4294967295))
