def read_data(n):
    with open('inputs/d{:02d}/input.txt'.format(n)) as f:
        return f.read()


def get_lines(data):
    return [line.strip() for line in data.strip().split('\n')]


def manhattan_distance(a, b):
    return sum(abs(u - v) for u, v in zip(a, b))
