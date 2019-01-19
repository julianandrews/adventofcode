import fileinput


class Reindeer:
    def __init__(self, name, speed, fly_time, rest_time):
        self.name = name
        self.speed = speed
        self.fly_time = fly_time
        self.rest_time = rest_time

    def distance(self, travel_time):
        cycles, remainder = divmod(travel_time, self.fly_time + self.rest_time)
        return self.speed * (self.fly_time * cycles + min(remainder, self.fly_time))

    @classmethod
    def from_string(cls, s):
        words = s.split()
        return cls(words[0], int(words[3]), int(words[6]), int(words[13]))


def part1(reindeer):
    return max(r.distance(2503) for r in reindeer)


def part2(reindeer):
    points = {r.name: 0 for r in reindeer}
    for t in range(1, 2504):
        distances = [r.distance(t) for r in reindeer]
        lead = max(distances)
        for r, d  in zip(reindeer, distances):
            if d == lead:
                points[r.name] += 1

    return max(points.values())


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    reindeer = [Reindeer.from_string(line) for line in lines]

    print("Part 1: %s" % part1(reindeer))
    print("Part 2: %s" % part2(reindeer))
