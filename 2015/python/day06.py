import collections
import enum
import fileinput
import numpy


Instruction = collections.namedtuple("Instruction", ["action", "sx", "sy"])


class Action(enum.Enum):
    TURN_ON = 0
    TURN_OFF = 1
    TOGGLE = 2

    @classmethod
    def from_word(cls, word):
        return {
            "on": Action.TURN_ON,
            "off": Action.TURN_OFF,
            "toggle": Action.TOGGLE
        }[word]


class Grid:
    def __init__(self):
        self.array = numpy.zeros((1000, 1000), dtype=int)

    def total_brightness(self):
        return sum(sum(self.array))

    def process(self, instruction):
        if instruction.action == Action.TURN_ON:
            self.array[instruction.sx, instruction.sy] = 1
        elif instruction.action == Action.TURN_OFF:
            self.array[instruction.sx, instruction.sy] = 0
        else:
            self.array[instruction.sx, instruction.sy] ^= 1


class BrightnessGrid(Grid):
    def process(self, instruction):
        if instruction.action == Action.TURN_ON:
            self.array[instruction.sx, instruction.sy] += 1
        elif instruction.action == Action.TURN_OFF:
            self.array[instruction.sx, instruction.sy] -= 1
            self.array[self.array < 0] = 0
        else:
            self.array[instruction.sx, instruction.sy] += 2


def part1(instructions):
    grid = Grid()
    for instruction in instructions:
        grid.process(instruction)

    return grid.total_brightness()


def part2(instructions):
    grid = BrightnessGrid()
    for instruction in instructions:
        grid.process(instruction)

    return grid.total_brightness()


def parse_line(line):
    words = line.split()
    action = Action.from_word(words[-4])
    min_x, min_y = [int(x) for x in words[-3].split(',')]
    max_x, max_y = [int(x) for x in words[-1].split(',')]

    return Instruction(action, slice(min_x, max_x + 1), slice(min_y, max_y + 1))


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    instructions = [parse_line(line) for line in lines]

    print("Part 1: %s" % part1(instructions))
    print("Part 2: %s" % part2(instructions))
