import fileinput
import itertools


class DisjointSet:
    def __init__(self, elements):
        self.elements = {e: i for i, e in enumerate(elements)}
        self.ids = list(range(len(elements)))

    def union(self, a, b):
        i = self.root(a)
        j = self.root(b)
        self.ids[j] = i

    def find(self, a, b):
        return self.root(a) == self.root(b)

    def root(self, a):
        i = self.elements[a]

        root = i
        while root != self.ids[root]:
            root = self.ids[root]

        while i != root:
            parent = self.ids[i]
            self.ids[i] = root
            i = parent

        return root


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
