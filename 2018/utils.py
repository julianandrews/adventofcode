def readlines(filename):
    with open(filename) as f:
        return f.readlines()


def readints(filename):
    data = readlines(filename)
    return (int(line.strip()) for line in data)
