import itertools
import math

from utils import get_lines, read_data
from utils.iterables import cycle_detect


def parse_position(line):
    return [int(s[2:]) for s in line[1:-1].split(", ")]


def lcm(values):
    result = values[0]
    for value in values[1:]:
        result *= value // math.gcd(value, result)
    return result


def abs_sum(l):
    return sum(abs(x) for x in l)


class Moon:
    def __init__(self, position):
        self.position = position
        self.velocity = [0, 0, 0]

    def update_velocity(self, others):
        for other in others:
            for i, (a, b) in enumerate(zip(self.position, other.position)):
                self.velocity[i] += 0 if a == b else (1 if a < b else -1)

    def update_position(self):
        for i, v in enumerate(self.velocity):
            self.position[i] += v

    @property
    def energy(self):
        return abs_sum(self.position) * abs_sum(self.velocity)

    def __repr__(self):
        return f"Moon(position={self.position}, velocity={self.velocity})"


class PlanetarySystem:
    def __init__(self, moons):
        self.moons = moons

    def step(self):
        for moon in self.moons:
            moon.update_velocity(self.moons)
        for moon in self.moons:
            moon.update_position()

    def state(self):
        """Returns state as a tuple by dimension."""
        return tuple(
            tuple((moon.position[i], moon.velocity[i]) for moon in self.moons)
            for i in range(3)
        )

    @property
    def energy(self):
        return sum(moon.energy for moon in self.moons)


def p1(system):
    for i in range(1000):
        system.step()

    return system.energy


def p2(system):
    def states():
        while True:
            yield system.state()
            system.step()

    state_iterators = itertools.tee(states(), 3)
    cycles = [cycle_detect(state[i] for state in it) for (i, it) in enumerate(state_iterators)]
    return max(start for (start, end) in cycles) + lcm([end - start for (start, end) in cycles])


def run_tests():
    positions_a = ((-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1))
    positions_b = ((-8, -10, 0), (5, 5, 10), (2, -7, 3), (9, -8, -3))

    system = PlanetarySystem([Moon(list(p)) for p in positions_a])
    for i in range(10):
        system.step()
    assert system.energy == 179

    system = PlanetarySystem([Moon(list(p)) for p in positions_b])
    for i in range(100):
        system.step()
    assert system.energy == 1940

    system = PlanetarySystem([Moon(list(p)) for p in positions_a])
    assert p2(system) == 2772

    system = PlanetarySystem([Moon(list(p)) for p in positions_b])
    assert p2(system) == 4686774924


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(12)
    system = PlanetarySystem([Moon(parse_position(line)) for line in get_lines(data)])
    print("Part 1: {}".format(p1(system)))
    print("Part 2: {}".format(p2(system)))
