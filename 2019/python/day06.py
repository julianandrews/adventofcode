import collections

from utils import read_data
from utils.graphs import bfs


def parse_orbits(s):
    pairs = [line.split(")") for line in s.strip().split()]
    orbit_map = collections.defaultdict(set)
    for a, b in pairs:
        orbit_map[a].add(b)
        orbit_map[b].add(a)

    return orbit_map


def p1(neighbors):
    return sum(node.depth for node in bfs("COM", lambda value: neighbors[value]))


def p2(neighbors):
    for node in bfs("YOU", lambda value: neighbors[value]):
        if node.value == "SAN":
            return node.depth - 2


def run_tests():
    neighbors = parse_orbits("""
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L""")
    assert p1(neighbors) == 42
    neighbors = parse_orbits("""
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN""")
    assert p2(neighbors) == 4


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(6)
    neighbors = parse_orbits(data)

    print("Part 1: {}".format(p1(neighbors)))
    print("Part 2: {}".format(p2(neighbors)))
