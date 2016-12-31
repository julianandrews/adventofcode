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

    def toggle_instruction(self, i):
        if i >= len(self.instructions):
            return
        inst, args = self.instructions[i]
        if inst == 'dec':
            self.instructions[i] = ['inc', args]
        elif inst == 'inc':
            self.instructions[i] = ['dec', args]
        elif inst == 'tgl':
            self.instructions[i] = ['inc', args]
        elif inst == 'jnz':
            self.instructions[i] = ['cpy', args]
        else:
            self.instructions[i] = ['jnz', args]

    def is_addition(self, i):
        inst, args = self.instructions[i]
        next_inst, next_args = self.instructions[i + 1]
        return inst == 'inc' and next_inst == 'dec' and next_args != args and \
            self.instructions[i + 2] == ['jnz', '{} -2'.format(next_args)]

    def is_multiplication(self, i):
        if len(self.instructions) < i + 6:
            return False

        source = self.instructions[i][1].split(' ')[0]
        target = self.instructions[i + 1][1]
        counter = self.instructions[i + 2][1]
        multiplier = self.instructions[i + 4][1]

        return self.instructions[i:i + 6] == [
            ['cpy',  '{} {}'.format(source, counter)],
            ['inc',  '{}'.format(target)],
            ['dec',  '{}'.format(counter)],
            ['jnz',  '{} -2'.format(counter)],
            ['dec',  '{}'.format(multiplier)],
            ['jnz',  '{} -5'.format(multiplier)],
        ]

    def __call__(self):
        i = 0
        while i < len(self.instructions):
            inst, args = self.instructions[i]
            if inst == 'cpy':
                source, target = args.split(' ')
                if self.registers.get(target) is not None:
                    if self.is_multiplication(i):
                        a = self.parse_value(source)
                        b = self.parse_value(self.instructions[i + 4][1])
                        destination = self.instructions[i + 1][1]
                        self.registers[destination] += a * b
                        self.registers[self.instructions[i + 2][1]] = 0
                        self.registers[self.instructions[i + 4][1]] = 0
                        i += 5
                    else:
                        self.registers[target] = self.parse_value(source)
            elif inst == 'inc':
                next_args = self.instructions[i + 1][1]
                if self.registers.get(args) is not None:
                    if self.is_addition(i):
                        self.registers[args] += self.registers[next_args]
                        self.registers[next_args] = 0
                        i += 2
                    else:
                        self.registers[args] += 1
            elif inst == 'dec':
                if self.registers.get(args) is not None:
                    self.registers[args] -= 1
            elif inst == 'jnz':
                flag, offset = args.split(' ')
                if self.parse_value(flag):
                    i += self.parse_value(offset) - 1
            elif inst == 'tgl':
                offset = self.parse_value(args)
                self.toggle_instruction(i + offset)

            i += 1


def get_a_register(data, initial=None):
    runner = AssembunnyRunner(data, initial)
    runner()
    return runner.registers['a']


if __name__ == '__main__':
    data = read_data(23)

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
    assert get_a_register(
        """
        cpy 2 a
        tgl a
        tgl a
        tgl a
        cpy 1 a
        dec a
        dec a
        """
    ) == 3
    print("All tests passed")

    print(get_a_register(data, {'a': 7}))
    print(get_a_register(data, {'a': 12}))
