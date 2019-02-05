import collections

from utils import get_lines, read_data


InstructionBase = collections.namedtuple(
    "InstructionBase",
    ["reg", "action", "value", "cond_reg", "cond", "cond_value"]
)


class Instruction(InstructionBase):
    @classmethod
    def from_string(cls, string):
        action_part, condition_part = string.split(" if ")
        reg, action, value = action_part.split()
        cond_reg, cond, cond_value = condition_part.split()
        return cls(reg, action, int(value), cond_reg, cond, int(cond_value))

    def passes_condition(self, value):
        return {
            "<": value < self.cond_value,
            "<=": value <= self.cond_value,
            ">": value > self.cond_value,
            ">=": value >= self.cond_value,
            "==": value == self.cond_value,
            "!=": value != self.cond_value,
        }[self.cond]


class VirtualMachine:
    def __init__(self):
        self.registers = collections.defaultdict(int)

    def run_program(self, program):
        for instruction in program:
            self.run_instruction(instruction)

    def run_instruction(self, instruction):
        if instruction.passes_condition(self.registers[instruction.cond_reg]):
            self.execute(instruction)

    def execute(self, instruction):
        if instruction.action == "inc":
            self.registers[instruction.reg] += instruction.value
        else:
            self.registers[instruction.reg] -= instruction.value


def get_max_register(program):
    vm = VirtualMachine()
    vm.run_program(program)
    return max(vm.registers.values())


def get_max_register_ever(program):
    best = 0
    vm = VirtualMachine()
    for instruction in program:
        vm.run_instruction(instruction)
        best = max(best, max(vm.registers.values()))

    return best


def run_tests():
    test_program = [
        Instruction.from_string(line) for line in [
            "b inc 5 if a > 1",
            "a inc 1 if b < 5",
            "c dec -10 if a >= 1",
            "c inc -20 if c == 10",
        ]]
    assert get_max_register(test_program) == 1
    assert get_max_register_ever(test_program) == 10


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    program = [Instruction.from_string(line) for line in get_lines(read_data(8))]
    print("Part 1: {}".format(get_max_register(program)))
    print("Part 2: {}".format(get_max_register_ever(program)))
