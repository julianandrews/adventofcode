import enum

from utils import read_data


class Firewall:
    def __init__(self, data):
        layer_data = [line.split(": ") for line in data.strip().split("\n")]
        self.layers = {int(layer): int(depth) for (layer, depth) in layer_data}
        # Scanner depth represented as index in its (2 * depth - 2) length cycle.
        self.scanners = {layer: 0 for layer in self.layers}
        self.time = 0

    def tick(self):
        for layer, depth in self.layers.items():
            self.scanners[layer] = (self.scanners[layer] + 1) % (2 * depth - 2)
        self.time += 1

    def path_is_clear(self):
        for layer, depth in self.layers.items():
            meeting_depth = (self.scanners[layer] + layer) % (2 * depth - 2)
            if meeting_depth == 0:
                return False

        return True


def trip_severity(firewall):
    severity = 0
    for tick in range(max(firewall.layers.keys()) + 1):
        if tick in firewall.scanners:
            if firewall.scanners[tick] == 0:
                severity += tick * firewall.layers[tick]

        firewall.tick()

    return severity


def first_safe_trip_time(firewall):
    while not firewall.path_is_clear():
        firewall.tick()

    return firewall.time


def run_tests():
    data = """
        0: 3
        1: 2
        4: 4
        6: 4
    """
    assert trip_severity(Firewall(data)) == 24
    assert first_safe_trip_time(Firewall(data)) == 10


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(13)
    firewall = Firewall(data)
    print("Part 1: {}".format(trip_severity(Firewall(data))))
    print("Part 2: {}".format(first_safe_trip_time(Firewall(data))))
