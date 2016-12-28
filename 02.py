from utils import read_data, lines


OFFSETS = {'U': (0, -1), 'D': (0, 1), 'L': (-1, 0), 'R': (1, 0)}
VALUES = {
    'grid': [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
    'diamond': [
        [None, None, '1', None, None],
        [None, '2', '3', '4', None],
        ['5', '6', '7', '8', '9'],
        [None, 'A', 'B', 'C', None],
        [None, None, 'D', None, None],
    ],
}


def clamp(x, max_val):
    return max(min(x, max_val), 0)


def walk_grid(data):
    position = (1, 1)
    digits = ''
    for row in lines(data):
        for command in row.strip():
            position = tuple(clamp(a + b, 2) for a, b in zip(position, OFFSETS[command]))
        digits += str(VALUES['grid'][position[1]][position[0]])
    return digits


def walk_diamond(data):
    position = (0, 2)
    digits = ''
    for row in lines(data):
        for command in row.strip():
            new_position = tuple(clamp(a + b, 4) for a, b in zip(position, OFFSETS[command]))
            if VALUES['diamond'][new_position[1]][new_position[0]] is not None:
                position = new_position
        digits += VALUES['diamond'][position[1]][position[0]]
    return digits


if __name__ == '__main__':
    data = read_data(2)

    assert walk_grid('ULL\nRRDDD\nLURDL\nUUUUD') == '1985'
    assert walk_diamond('ULL\nRRDDD\nLURDL\nUUUUD') == '5DB3'
    print('All tests passed')

    print(walk_grid(data))
    print(walk_diamond(data))
