import collections

from utils import read_data
from utils.math import Quadratic, dot_product


class Particle:
    def __init__(self, index, position, velocity, acceleration):
        self.index = index
        self.position = position
        self.velocity = velocity
        self.acceleration = acceleration

    def collides(self, other, time):
        # Check all three dimensions.
        for i in range(len(self.position)):
            quadratic = self.difference_equation_quadratic(other, i)
            if quadratic.value(time) != 0:
                return False
        return True

    def collision_time(self, other):
        # Find the collision times in the first dimension.
        quadratic = self.difference_equation_quadratic(other, 0)
        candidate_times = quadratic.integer_solutions()

        # Filter to actual collisions for time > 0.
        collision_times = [
            time for time in candidate_times
            if time > 0 and self.collides(other, time)
        ]

        return min(collision_times) if collision_times else None

    def difference_equation_quadratic(self, other, i):
        # XXX: return a Quadratic!
        # The kinematic difference equation for motion gives us a quadratic for
        # particle separation with these coefficients.
        a = self.acceleration[i] - other.acceleration[i]
        b = (2 * (self.velocity[i] - other.velocity[i]) +
             self.acceleration[i] - other.acceleration[i])
        c = 2 * (self.position[i] - other.position[i])

        return Quadratic(a, b, c)

    @classmethod
    def from_string(cls, index, line):
        parts = line.strip().split(", ")
        return cls(
            index,
            *(tuple(int(x) for x in part[3:-1].split(",")) for part in parts)
        )

    @classmethod
    def particles_from_lines(cls, lines):
        return [cls.from_string(i, line) for i, line in enumerate(lines)]


def laggard_index(particles):
    # Eventually, acceleration dominates, so the least accelerated particle lags.
    # In case of a tie, the initial velocity in the direction of acceleration wins.
    # As a final tie breaker, initial position in the direction of acceleration wins.
    particle = min(particles, key=lambda p: (
        dot_product(p.acceleration, p.acceleration),
        dot_product(p.acceleration, p.velocity),
        dot_product(p.acceleration, p.position)
    ))

    return particle.index


def survivor_count(particles):
    collisions = collections.defaultdict(list)
    for i in range(len(particles) - 1):
        for j in range(i + 1, len(particles)):
            time = particles[i].collision_time(particles[j])
            if time is not None:
                collisions[time].append((i, j))

    survivors = set(range(len(particles)))
    for time in sorted(collisions.keys()):
        removed = set()
        for (i, j) in collisions[time]:
            if i in survivors and j in survivors:
                removed.add(i)
                removed.add(j)
        survivors -= removed

    return len(survivors)


def run_tests():
    lines = [
        "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>",
        "p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>",
    ]
    assert laggard_index(Particle.particles_from_lines(lines)) == 0

    lines = [
        "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>",
        "p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>",
        "p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>",
        "p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>",
    ]
    assert survivor_count(Particle.particles_from_lines(lines)) == 1


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(20)
    particles = Particle.particles_from_lines(data.strip().split("\n"))
    print("Part 1: {}".format(laggard_index(particles)))
    print("Part 2: {}".format(survivor_count(particles)))
