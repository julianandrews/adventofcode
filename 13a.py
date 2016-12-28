try:
    from queue import Queue
except ImportError:
    from Queue import Queue


SEED = 1364


def is_open(x, y, seed=SEED):
    value = x * x + 3 * x + 2 * x * y + y + y * y + seed
    parity = sum((value >> i) & 1 for i in range(value.bit_length())) % 2
    return parity == 0


def draw_sample_map():
    return '\n'.join(
        ''.join('.' if is_open(x, y, 10) else '#' for x in range(10)) for y in range(7)
    )


def doit(destination, seed=SEED):
    queue = Queue()
    seen = set((1, 1))
    parents = {}
    queue.put((1, 1))

    while not queue.empty():
        x, y = location = queue.get()
        if location == destination:
            distance = 0
            while location != (1, 1):
                location = parents[location]
                distance += 1
            return distance

        for neighbor in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
            if neighbor not in seen and is_open(neighbor[0], neighbor[1], seed):
                queue.put(neighbor)
                parents[neighbor] = location
                seen.add(neighbor)

    return None


if __name__ == '__main__':
    assert draw_sample_map() == """
.#.####.##
..#..#...#
#....##...
###.#.###.
.##..#..#.
..##....#.
#...##.###
    """.strip()
    assert doit((7, 4), seed=10) == 11
    print('All tests passed')

    print(doit((31, 39)))
