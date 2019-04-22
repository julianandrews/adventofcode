from utils import read_data


class CubeCoordinates:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def distance(self, other):
        return (abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)) // 2

    def neighbor(self, direction):
        if direction == "n":
            return CubeCoordinates(self.x - 1, self.y + 1, self.z)
        elif direction == "ne":
            return CubeCoordinates(self.x, self.y + 1, self.z - 1)
        elif direction == "se":
            return CubeCoordinates(self.x + 1, self.y, self.z - 1)
        elif direction == "s":
            return CubeCoordinates(self.x + 1, self.y - 1, self.z)
        elif direction == "sw":
            return CubeCoordinates(self.x, self.y - 1, self.z + 1)
        elif direction == "nw":
            return CubeCoordinates(self.x - 1, self.y, self.z + 1)


def max_distance(directions):
    origin = CubeCoordinates(0, 0, 0)
    locations = [origin]
    for direction in directions:
        locations.append(locations[-1].neighbor(direction))

    return max(location.distance(origin) for location in locations)


def distance_to_path(directions):
    origin = CubeCoordinates(0, 0, 0)
    location = origin
    for direction in directions:
        location = location.neighbor(direction)

    return location.distance(origin)


def run_tests():
    assert distance_to_path(["ne", "ne", "ne"]) == 3
    assert distance_to_path(["ne", "ne", "sw", "sw"]) == 0
    assert distance_to_path(["ne", "ne", "s", "s"]) == 2
    assert distance_to_path(["se", "sw", "se", "sw", "sw"]) == 3


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(11).strip()
    directions = data.split(",")
    print("Part 1: {}".format(distance_to_path(directions)))
    print("Part 2: {}".format(max_distance(directions)))
