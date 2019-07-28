import collections
import fileinput
import re


def tokenize(molecule):
    return tuple(re.findall('[A-Z][a-z]*', molecule))


def neighbors(replacements, molecule):
    neighbors = set()
    for i, atom in enumerate(molecule):
        for replacement in replacements[atom]:
            neighbors.add(
                tuple(molecule[:i] + replacement + molecule[i + 1:])
            )

    return neighbors


def part1(replacements, molecule):
    return len(neighbors(replacements, molecule))


def part2(replacements, molecule):
    # All the rules generate exactly two outputs from one input except that Rn,
    # and Y never generate anything, and are in rules that always generate 2
    # extra molecules. Assuming the target can be generated, the number of
    # steps must just depend only on the length of the target, and the number
    # of Rn and Y molecules.
    counts = collections.Counter(molecule)

    return len(molecule) - 1 - 2 * counts["Rn"] - 2 * counts["Y"]


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    pairs = [line.split(" => ") for line in lines[:-2]]
    replacements = collections.defaultdict(set)
    for a, b in pairs:
        replacements[a].add(tokenize(b))
    molecule = tokenize(lines[-1])

    print("Part 1: %s" % part1(replacements, molecule))
    print("Part 2: %s" % part2(replacements, molecule))
