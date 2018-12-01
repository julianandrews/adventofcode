from utils import read_data, lines

from collections import Counter


def error_correct(data):
    return ''.join(Counter(x).most_common(1)[0][0] for x in zip(*lines(data)))


def inverted_error_correct(data):
    counters = [Counter(x) for x in zip(*lines(data))]
    inverted = [Counter(()) for c  in counters]
    for a, b in zip(counters, inverted):
        b.subtract(a)
    return ''.join(c.most_common(1)[0][0] for c in inverted)


if __name__ == '__main__':
    data = read_data(6)

    test_input = """
        eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar
    """
    assert error_correct(test_input) == 'easter'
    assert inverted_error_correct(test_input) == 'advent'
    print('All tests passed')

    print(error_correct(data))
    print(inverted_error_correct(data))
