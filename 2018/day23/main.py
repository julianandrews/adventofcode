import fileinput


class NanoBot:
    def __init__(self, x, y, z, r):
        self.x = x
        self.y = y
        self.z = z
        self.r = r

    def distance(self, other):
        return abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)

    def __repr__(self):
        return "NanoBot(%s, %s, %s, %s)" % (self.x, self.y, self.z, self.r)


def parse_line(line):
    position_part, radius_part = line.split()
    x, y, z = [int(n) for n in position_part[5:-2].split(",")]
    r = int(radius_part[2:])
    return NanoBot(x, y, z, r)


def p1(bots):
    strongest = max(bots, key=lambda bot: bot.r)
    distances = [strongest.distance(bot) for bot in bots]
    return len([d for d in distances if d <= strongest.r])


if __name__ == "__main__":
    bots = [parse_line(line.strip()) for line in fileinput.input()]
    print(p1(bots))
