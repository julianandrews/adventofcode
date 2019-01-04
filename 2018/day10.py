import fileinput


class PointSet:
  EXPECTED_PACKING_FRACTION = 31 / 80.0

  def __init__(self, points):
    self.points = points
    self.step_count = 0

  def step(self, n):
    self.step_count += n
    for point in self.points:
      point.step(n)

  def expected_gradient(self):
    return int(len(self.points) / self.EXPECTED_PACKING_FRACTION)

  def gradient(self):
    self.step(1)
    new_value = self.minimized_value()
    self.step(-1)

    return new_value - self.minimized_value()

  def minimized_value(self):
    x_range, y_range = self.get_ranges()
    return (x_range[1] - x_range[0]) * (y_range[1] - y_range[0])

  def get_ranges(self):
    return ((min(p.position[0] for p in self.points),
             max(p.position[0] for p in self.points)),
            (min(p.position[1] for p in self.points),
             max(p.position[1] for p in self.points)))

  def __str__(self):
    x_range, y_range = self.get_ranges()

    grid = [['.'
             for i in range(x_range[1] - x_range[0] + 1)]
            for j in range(y_range[1] - y_range[0] + 1)]
    for point in self.points:
      grid[point.position[1] - y_range[0]][point.position[0] - x_range[0]] = '#'
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


def p_both(point_set):
  step = -point_set.gradient() // point_set.expected_gradient()
  while step:
    point_set.step(step)
    step = -point_set.gradient() // point_set.expected_gradient()

  # Switch to linear scan
  direction = -point_set.gradient() // abs(point_set.gradient())
  best = point_set.minimized_value()
  while True:
    point_set.step(direction)
    value = point_set.minimized_value()
    if value < best:
      best = value
    else:
      point_set.step(-direction)
      break

  return str(point_set)


if __name__ == '__main__':
  point_set = PointSet([Point.from_line(line) for line in fileinput.input()])
  print(p_both(point_set))
