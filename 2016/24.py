import collections
import itertools

from utils import bfs, lines, read_data


class MapGraph:
    def __init__(self, ls):
        self.lines = ls
        destinations = set()
        for x in range(len(self.lines[0])):
            for y in range(len(self.lines)):
                value = self.lines[y][x]
                if self.lines[y][x].isdigit():
                    destinations.add((x, y))
                if value == "0":
                    self.starting_point = (x, y)

        self.distances = collections.defaultdict(dict)
        for start_point in destinations:
            seen = {start_point}
            for node in bfs(start_point, self.neighbors):
                point = node.value
                if point in destinations:
                    seen.add(point)
                    self.distances[start_point][point] = node.depth
                    self.distances[point][start_point] = node.depth
                    if len(seen) == len(destinations):
                        break

    def neighbors(self, position):
        for x in (position[0] - 1, position[0] + 1):
            y = position[1]
            if 0 <= x < len(self.lines[0]):
                if self.lines[y][x] != "#":
                    yield (x, y)
        for y in (position[1] - 1, position[1] + 1):
            x = position[0]
            if 0 <= y < len(self.lines):
                if self.lines[y][x] != "#":
                    yield (x, y)

    def shortest_traversal(self):
        traversal_lengths = []
        remaining_points = set(self.distances.keys()) - {self.starting_point}
        for traversal in itertools.permutations(remaining_points):
            traversal_lengths.append(
                sum(self.distances[a][b] for a, b in zip(
                    [self.starting_point] + list(traversal), traversal
                ))
            )

        return min(traversal_lengths)

    def shortest_circuit(self):
        circuit_lengths = []
        for circuit in itertools.permutations(self.distances.keys()):
            # circuit = [self.starting_point] + list(circuit) + [self.starting_point]
            circuit_lengths.append(
                sum(self.distances[a][b] for a, b in zip(
                    [self.starting_point] + list(circuit),
                    list(circuit) + [self.starting_point]
                ))
            )

        return min(circuit_lengths)


if __name__ == "__main__":
    test_graph = MapGraph([
        "###########",
        "#0.1.....2#",
        "#.#######.#",
        "#4.......3#",
        "###########",
    ])
    assert test_graph.shortest_traversal() == 14
    assert test_graph.shortest_circuit() == 20
    print("All tests passed")
    data = read_data(24)

    map_graph = MapGraph(lines(data))
    print("Part 1: %s" % map_graph.shortest_traversal())
    print("Part 2: %s" % map_graph.shortest_circuit())
