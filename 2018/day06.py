import fileinput


from utils.graphs import bfs
from utils.point import Point


def p2(points, max_distance):
    center = Point(
        int(round(sum(x) / len(x))) for x in zip(*map(iter, points))
    )

    def neighbors(point):
        return (
            n for n in point.manhattan_neighbors()
            if sum(n.manhattan_distance(p) for p in points) < max_distance
        )

    set_points = set(bfs(center, neighbors))

    return len(set_points)


if __name__ == "__main__":
    points = [
        Point(map(int, line.split(", "))) for line in fileinput.input()
    ]
    print("Part 2: %s" % p2(points, 10000))
