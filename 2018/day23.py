import fileinput

import z3


def manhattan_distance(point, other):
  return sum(abs(a - b) for a, b in zip(point, other))


class NanoBot:
  def __init__(self, point, radius):
    self.point = point
    self.radius = radius

  @classmethod
  def from_string(cls, string):
    position_part, radius_part = string.split()
    x, y, z = [int(n) for n in position_part[5:-2].split(",")]
    r = int(radius_part[2:])
    return cls((x, y, z), r)

  def __repr__(self):
    return "NanoBot(%s, %s)" % (self.point, self.radius)


def p1(bots):
  strongest = max(bots, key=lambda bot: bot.radius)
  distances = [manhattan_distance(strongest.point, bot.point) for bot in bots]
  return len([d for d in distances if d <= strongest.radius])


def p2(bots):
  x, y, z = z3.Int("x"), z3.Int("y"), z3.Int("z")
  num_in_range = z3.Int("num_in_range")
  opt = z3.Optimize()

  def target_distance(u, v, w):
    def z3abs(a):
      return z3.If(a >= 0, a, -a)

    return z3abs(x - u) + z3abs(y - v) + z3abs(z - w)

  opt.add(num_in_range == sum(
      z3.If(target_distance(*bot.point) <= bot.radius, 1, 0) for bot in bots)
  )

  opt.maximize(num_in_range)
  opt.minimize(target_distance(0, 0, 0))
  opt.check()

  return opt.model().eval(target_distance(0, 0, 0))


if __name__ == "__main__":
  bots = [NanoBot.from_string(line.strip()) for line in fileinput.input()]
  print(p1(bots))
  print(p2(bots))
