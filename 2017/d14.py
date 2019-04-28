from utils import read_data
from utils.knot_hash import KnotHasher
from utils.graphs import connected_components


def used_squares(key):
    for y in range(128):
        hasher = KnotHasher()
        hasher.update("%s-%s" % (key, y))
        hash_value = hasher.dense_hash()
        row = bin(hash_value)[2:].zfill(128)
        for x, value in enumerate(row):
            if value == "1":
                yield (x, y)


def used_count(key):
    return len(list(used_squares(key)))


def region_count(key):
    nodes = set(used_squares(key))

    def neighbors(node):
        x, y = node
        for neighbor in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]:
            if neighbor in nodes:
                yield neighbor

    return len(list(connected_components(nodes, neighbors)))


def run_tests():
    key = "flqrgnkx"
    assert used_count(key) == 8108
    assert region_count(key) == 1242


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    key = read_data(14).strip()
    print("Part 1: {}".format(used_count(key)))
    print("Part 2: {}".format(region_count(key)))
