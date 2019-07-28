import collections
import fileinput


def part1(lines):
    vowels = {"a", "e", "i", "o", "u"}
    forbidden_pairs = {("a", "b"), ("c", "d"), ("p", "q"), ("x", "y")}

    def is_nice(line):
        vowel_count = len([c for c in line if c in vowels])
        pairs = list(zip(line, line[1:]))
        has_pair = any(pair[0] == pair[1] for pair in pairs)
        has_forbidden_pair = any(pair in forbidden_pairs for pair in pairs)

        return vowel_count >= 3 and has_pair and not has_forbidden_pair

    return len([line for line in lines if is_nice(line)])


def part2(lines):
    def is_nice(line):
        seen = collections.defaultdict(list)
        for i, pair in enumerate(zip(line, line[1:])):
            seen[pair].append(i)
        has_double_pair = any(
            len(l) > 2 or len(l) == 2 and abs(l[1] - l[0]) > 1
            for l in seen.values()
        )

        has_skip_pair = any(pair[0] == pair[1] for pair in zip(line, line[2:]))

        return has_double_pair and has_skip_pair

    return len([line for line in lines if is_nice(line)])


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]

    print("Part 1: %s" % part1(lines))
    print("Part 2: %s" % part2(lines))
