import itertools
import numpy

from utils import read_data


def pattern(position):
    i = 0
    while True:
        for _ in range(position + 1):
            yield (0, 1, 0, -1)[i]
        i = (i + 1) % 4


def pattern_matrix(n):
    patterns = [pattern(i) for i in range(n)]
    for p in patterns:
        next(p)
    return numpy.stack(
        [numpy.fromiter(p, int, count=n) for p in patterns]
    )


def fft(input_list, num_phases):
    arr = numpy.array(input_list)
    m = pattern_matrix(len(arr))
    for _ in range(num_phases):
        arr = numpy.abs(m.dot(arr)) % 10
    return arr.tolist()


def fft_last_half_digits(last_n, num_phases):
    for _ in range(num_phases):
        for i in range(len(last_n) - 2, -1, -1):
            last_n[i] = (last_n[i + 1] + last_n[i]) % 10
    return last_n


def p1(input_list):
    return "".join(str(d) for d in fft(input_list, 100)[:8])


def p2(input_list):
    new_length = len(input_list) * 10000
    message_offset = int("".join(str(d) for d in input_list[:7]))
    if message_offset <= new_length // 2:
        raise ValueError("Message offset too close to start of list")
    n = new_length - message_offset
    start = message_offset % len(input_list)
    last_n = list(itertools.islice(itertools.cycle(input_list), start, start + n))
    digits = fft_last_half_digits(last_n, 100)[:8]
    return "".join(str(d) for d in digits)


def run_tests():
    assert fft([1, 2, 3, 4, 5, 6, 7, 8], 4) == [0, 1, 0, 2, 9, 4, 9, 8]

    input_list_1 = [int(c) for c in "80871224585914546619083218645595"]
    assert fft(input_list_1, 100)[:8] == [2, 4, 1, 7, 6, 1, 7, 6]
    assert fft(input_list_1, 100)[-10:] == fft_last_half_digits(input_list_1[-10:], 100)

    input_list_2 = [int(c) for c in "19617804207202209144916044189917"]
    assert fft(input_list_2, 100)[:8] == [7, 3, 7, 4, 5, 4, 1, 8]
    assert fft(input_list_2, 100)[-10:] == fft_last_half_digits(input_list_2[-10:], 100)

    input_list_3 = [int(c) for c in "69317163492948606335995924319873"]
    assert fft(input_list_3, 100)[:8] == [5, 2, 4, 3, 2, 1, 3, 3]
    assert fft(input_list_3, 100)[-10:] == fft_last_half_digits(input_list_3[-10:], 100)


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(16)
    input_list = [int(c) for c in data.strip()]
    print("Part 1: {}".format(p1(input_list)))
    print("Part 2: {}".format(p2(input_list)))
