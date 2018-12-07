import itertools


def readstrings(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]


def readints(filename):
    return [int(x) for x in readstrings(filename)]


def pairwise(iterable):
    a, b = itertools.tee(iterable)
    next(b, None)
    return zip(a, b)
