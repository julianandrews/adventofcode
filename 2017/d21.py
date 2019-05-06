import collections

from utils import read_data
from utils.math import integer_sqrt

STARTING_PATTERN = """.#...####"""


class Pattern:
    def __init__(self, size, value):
        self.size = size
        self.value = value

    def __hash__(self):
        return hash((self.size, self.value))

    def __eq__(self, other):
        return self.size == other.size and self.value == other.value

    def __repr__(self):
        return "Pattern(size=%s, value=%s)" % (self.size, self.value)

    def evolve(self, rules):
        subpatterns = [rules[pattern] for pattern in self.subpatterns()]
        return self.from_subpatterns(subpatterns)

    def subpatterns(self):
        subpattern_size = 2 if self.size % 2 == 0 else 3
        subpatterns = []
        for y in range(0, self.size, subpattern_size):
            for x in range(0, self.size, subpattern_size):
                mask = (1 << subpattern_size) - 1
                value = 0
                for j in range(subpattern_size):
                    offset = j * subpattern_size
                    value |= (self.value >> (x + self.size * (y + j)) & mask) << offset
                subpatterns.append(Pattern(subpattern_size, value))

        return list(reversed(subpatterns))

    def pixel_count(self):
        return bin(self.value).count("1")

    @classmethod
    def from_subpatterns(cls, patterns):
        subpattern_size = patterns[0].size
        foo = integer_sqrt(len(patterns))
        size = foo * subpattern_size
        mask = (1 << subpattern_size) - 1

        value = 0
        for j in range(foo):
            for i in range(foo):
                pattern = patterns[len(patterns) - i - foo * j - 1]
                for k in range(subpattern_size):
                    offset = (j * size + i) * subpattern_size + k * size
                    change = (pattern.value >> (k * subpattern_size)) & mask
                    value |= change << offset

        return Pattern(size, value)

    @classmethod
    def from_string(cls, s):
        size = integer_sqrt(len(s))
        return cls(size, int(s.replace("#", "1").replace(".", "0"), 2))


def rotate(s):
    size = integer_sqrt(len(s))
    rows = [list(s[i * size:i * size + size]) for i in range(size)]
    return "".join(rows[size - i - 1][j] for j in range(size) for i in range(size))


def flip(s):
    size = integer_sqrt(len(s))
    rows = [s[i * size:i * size + size] for i in range(size)]
    return "".join(reversed(rows))


def transformations(s):
    rotated = rotate(s)
    flipped = flip(s)
    rot_flipped = rotate(flipped)
    return [
        s, rotated, s[::-1], rotated[::-1],
        flipped, rot_flipped, flipped[::-1], rot_flipped[::-1]
    ]


def parse_rules(lines):
    rules = {}
    for line in lines:
        value, output = (x.replace("/", "").strip() for x in line.split(" => "))
        output_pattern = Pattern.from_string(output)
        for s in transformations(value):
            rules[Pattern.from_string(s)] = output_pattern

    return rules


def run(rules, time, pattern=None):
    pattern = pattern or Pattern.from_string(STARTING_PATTERN)

    for _ in range(time):
        pattern = pattern.evolve(rules)

    return pattern


def pixels_on_after(rules, time):
    subpattern_counts = collections.Counter([Pattern.from_string(STARTING_PATTERN)])

    three_step_cache = {}
    if time >= 3:
        for pattern in rules:
            three_step_cache[pattern] = collections.Counter(run(rules, 3, pattern).subpatterns())

    for _ in range(time // 3):
        new_subpattern_counts = collections.Counter()
        for pattern, pattern_count in subpattern_counts.items():
            new_subpattern_counts.update({
                p: three_step_cache[pattern][p] * pattern_count
                for p in three_step_cache[pattern]
            })
        subpattern_counts = new_subpattern_counts

    pixel_count = 0
    for pattern, pattern_count in subpattern_counts.items():
        result = run(rules, time % 3, pattern)
        pixel_count += result.pixel_count() * pattern_count

    return pixel_count


def run_tests():
    patterns = [Pattern(size=2, value=value) for value in (8, 4, 2, 1)]
    assert Pattern.from_string("#..#........#..#").subpatterns() == patterns
    assert Pattern.from_subpatterns(patterns) == Pattern.from_string("#..#........#..#")

    patterns = [Pattern(size=2, value=x) for x in [14, 5, 8, 3, 1, 2, 8, 4, 0]]
    assert Pattern.from_string("##.##.#..#........##.##.#..#........").subpatterns() == patterns
    assert Pattern.from_subpatterns(patterns) == \
        Pattern.from_string("##.##.#..#........##.##.#..#........")

    rules = parse_rules([
        "../.# => ##./#../...",
        ".#./..#/### => #..#/..../..../#..#",
    ])
    assert run(rules, 2) == Pattern.from_string("##.##.#..#........##.##.#..#........")
    assert pixels_on_after(rules, 2) == 12


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(21)
    rules = parse_rules(data.strip().split("\n"))
    print("Part 1: {}".format(pixels_on_after(rules, 5)))
    print("Part 2: {}".format(pixels_on_after(rules, 18)))
