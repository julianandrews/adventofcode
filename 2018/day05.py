import fileinput
import string


def collapse(polymer):
    units = []

    for c in polymer:
        units.append(c)
        while len(units) > 1 and abs(ord(units[-1]) - ord(units[-2])) == 32:
            units.pop()
            units.pop()

    return "".join(units)


def p1(polymer):
    result = collapse(polymer)

    return len(result)


def p2(polymer):
    collapsed = collapse(polymer)

    candidates = [
        collapse([x for x in collapsed if x not in {c, chr(ord(c) + 32)}])
        for c in string.ascii_uppercase
    ]
    return min(len(x) for x in candidates)


if __name__ == "__main__":
    polymer = next(iter(fileinput.input())).strip()
    print("Part 1: %d" % p1(polymer))
    print("Part 2: %d" % p2(polymer))
