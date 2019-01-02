import collections
import fileinput
import itertools


class NanoBot:
    def __init__(self, x, y, z, r):
        self.x = x
        self.y = y
        self.z = z
        self.r = r

    def distance(self, x, y, z):
        return abs(self.x - x) + abs(self.y - y) + abs(self.z - z)

    def __repr__(self):
        return "NanoBot(%s, %s, %s, %s)" % (self.x, self.y, self.z, self.r)


class Octant:
    def __init__(self, center, dimensions):
        self.center = center
        self.dimensions = dimensions

    def subdivide(self):
        new_dimensions = tuple(d / 2.0 for d in self.dimensions)
        corner = tuple(a - b / 2.0 for (a, b) in zip(self.center, self.dimensions))
        for multiples in itertools.product([0, 1], repeat=3):
            new_center = tuple(a + b *c for a, b, c in zip(corner, new_dimensions, multiples))
            yield Octant(new_center, new_dimensions)

    def __repr__(self):
        return "Octant(%s, %s)" % (self.center, self.dimensions)


def parse_line(line):
    position_part, radius_part = line.split()
    x, y, z = [int(n) for n in position_part[5:-2].split(",")]
    r = int(radius_part[2:])
    return NanoBot(x, y, z, r)


def p1(bots):
    strongest = max(bots, key=lambda bot: bot.r)
    distances = [strongest.distance(bot.x, bot.y, bot.z) for bot in bots]
    return len([d for d in distances if d <= strongest.r])


def p2(bots):
    pass
    # pairwise_intersections = collections.defaultdict(list)
    # for b1, b2 in itertools.product(bots, repeat=2):
    #     if b1.distance(b2.x, b2.y, b2.z) <= b1.r + b2.r:
    #         if (b1.x, b1.y, b1.z, b1.r) < (b2.x, b2.y, b2.z, b2.r):
    #             pairwise_intersections[(b1.x, b1.y, b1.z)].append((b2.x, b2.y, b2.z))

    # by_intersectiness = list(sorted(pairwise_intersections.keys(),
    #                                 key=lambda x:-len(pairwise_intersections[x])))
    # for x in by_intersectiness:
    #     print(x, len(pairwise_intersections[x]))
    # most_intersecty = by_intersectiness[0]
    # intersecting = set(pairwise_intersections[most_intersecty])
    # missing = [b for b in bots if not (b.x, b.y, b.z) in intersecting]

    # for b in missing:
    #     intersectyness = len(pairwise_intersections[(b.x, b.y, b.z)])
    #     print(b, intersectyness, b.distance(*most_intersecty))


    # print([b for b in bots if not (b.x, b.y, b.z) in pairwise_intersections])
    # return len(pairwise_intersections)


if __name__ == "__main__":
    bots = [parse_line(line.strip()) for line in fileinput.input()]
    print(p1(bots))
    print(p2(bots))


# Notes:
#
# Most of the bots pairwise-intersect each other: 484217 or 499500 pairs
# 2 points don't intersect anything:
#   237310742, 38054863, 37407946, 79970373
#   130170153, 4438572, 143869320, 51423023
# The most 'intersecty' point intersects 981 points:
#   -52695886, 28373100, 52495119
# The smallest radius is 49617730
# The largest radius is 99192429
# x, y, z ranges of centers are:
#   -166955194, 237310742
#   -133651644, 173150114
#   -39109135,  185705169
