import fileinput
import re


def unescape(s):
    s = s[1:-1]
    s = s.replace("\\\\", "\\")
    s = s.replace("\\\"", "\"")

    return re.sub(
        r"\\x([0-9a-f][0-9a-f])",
        lambda m: chr(int(m.group(1), 16)),
        s
    )


def escape(s):
    return "\"%s\"" % s.replace("\\", "\\\\").replace("\"", "\\\"")


def part1(lines):
    parsed = [unescape(line) for line in lines]

    return sum(map(len, lines)) - sum(map(len, parsed))


def part2(lines):
    escaped = [escape(line) for line in lines]

    return sum(map(len, escaped)) - sum(map(len, lines))


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]

    print("Part 1: %s" % part1(lines))
    print("Part 2: %s" % part2(lines))
