def winner(n):
    value = 0
    for i in range(2, n):
        value = (value + 1) % i
        if value >= (i + 1) // 2:
            value += 1
    return value


def doit(n):
    return winner(n) + 1


if __name__ == '__main__':
    assert doit(1) == 1
    assert doit(2) == 1
    assert doit(3) == 3
    assert doit(4) == 1
    assert doit(5) == 2
    print('All tests passed')

    print(doit(3014387))
