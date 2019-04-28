from utils import read_data
from utils.knot_hash import KnotHasher


def knot_hash(data):
    hasher = KnotHasher()
    hasher.update(data)

    return hex(hasher.dense_hash())[2:]


def simple_hash(lengths, size):
    hasher = KnotHasher(size)
    for length in lengths:
        hasher.evolve(length)

    return hasher.values[0] * hasher.values[1]


def run_tests():
    assert simple_hash([3, 4, 1, 5], 5) == 12
    assert KnotHasher.ascii_lengths("1,2,3") == [49, 44, 50, 44, 51, 17, 31, 73, 47, 23]
    assert knot_hash("") == "a2582a3a0e66e6e86e3812dcb672a272"
    assert knot_hash("AoC 2017") == "33efeb34ea91902bb2f59c9920caa6cd"
    assert knot_hash("1,2,3") == "3efbe78a8d82f29979031a4aa0b16a9d"
    assert knot_hash("1,2,4") == "63960835bcdc130f0b66d7ff4f6a5a8e"


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(10).strip()
    lengths = [int(x) for x in data.split(",")]
    print("Part 1: {}".format(simple_hash(lengths, 256)))
    print("Part 2: {}".format(knot_hash(data)))
