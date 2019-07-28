import fileinput
import hashlib


def first_with_prefix(secret, prefix):
    i = 1
    root_hash = hashlib.md5(secret.encode())
    while True:
        h = root_hash.copy()
        h.update(str(i).encode())
        if h.hexdigest().startswith(prefix):
            return i
        i += 1


def part1(secret):
    return first_with_prefix(secret, '00000')


def part2(secret):
    return first_with_prefix(secret, '000000')


if __name__ == "__main__":
    secret = next(fileinput.input()).strip()

    print("Part 1: %s" % part1(secret))
    print("Part 2: %s" % part2(secret))
