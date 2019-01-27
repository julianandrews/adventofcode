from utils import read_data


def winner(n, opposite=False):
    value = 0
    for i in range(2, n):
        value = (value + 1) % i
        if (not opposite and value >= 1) or (opposite and value >= (i + 1) // 2):
            value += 1
    return value + 1


if __name__ == '__main__':
    assert winner(5) == 3
    assert winner(5, opposite=True) == 2
    print('All tests passed')

    num_elves = int(read_data(19))

    print(winner(num_elves))
    print(winner(num_elves, opposite=True))
