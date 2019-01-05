class Point:
    def __init__(self, coordinates):
        self.coordinates = tuple(coordinates)

    def manhattan_distance(self, other):
        return sum(
            abs(a - b)
            for a, b in zip(self.coordinates, other.coordinates)
        )

    def manhattan_neighbors(self):
        for i in range(len(self.coordinates)):
            new_coordinates = list(self.coordinates)
            for offset in (1, -1):
                new_coordinates[i] = self.coordinates[i] + offset
                yield Point(tuple(new_coordinates))

    def __iter__(self):
        return iter(self.coordinates)

    def __repr__(self):
        return "Point%s" % (self.coordinates, )

    def __hash__(self):
        return hash(self.coordinates)

    def __eq__(self, other):
        return self.coordinates == other.coordinates

    def __lt__(self, other):
        return self.coordinates < other.coordinates
