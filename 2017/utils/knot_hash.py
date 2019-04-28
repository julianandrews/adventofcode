import functools


class KnotHasher:
    HASH_ROUNDS = 64
    EXTRA_LENGTHS = [17, 31, 73, 47, 23]

    def __init__(self, size=256):
        self.values = list(range(size))
        self.position = 0
        self.skip_size = 0

    def update(self, data):
        lengths = self.ascii_lengths(data)
        for _ in range(self.HASH_ROUNDS):
            for length in lengths:
                self.evolve(length)

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
        blocks = [
            self.values[i:i + 16]
            for i in range(0, len(self.values), 16)
        ]
        dense_hash = 0
        for i, block in enumerate(reversed(blocks)):
            dense_hash |= functools.reduce(lambda a, b: a ^ b, block) << (i * 8)

        return dense_hash

    @classmethod
    def ascii_lengths(cls, data):
        return [ord(c) for c in data] + cls.EXTRA_LENGTHS
