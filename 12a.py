class AssembunnyRunner(object):
    def __init__(self, data):
        self.registers = {'a': 0, 'b': 0, 'c': 0, 'd': 0}
        lines = [s.strip() for s in data.strip().split('\n')]
        self.instructions = [line.split(' ', 1) for line in lines]

    def parse_value(self, source):
        try:
            return int(source)
        except ValueError:
            return self.registers[source]

    def run(self):
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


def doit(data):
    runner = AssembunnyRunner(data)
    runner.run()
    return runner.registers['a']


if __name__ == '__main__':
    assert doit(
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

    with open('data/d12.txt') as f:
        data = f.read()
    print(doit(data))
