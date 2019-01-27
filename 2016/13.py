from utils import bfs, lines, read_data


def is_open(location, seed):
    x, y = location
    value = x * x + 3 * x + 2 * x * y + y + y * y + seed
    parity = sum((value >> i) & 1 for i in range(value.bit_length())) % 2
    return parity == 0


def draw_map(max_x, max_y, seed):
    return '\n'.join(
        ''.join('.' if is_open((x, y), seed) else '#' for x in range(max_x))
        for y in range(max_y)
    )


def cubicle_bfs(seed):
    def get_neighbors(location):
        x, y = location
        for neighbor in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
            if neighbor[0] >= 0 and neighbor[1] >= 0 and is_open(neighbor, seed):
                yield neighbor

    return bfs((1, 1), get_neighbors)


def shortest_path(destination, seed):
    for node in cubicle_bfs(seed):
        if node.value == destination:
            return node.depth
    return None


def seen_in_steps(steps, seed):
    seen = set()
    for node in cubicle_bfs(seed):
        if node.depth > steps:
            break
        else:
            seen.add(node.value)
    return len(seen)


if __name__ == '__main__':
    assert draw_map(10, 7, 10) == '\n'.join(lines(
        """
        .#.####.##
        ..#..#...#
        #....##...
        ###.#.###.
        .##..#..#.
        ..##....#.
        #...##.###
        """
    ))
    assert shortest_path((7, 4), seed=10) == 11
    print('All tests passed')

    favorite_number = int(read_data(13))

    print(shortest_path((31, 39), favorite_number))
    print(seen_in_steps(50, favorite_number))
