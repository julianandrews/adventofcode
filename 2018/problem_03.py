import collections

import utils


Claim = collections.namedtuple('Claim', ['num', 'x', 'y', 'h', 'w'])


def parse_claim(s):
    parts = s.split()
    num = parts[0][1:]
    x, y = parts[2][:-1].split(',')
    w, h = parts[3].split('x')
    return Claim(int(num), int(x), int(y), int(h), int(w))


def claim_points(claim):
    points = set()
    for x in range(claim.x, claim.x + claim.w):
        for y in range(claim.y, claim.y + claim.h):
            points.add((x, y))
    return points


def get_overlap(claims):
    overlap = set()
    points = set()
    for claim in claims:
        ps = claim_points(claim)
        for p in ps:
            if p in points:
                overlap.add(p)
            points.add(p)
    return overlap


def p1(data):
    claims = [parse_claim(s) for s in data]
    return len(get_overlap(claims))


def p2(data):
    claims = [parse_claim(s) for s in data]
    overlap = get_overlap(claims)

    for claim in claims:
        ps = claim_points(claim)
        if all(p not in overlap for p in ps):
            return claim.num


if __name__ == "__main__":
    data = utils.readstrings("data/input-03.txt")
    print("Part 1: %s" % p1(data))
    print("Part 2: %s" % p2(data))
