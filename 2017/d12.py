import utils.graphs

from utils import read_data


def parse_data(data):
    neighbors = {}
    for line in data.strip().split("\n"):
        value, destinations = line.split(" <-> ")
        neighbors[int(value.strip())] = [int(x) for x in destinations.strip().split(", ")]

    return neighbors


def component_count(neighbors):
    return len(list(utils.graphs.connected_components(list(neighbors.keys()), neighbors.get)))


def component_size(neighbors, start):
    return len(list(utils.graphs.dfs(start, neighbors.get)))


def run_tests():
    neighbors = parse_data("""
        0 <-> 2
        1 <-> 1
        2 <-> 0, 3, 4
        3 <-> 2, 4
        4 <-> 2, 3, 6
        5 <-> 6
        6 <-> 4, 5
    """)
    assert component_size(neighbors, 0) == 6
    assert component_count(neighbors) == 2


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(12).strip()
    neighbors = parse_data(data)

    print("Part 1: {}".format(component_size(neighbors, 0)))
    print("Part 2: {}".format(component_count(neighbors)))
