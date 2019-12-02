import enum
import logging


class Op(enum.Enum):
    ADD = 1
    MULTIPLY = 2
    HALT = 99

    @property
    def num_parameters(self):
        if self.is_binary:
            return 3
        elif self == Op.HALT:
            return 0
        else:
            raise RuntimeError(f"Unexpected Operation {self}")

    @property
    def is_binary(self):
        return self in {Op.ADD, Op.MULTIPLY}


class VM:
    logger = logging.getLogger(__name__)

    def __init__(self, memory, noun=None, verb=None):
        self.logger.debug(f"Initializing VM")
        self.memory = memory
        self.ip = 0
        if noun is not None:
            self.noun = noun
        if verb is not None:
            self.verb = verb

    def step(self):
        op = Op(self.memory[self.ip])
        params = self.memory[self.ip + 1:self.ip + op.num_parameters + 1]
        self.logger.debug(f"Executing {op} with {params}")
        if op.is_binary:
            a = self.memory[params[0]]
            b = self.memory[params[1]]
            target = params[2]
            if op == Op.ADD:
                self.memory[target] = a + b
            elif op == Op.MULTIPLY:
                self.memory[target] = a * b
        elif op == Op.HALT:
            pass
        else:
            raise RuntimeError("Unexpected Operation {op}")

        self.ip += 1 + op.num_parameters
        return op, params

    def set_verb(self, verb):
        self.logger.debug(f"Setting verb to {verb}")
        if verb is not None:
            self.memory[2] = verb

    def run(self):
        self.logger.info(f"Starting program with inputs {self.noun}, {self.verb}")
        while True:
            op, _ = self.step()
            if op == Op.HALT:
                return

    @property
    def noun(self):
        return self.memory[1]

    @noun.setter
    def noun(self, value):
        self.logger.debug(f"Setting noun to {value}")
        self.memory[1] = value

    @property
    def verb(self):
        return self.memory[2]

    @verb.setter
    def verb(self, value):
        self.logger.debug(f"Setting verb to {value}")
        self.memory[2] = value

    @property
    def output(self):
        return self.memory[0]
