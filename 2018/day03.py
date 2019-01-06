import collections
import fileinput
import re

Claim = collections.namedtuple('Claim', ['id', 'x', 'y', 'w', 'h'])
CLAIM_RE = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')


def parse_claim(s):
    return Claim(*map(int, CLAIM_RE.match(s.strip()).groups()))


def claim_points(claim):
    return {(x, y)
            for x in range(claim.x, claim.x + claim.w)
            for y in range(claim.y, claim.y + claim.h)}


def get_overlap(claims):
    all_points = set()
    overlap = set()
    for claim in claims:
        new_points = claim_points(claim)
        overlap |= (new_points & all_points)
        all_points |= new_points
    return overlap


def p1(data):
    claims = [parse_claim(s) for s in data]
    return len(get_overlap(claims))


def p2(data):
    claims = [parse_claim(s) for s in data]
    overlap = get_overlap(claims)

    good_claims = [
        claim.id for claim in claims if not claim_points(claim) & overlap
    ]

    assert len(good_claims) == 1
    return good_claims[0]


if __name__ == '__main__':
    data = [line.strip() for line in fileinput.input()]
    print('Part 1: %s' % p1(data))
    print('Part 2: %s' % p2(data))
