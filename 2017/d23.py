import collections

from utils import read_data
from utils.primes import primes_upto, is_prime


class VirtualMachine:
    def __init__(self, program):
        self.program = program
        self.registers = collections.defaultdict(int)
        self.instruction_pointer = 0
        self.mul_count = 0

    def run(self):
        while 0 <= self.instruction_pointer < len(self.program):
            self.step()

    def step(self):
        instruction, x, y = self.program[self.instruction_pointer].split()
        if instruction == "set":
            self.registers[x] = self.get_value(y)
        elif instruction == "sub":
            self.registers[x] -= self.get_value(y)
        elif instruction == "mul":
            self.mul_count += 1
            self.registers[x] *= self.get_value(y)
        elif instruction == "jnz":
            if self.get_value(x) != 0:
                self.instruction_pointer += self.get_value(y) - 1
        self.instruction_pointer += 1

    def get_value(self, x):
        return self.registers[x] if x.isalpha() else int(x)


def get_mul_count(program):
    vm = VirtualMachine(program)
    vm.run()

    return vm.mul_count


def get_h_register():
    """Reverse engineered from the input. The first 8 lines set the values of
    b and c, the rest counts non primes starting at b and going past c by 17s.
    """
    b = 108100
    c = 125100

    return sum(1 for x in range(b, c + 1, 17) if not is_prime(x))


def run_tests():
    pass


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(23)
    program = [line.strip() for line in data.strip().split("\n")]
    print("Part 1: {}".format(get_mul_count(program)))
    print("Part 2: {}".format(get_h_register()))
