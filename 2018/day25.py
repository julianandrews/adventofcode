import fileinput
import itertools

from utils.disjointset import DisjointSet


def distance(point1, point2):
    return sum(abs(a - b) for a, b in zip(point1, point2))


def p1(points):
    disjoint_set = DisjointSet(points)
    for (a, b) in itertools.product(points, repeat=2):
        if a != b and distance(a, b) <= 3:
            disjoint_set.union(a, b)

    return len(set(map(disjoint_set.root, points)))


if __name__ == "__main__":
    points = [
        tuple(int(x) for x in line.split(","))
        for line in fileinput.input()
    ]
    print(p1(points))
