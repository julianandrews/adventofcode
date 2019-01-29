import itertools

from utils import lines, read_data


class AssembunnyRunner(object):
    def __init__(self, initial=None):
        self.registers = {'a': 0, 'b': 0, 'c': 0, 'd': 0}
        self.registers.update(initial if initial is not None else {})
        self.ptr = 0

    def state(self):
        return tuple([self.ptr] + list(self.registers[x] for x in "abcd"))

    def parse_value(self, source):
        try:
            return int(source)
        except ValueError:
            return self.registers[source]

    def output_generator(self, instructions):
        self.ptr = 0
        while self.ptr < len(instructions):
            inst, args = instructions[self.ptr]
            if inst == 'cpy':
                source, target = args.split(' ')
                if self.registers.get(target) is not None:
                    self.registers[target] = self.parse_value(source)
            elif inst == 'inc':
                if self.registers.get(args) is not None:
                    self.registers[args] += 1
            elif inst == 'dec':
                if self.registers.get(args) is not None:
                    self.registers[args] -= 1
            elif inst == 'jnz':
                flag, offset = args.split(' ')
                if self.parse_value(flag):
                    self.ptr += self.parse_value(offset) - 1
            elif inst == 'out':
                yield self.parse_value(args)

            self.ptr += 1


def find_clock_signal(instructions):
    a = 0
    while True:
        vm = AssembunnyRunner(initial={'a': a})
        signal = vm.output_generator(instructions)
        seen = set()
        for expected_signal in itertools.cycle((0, 1)):
            value = next(signal)
            if value != expected_signal:
                a += 1
                break
            state = vm.state()
            if state in seen:
                return a
            seen.add(state)


def find_clock_signal_fast():
    """Disassembly shows that the first handful of lines set d = 2538 + a, and
    then output the repeated binary representation of d. The solution is the
    first binary number that looks like '101010...' greater than 2538."""
    offset = 2538
    n = offset
    digits = 0
    while n > 0:
        digits += 2
        n = (n - 2) >> 2
    return int('10' * (digits // 2), 2) - offset


if __name__ == '__main__':
    data = read_data(25)
    instructions = [line.split(' ', 1) for line in lines(data)]

    print("Part 1: %s" % find_clock_signal_fast())
    print("All done!")
