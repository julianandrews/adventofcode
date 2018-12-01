from utils import read_data


def move(orientation, location, command):
    turn_sign = 1 if command[0] == 'R' else -1
    orientation = (orientation + turn_sign) % 4
    distance = int(command[1:])
    if orientation == 0:
        return (orientation, (location[0], location[1] + distance))
    elif orientation == 1:
        return (orientation, (location[0] + distance, location[1]))
    elif orientation == 2:
        return (orientation, (location[0], location[1] - distance))
    elif orientation == 3:
        return (orientation, (location[0] - distance, location[1]))


def find_distance(data):
    commands = data.split(', ')
    orientation = 0
    location = (0, 0)
    for command in commands:
        orientation, location = move(orientation, location, command)
    return abs(location[0]) + abs(location[1])


def find_first_repeat(data):
    commands = data.split(', ')
    orientation = 0
    location = (0, 0)
    visited = set([location])
    for command in commands:
        move(orientation, location, command)
        turn_sign = 1 if command[0] == 'R' else -1
        orientation = (orientation + turn_sign) % 4
        distance = int(command[1:])
        for i in range(distance):
            if orientation == 0:
                location = (location[0], location[1] + 1)
            elif orientation == 1:
                location = (location[0] + 1, location[1])
            elif orientation == 2:
                location = (location[0], location[1] - 1)
            elif orientation == 3:
                location = (location[0] - 1, location[1])
            if location in visited:
                return abs(location[0]) + abs(location[1])
            visited.add(location)


if __name__ == '__main__':
    data = read_data(1)

    assert find_distance("R2, L3") == 5
    assert find_distance("R2, R2, R2") == 2
    assert find_distance("R5, L5, R5, R3") == 12
    assert find_first_repeat("R8, R4, R4, R8") == 4
    print('All tests passed')

    print(find_distance(data))
    print(find_first_repeat(data))
