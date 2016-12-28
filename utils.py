def read_data(n):
    with open('data/d{}.txt'.format(n)) as f:
        data = f.read()
    return data


def lines(data):
    return [line.strip() for line in data.strip().split('\n')]
