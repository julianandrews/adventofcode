import collections
import fileinput


BaseInstruction = collections.namedtuple("Instruction", ["op", "args"])


class Instruction(BaseInstruction):
    @property
    def register(self):
        return self.args[0] if self.op != "jmp" else None

    @property
    def offset(self):
        if self.op == "jmp":
            return int(self.args[0])
        elif self.op in {"jio", "jie"}:
            return int(self.args[1])
        else:
            return None

    def __str__(self):
        return "%s %s" % (self.op, ", ".join(self.args))


def parse_line(line):
    op, arg_part = line.split(" ", 1)
    args = tuple(arg_part.split(", "))
    return Instruction(op, args)


class VirtualMachine:
    def __init__(self):
        self.registers = {'a': 0, 'b': 0}
        self.instruction_ptr = 0

    def execute_instruction(self, instruction):
        offset = 1
        if instruction.op == "hlf":
            self.registers[instruction.register] //= 2
        elif instruction.op == "tpl":
            self.registers[instruction.register] *= 3
        elif instruction.op == "inc":
            self.registers[instruction.register] += 1
        elif instruction.op == "jmp":
            self.instruction_ptr += instruction.offset - 1
        elif instruction.op == "jie":
            if self.registers[instruction.register] % 2 == 0:
                offset = instruction.offset
        elif instruction.op == "jio":
            if self.registers[instruction.register] == 1:
                offset = instruction.offset
        self.instruction_ptr += offset

    def run_program(self, program, debug=False):
        while self.instruction_ptr >= 0 and self.instruction_ptr < len(program):
            instruction = program[self.instruction_ptr]
            if debug:
                print("{ptr:<4} {ins!s:12}{a:5}{b:5}".format(
                    ptr=self.instruction_ptr,
                    ins=instruction,
                    a=self.registers["a"],
                    b=self.registers["b"]
                ))
            self.execute_instruction(instruction)


def part1(program):
    vm = VirtualMachine()
    vm.run_program(program)

    return vm.registers["b"]


def part2(program):
    vm = VirtualMachine()
    vm.registers["a"] = 1
    vm.run_program(program)

    return vm.registers["b"]


if __name__ == "__main__":
    vm = VirtualMachine()
    vm.run_program([
        Instruction("inc", ["a"]),
        Instruction("jio", ["a", "+2"]),
        Instruction("tpl", ["a"]),
        Instruction("inc", ["a"]),
    ])
    assert vm.registers["a"] == 2
    print("All tests passed")

    lines = [line.strip() for line in fileinput.input()]
    program = [parse_line(line) for line in lines]

    print("Part 1: %s" % part1(program))
    print("Part 2: %s" % part2(program))
