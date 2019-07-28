import enum
import fileinput

from utils.graphs import toposort


class OpType(enum.Enum):
    AND = 0
    LSHIFT = 1
    NOT = 2
    OR = 3
    RSHIFT = 4

    def evaluate(self, values):
        if self == OpType.AND:
            return values[0] & values[1]
        elif self == OpType.LSHIFT:
            return (values[0] << values[1]) & ((1 << 16) - 1)
        elif self == OpType.NOT:
            return (1 << 16) - 1 - values[0]
        elif self == OpType.OR:
            return values[0] | values[1]
        elif self == OpType.RSHIFT:
            return values[0] >> values[1]
        else:
            raise ValueError("Invalid operation.")


class Op:
    def __init__(self, op):
        self.op = op
        words = op.split()

        if len(words) == 1:
            self.optype = None
            self.values = (self.parse_value(words[0]), )
        elif len(words) == 2:
            self.optype = OpType.NOT
            self.values = (self.parse_value(words[1]), )
        elif len(words) == 3:
            self.optype = OpType[words[1]]
            self.values = (
                self.parse_value(words[0]),
                self.parse_value(words[2]),
            )
        else:
            raise ValueError("Invalid instruction.")

    def evaluate(self, wire_values):
        numbers = tuple(wire_values.get(value, value) for value in self.values)

        return numbers[0] if self.optype is None else self.optype.evaluate(numbers)

    def __repr__(self):
        return "Op('%s')" % self.op

    @staticmethod
    def parse_value(value):
        try:
            return int(value)
        except ValueError:
            return value


def evaluate_instructions(instructions, wire_values):
    instruction_map = {out: (out, op) for (out, op) in instructions}

    def neighbors(instruction):
        out, op = instruction
        return [instruction_map[x] for x in op.values if type(x) == str]

    for out, op in reversed(toposort(instructions, neighbors)):
        if out not in wire_values:
            wire_values[out] = op.evaluate(wire_values)

    return wire_values


def part1(instructions):
    return evaluate_instructions(instructions, {})["a"]


def part2(instructions):
    initial = evaluate_instructions(instructions, {})
    final = evaluate_instructions(instructions, {"b": initial["a"]})

    return final["a"]


def parse_line(line):
    op, out = line.split(" -> ")
    return (out, Op(op))


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    instructions = [parse_line(line) for line in lines]

    print("Part 1: %s" % part1(instructions))
    print("Part 2: %s" % part2(instructions))
