import fileinput
import functools


class Box:
    def __init__(self, sides):
        self.sides = tuple(sides)

    def paper_needed(self):
        areas = (
            self.sides[0] * self.sides[1],
            self.sides[1] * self.sides[2],
            self.sides[2] * self.sides[0],
        )

        return 2 * sum(areas) + min(areas)

    def ribbon_needed(self):
        min_perimeter = 2 * (sum(self.sides) - max(self.sides))

        return min_perimeter + functools.reduce(int.__mul__, self.sides, 1)

    @classmethod
    def from_string(cls, s):
        return cls(int(x) for x in s.split("x"))


def part1(boxes):
    return sum(box.paper_needed() for box in boxes)


def part2(boxes):
    return sum(box.ribbon_needed() for box in boxes)


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    boxes = [Box.from_string(line) for line in lines]

    print("Part 1: %s" % part1(boxes))
    print("Part 2: %s" % part2(boxes))
