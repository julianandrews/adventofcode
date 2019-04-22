import functools

from utils import read_data

HASH_ROUNDS = 64
STANDARD_LIST_SIZE = 256


class HashState:
    def __init__(self, size):
        self.values = list(range(size))
        self.position = 0
        self.skip_size = 0

    def evolve(self, length):
        for n in range(length // 2):
            i = (n + self.position) % len(self.values)
            j = (self.position + length - n - 1) % len(self.values)
            old_value = self.values[j]
            self.values[j] = self.values[i]
            self.values[i] = old_value
        self.position = (self.position + length + self.skip_size) % len(self.values)
        self.skip_size += 1

    def dense_hash(self):
        blocks = [self.values[i:i + 16] for i in range(0, len(self.values), 16)]
        dense_hash = [functools.reduce(lambda a, b: a ^ b, block) for block in blocks]

        return "".join(hex(n)[2:].zfill(2) for n in dense_hash)


def ascii_lengths(data):
    return [ord(c) for c in data] + [17, 31, 73, 47, 23]


def complex_hash(data):
    lengths = ascii_lengths(data)
    state = HashState(STANDARD_LIST_SIZE)
    for i in range(HASH_ROUNDS):
        for length in lengths:
            state.evolve(length)

    return state.dense_hash()


def simple_hash(lengths, size=STANDARD_LIST_SIZE):
    state = HashState(size)
    for length in lengths:
        state.evolve(length)

    return state.values[0] * state.values[1]


def run_tests():
    assert simple_hash([3, 4, 1, 5], 5) == 12
    assert ascii_lengths("1,2,3") == [49, 44, 50, 44, 51, 17, 31, 73, 47, 23]
    assert complex_hash("") == "a2582a3a0e66e6e86e3812dcb672a272"
    assert complex_hash("AoC 2017") == "33efeb34ea91902bb2f59c9920caa6cd"
    assert complex_hash("1,2,3") == "3efbe78a8d82f29979031a4aa0b16a9d"
    assert complex_hash("1,2,4") == "63960835bcdc130f0b66d7ff4f6a5a8e"


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(10).strip()
    lengths = [int(x) for x in data.split(",")]
    print("Part 1: {}".format(simple_hash(lengths)))
    print("Part 2: {}".format(complex_hash(data)))
