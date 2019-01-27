from utils import lines, read_data


class AssembunnyRunner(object):
    def __init__(self, data, initial=None):
        self.registers = {'a': 0, 'b': 0, 'c': 0, 'd': 0}
        self.registers.update(initial if initial is not None else {})
        self.instructions = [line.split(' ', 1) for line in lines(data)]

    def parse_value(self, source):
        try:
            return int(source)
        except ValueError:
            return self.registers[source]

    def __call__(self):
        i = 0
        while i < len(self.instructions):
            inst, args = self.instructions[i]
            if inst == 'cpy':
                source, target = args.split(' ')
                self.registers[target] = self.parse_value(source)
            elif inst == 'inc':
                self.registers[args] += 1
            elif inst == 'dec':
                self.registers[args] -= 1
            elif inst == 'jnz':
                flag, offset = args.split(' ')
                if self.parse_value(flag):
                    i += int(offset) - 1
            i += 1


def get_a_register(data, initial=None):
    runner = AssembunnyRunner(data, initial)
    runner()
    return runner.registers['a']


if __name__ == '__main__':
    data = read_data(12)

    assert get_a_register(
        """
        cpy 41 a
        inc a
        inc a
        dec a
        jnz a 2
        dec a
        """
    ) == 42
    print("All tests passed")

    print(get_a_register(data))
    print(get_a_register(data, {'c': 1}))
