import collections
import fileinput
import itertools


from utils.graphs import bfs
from utils.point import Point


def debug_string(points, assigned_points):
    def char_for_value(value):
        return chr(value + 65)

    min_x, min_y = [min(values) for values in zip(*(p.coordinates for p in points))]
    max_x, max_y = [max(values) for values in zip(*(p.coordinates for p in points))]

    rows = []
    for y in range(min_y, max_y + 1):
        row = []
        for x in range(min_x, max_x + 1):
            value = assigned_points.get(Point((x, y)), "?")
            row.append("." if value is None else char_for_value(value).lower() if value is not "?" else value)
        rows.append(row)

    for i, point in enumerate(points):
        rows[point.coordinates[1] - min_y][point.coordinates[0] - min_x] = char_for_value(i)

    return "\n".join("".join(line) for line in rows)


def p1(points):
    min_x, min_y = [min(values) for values in zip(*(p.coordinates for p in points))]
    max_x, max_y = [max(values) for values in zip(*(p.coordinates for p in points))]

    assigned_points = {}
    working_points = [{point} for point in points]

    while any(ps for ps in working_points):
        new_points = [set(n for point in ps for n in point.manhattan_neighbors()) for ps in working_points]
        working_points = [set() for i in range(len(points))]
        counts = collections.Counter(p for ps in new_points for p in ps)
        for i, ps in enumerate(new_points):
            for p in ps:
                if min_x <= p.coordinates[0] <= max_x \
                        and min_y <= p.coordinates[1] <= max_y \
                        and p not in assigned_points:
                    if counts[p] == 1:
                        assigned_points[p] = i
                        working_points[i].add(p)
                    else:
                        assigned_points[p] = None

    edge_values = set()
    for x in range(min_x, max_x + 1):
        edge_values.add(assigned_points.get(Point((x, min_y)), None))
        edge_values.add(assigned_points.get(Point((x, max_y)), None))
    for y in range(min_y, max_y + 1):
        edge_values.add(assigned_points.get(Point((min_x, y)), None))
        edge_values.add(assigned_points.get(Point((max_x, y)), None))

    possible_areas = set(range(len(points))) - edge_values

    areas = collections.Counter(assigned_points.values())
    return max(areas[i] for i in possible_areas)



def p2(points, max_distance):
    center = Point(
        int(round(sum(x) / len(x))) for x in zip(*map(iter, points))
    )

    def neighbors(point):
        return (
            n for n in point.manhattan_neighbors()
            if sum(n.manhattan_distance(p) for p in points) < max_distance
        )

    set_points = {node.value for node in bfs(center, neighbors)}

    return len(set_points)


if __name__ == "__main__":
    points = [
        Point(map(int, line.split(", "))) for line in fileinput.input()
    ]
    print("Part 1: %s" % p1(points))
    print("Part 2: %s" % p2(points, 10000))
