from utils import read_data
from utils.primes import prime_factors


class Dance:
    def __init__(self, size):
        """A dance can be represented as a permutation of the indices and
        of the dancers themselves. Since the two types of permutations commute,
        we can track them separately and apply them in any order."""
        self.dancer_permutation = list(range(size))
        self.index_permutation = list(range(size))

    def dance(self, dance_steps):
        dancer_steps = [d for d in dance_steps if d[0] == "p"]
        index_steps = [d for d in dance_steps if d[0] in {"x", "s"}]

        for dance_step in dancer_steps:
            dancer_a, dancer_b = dance_step[1:].split("/")
            position_index_a = int(ord(dancer_a) - ord('a'))
            position_index_b = int(ord(dancer_b) - ord('a'))

            old_position_a = self.dancer_permutation[position_index_a]
            old_position_b = self.dancer_permutation[position_index_b]
            self.dancer_permutation[position_index_a] = old_position_b
            self.dancer_permutation[position_index_b] = old_position_a

        index = 0
        for dance_step in index_steps:
            dance_type = dance_step[0]
            dance_args = dance_step[1:].split("/")
            if dance_type == "s":
                index = (index - int(dance_args[0])) % len(self.index_permutation)
            elif dance_type == "x":
                a, b = map(int, dance_args)
                index_a = (index + a) % len(self.index_permutation)
                index_b = (index + b) % len(self.index_permutation)

                old_dancer_a = self.index_permutation[index_a]
                old_dancer_b = self.index_permutation[index_b]
                self.index_permutation[index_a] = old_dancer_b
                self.index_permutation[index_b] = old_dancer_a
        self.index_permutation = self.index_permutation[index:] + self.index_permutation[:index]

    def dancers(self):
        # Transform to position space
        positions = [0] * len(self.index_permutation)
        for i, v in enumerate(self.index_permutation):
            positions[v] = i

        # Apply the permutation
        positions = [positions[i] for i in self.dancer_permutation]

        # Transform back
        dancers = [0] * len(self.index_permutation)
        for i, v in enumerate(positions):
            dancers[v] = i

        # Map the integers to characters
        return "".join(chr(dancer + ord('a')) for dancer in dancers)

    def __add__(self, other):
        if len(self.index_permutation) != len(other.index_permutation):
            raise ValueError("Inconsistent dance sizes")
        result = Dance(len(self.index_permutation))
        result.index_permutation = [
            self.index_permutation[i] for i in other.index_permutation
        ]
        result.dancer_permutation = [
            self.dancer_permutation[i] for i in other.dancer_permutation
        ]

        return result

    def __mul__(self, n):
        result = self
        for i in range(n - 1):
            result += self

        return result

    def __rmul__(self, n):
        return self * n


def dance(dance_steps, times=1, size=16):
    dance = Dance(size)
    dance.dance(dance_steps)

    for n in prime_factors(times):
        dance = n * dance

    return dance.dancers()


def run_tests():
    dance_steps = ["s1", "x3/4", "pe/b"]
    assert dance(dance_steps, size=5) == "baedc"
    assert dance(dance_steps, times=2, size=5) == "ceadb"


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(16)
    dance_steps = data.strip().split(",")

    print("Part 1: {}".format(dance(dance_steps)))
    print("Part 2: {}".format(dance(dance_steps, times=1_000_000_000)))
