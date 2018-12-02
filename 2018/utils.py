def readstrings(filename):
    with open(filename) as f:
        return [line.strip() for line in f.readlines()]


def readints(filename):
    return [int(x) for x in readstrings(filename)]
