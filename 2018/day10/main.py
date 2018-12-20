import collections
import fileinput

EXPECTED_PACKING_FRACTION = 31 / 80.0


class PointSet:

  def __init__(self, points):
    self.points = points
    self.step_count = 0
    self._set_ranges()

  def step(self, n):
    self.step_count += n
    for point in self.points:
      point.step(n)
    self._set_ranges()

  def area_gradient(self):
    self.step(1)
    new_area = self.area()
    self.step(-1)

    return new_area - self.area()

  def area(self):
    return (self.x_range[1] - self.x_range[0]) * (
        self.y_range[1] - self.y_range[0])

  def _set_ranges(self):
    self.x_range = (min(p.position[0] for p in self.points),
                    max(p.position[0] for p in self.points))
    self.y_range = (min(p.position[1] for p in self.points),
                    max(p.position[1] for p in self.points))

  def __str__(self):
    grid = [['.'
             for i in range(self.x_range[1] - self.x_range[0] + 1)]
            for j in range(self.y_range[1] - self.y_range[0] + 1)]
    for point in self.points:
      grid[point.position[1] - self.y_range[0]][point.position[0] -
                                                self.x_range[0]] = '#'
    grid_string = '\n'.join(''.join(line) for line in grid)

    return 'Time: %s\n%s\n' % (self.step_count, grid_string)


class Point:

  def __init__(self, position, velocity):
    self.position = position
    self.velocity = velocity

  def __repr__(self):
    return 'Point(%s, %s)' % (self.position, self.velocity)

  @classmethod
  def from_line(cls, line):
    return cls(*(tuple(map(int,
                           s.split('<')[1].split(',')))
                 for s in line.split('>')[:-1]))

  def step(self, n):
    self.position = tuple(
        p + n * v for (p, v) in zip(self.position, self.velocity))


def p1(point_set):
  expected_area = int(len(point_set.points) / EXPECTED_PACKING_FRACTION)
  step = -point_set.area_gradient() // expected_area
  while step:
    point_set.step(step)
    step = -point_set.area_gradient() // expected_area

  CHECK_WINDOW = 10
  point_set.step(-CHECK_WINDOW // 2)
  for i in range(CHECK_WINDOW):
    print(str(point_set))
    point_set.step(1)

  return str(point_set)


if __name__ == '__main__':
  point_set = PointSet([Point.from_line(line) for line in fileinput.input()])
  p1(point_set)
