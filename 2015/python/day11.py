import collections
import fileinput


def is_valid(password):
    if "i" in password or "o" in password or "l" in password:
        return False

    seen_pairs = []
    for i, pair in enumerate(zip(password, password[1:])):
        if pair[0] == pair[1]:
            seen_pairs.append(i)

    if len(seen_pairs) < 2 or (len(seen_pairs) == 2 and seen_pairs[1] - seen_pairs[0] == 1):
        return False

    has_run = False
    nums = [ord(c) for c in password]
    for a, b, c in zip(nums, nums[1:], nums[2:]):
        if a + 1 == b and b + 1 == c:
            has_run = True
    if not has_run:
        return False

    return True


def next_password(password):
    for i, c in enumerate(password):
        if c in "iol":
            password = "%s%s%s" % (
                password[:i], chr(ord(c) + 1), "a" * (8 - i - 1)
            )
            break

    return from_int(to_int(password) + 1)


def to_int(password):
    n = 0
    for c in password:
        n *= 26
        n += ord(c) - ord("a")

    return n


def from_int(n):
    chars = []
    for i in range(8):
        chars.append(chr((n % 26) + ord("a")))
        n //= 26

    return "".join(reversed(chars))


def part1(password):
    password = next_password(password)
    while not is_valid(password):
        password = next_password(password)

    return password


def part2(password):
    return part1(part1(password))


if __name__ == "__main__":
    password = next(fileinput.input()).strip()

    print("Part 1: %s" % part1(password))
    print("Part 2: %s" % part2(password))
