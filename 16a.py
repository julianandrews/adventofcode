def fill_data(size, a):
    b = ''.join('1' if c == '0' else '0' for c in reversed(list(a)))
    result = "{}0{}".format(a, b)
    if len(result) < size:
        result = fill_data(size, result)
    return result[:size]


def checksum(data):
    result = ''.join('1' if data[i] == data[i + 1] else '0' for i in range(0, len(data), 2))
    if len(result) % 2 == 0:
        result = checksum(result)
    return result


def doit(size, seed):
    data = fill_data(size, seed)
    return checksum(data)


if __name__ == '__main__':
    assert checksum('110010110100') == '100'
    assert doit(20, '10000') == '01100'
    print("All tests passed")

    print(doit(272, '10001110011110000'))
    print(doit(35651584, '10001110011110000'))
