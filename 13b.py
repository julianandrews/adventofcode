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


def doit(seed=SEED):
    queue = Queue()
    seen = set()
    seen.add((1, 1))
    queue.put(((1, 1), 0))

    while not queue.empty():
        location, depth = queue.get()
        x, y = location

        if depth < 50:
            for u, v in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                if (u, v) not in seen and u >= 0 and v >= 0 and is_open(u, v, seed):
                    queue.put(((u, v), depth + 1))
                    seen.add((u, v))

    return len(seen)


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
    print('All tests passed')

    print(doit())
