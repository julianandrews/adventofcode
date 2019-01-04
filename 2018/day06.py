import fileinput


class Point:

  def __init__(self, x, y):
    self.x = x
    self.y = y

  def distance(self, other):
    return abs(self.x - other.x) + abs(self.y - other.y)

  def neighbors(self):
    return [
        Point(self.x + i, self.y + j)
        for (i, j) in ((1, 0), (-1, 0), (0, 1), (0, -1))
    ]

  def total_distance(self, points):
    return sum(abs(self.distance(point)) for point in points)

  def __iter__(self):
    return iter((self.x, self.y))

  def __repr__(self):
    return "Point(%s, %s)" % (self.x, self.y)

  def __hash__(self):
    return hash((self.x, self.y))

  def __eq__(self, other):
    return self.x == other.x and self.y == other.y


def p2(points, max_distance):
  center = Point(
      *[int(round(sum(x) / len(x))) for x in zip(*map(iter, points))])
  set_points = set()
  working_points = {center}
  while working_points:
    difference = working_points - set_points
    set_points |= working_points
    working_points = {
        n for point in difference for n in point.neighbors()
        if n.total_distance(points) < max_distance
    }

  return len(set_points)


if __name__ == "__main__":
  points = [
      Point(*tuple(map(int, line.split(", ")))) for line in fileinput.input()
  ]
  print(p2(points, 10000))
