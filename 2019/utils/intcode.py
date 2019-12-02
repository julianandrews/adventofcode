import enum


class Op(enum.Enum):
    ADD = 1
    MULTIPLY = 2
    HALT = 99

    @property
    def parameters(self):
        if self == Op.HALT:
            return 0
        elif self in {Op.ADD, Op.MULTIPLY}:
            return 3


class VM:
    def __init__(self, memory):
        self.memory = memory
        self.ip = 0
        self.halted = False

    @classmethod
    def with_inputs(cls, memory, noun, verb):
        vm = VM(memory)
        vm.set_inputs(noun, verb)
        return vm

    def step(self):
        op = Op(self.memory[self.ip])
        params = self.memory[self.ip + 1:self.ip + op.parameters + 1]
        if op == Op.HALT:
            self.halted = True
        else:
            a = self.memory[params[0]]
            b = self.memory[params[1]]
            target = params[2]
            if op == Op.ADD:
                self.memory[target] = a + b
            elif op == Op.MULTIPLY:
                self.memory[target] = a * b

        self.ip += 1 + op.parameters

    def set_inputs(self, noun, verb):
        self.memory[1] = noun
        self.memory[2] = verb

    def run(self):
        while not self.halted:
            self.step()

    @property
    def output(self):
        return self.memory[0]
