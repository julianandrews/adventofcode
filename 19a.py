memo = {1: 0}


def winner(n):
    if memo.get(n) is not None:
        return memo[n]

    if n % 2 == 0:
        value = 2 * winner(n // 2)
    else:
        value = 2 * (winner(n // 2) + 1)

    memo[n] = value
    return value


def doit(n):
    return winner(n) + 1


if __name__ == '__main__':
    assert doit(1) == 1
    assert doit(2) == 1
    assert doit(3) == 3
    assert doit(4) == 1
    assert doit(5) == 3
    print('All tests passed')

    print(doit(3014387))
