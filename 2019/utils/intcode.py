import enum
import logging


class ValueMode(enum.Enum):
    POSITION = 0
    IMMEDIATE = 1


class Op(enum.Enum):
    ADD = 1
    MULTIPLY = 2
    STORE = 3
    OUTPUT = 4
    JUMP_IF_TRUE = 5
    JUMP_IF_FALSE = 6
    LESS_THAN = 7
    EQUALS = 8
    HALT = 99

    @property
    def num_parameters(self):
        if self.is_binary_op:
            return 3
        elif self.is_unary:
            return 1
        elif self.is_jump:
            return 2
        elif self == Op.HALT:
            return 0
        else:
            raise RuntimeError(f"Unexpected Operation {self}")

    @property
    def is_binary_op(self):
        return self in {Op.ADD, Op.MULTIPLY, Op.LESS_THAN, Op.EQUALS}

    @property
    def is_unary(self):
        return self in {Op.STORE, Op.OUTPUT}

    @property
    def is_jump(self):
        return self in {Op.JUMP_IF_TRUE, Op.JUMP_IF_FALSE}

    def should_jump(self, value):
        return self == Op.JUMP_IF_TRUE and value or self == Op.JUMP_IF_FALSE and not value


class VM:
    logger = logging.getLogger(__name__)

    def __init__(self, memory, noun=None, verb=None, inputs=None):
        self.logger.debug(f"Initializing VM")
        self.memory = memory
        self.ip = 0
        if noun is not None:
            self.noun = noun
        if verb is not None:
            self.verb = verb

        self.output = None
        self.inputs = inputs

    def outputs(self):
        op = None
        while op != Op.HALT:
            op, _ = self.step()
            if op == Op.OUTPUT:
                yield self.output

    def step(self):
        op = Op(self.memory[self.ip] % 100)
        params = self.memory[self.ip + 1:self.ip + op.num_parameters + 1]
        mode_str = (str(self.memory[self.ip] // 100) or '').zfill(len(params))
        modes = [ValueMode(int(c)) for c in reversed(mode_str)]
        ip_offset = op.num_parameters + 1

        self.logger.debug(f"Executing {op} with {params} and {modes}")

        if op.is_binary_op:
            a = self.get_value(params[0], modes[0])
            b = self.get_value(params[1], modes[1])
            if modes[2] != ValueMode.POSITION:
                raise RuntimeError(f"Unexpected {modes[2]} in {op} at 2")
            address = params[2]
            if op == Op.ADD:
                self.logger.debug(f"Storing {a} + {b} at {address}")
                self.memory[address] = a + b
            elif op == Op.MULTIPLY:
                self.logger.debug(f"Storing {a} * {b} at {address}")
                self.memory[address] = a * b
            elif op == Op.LESS_THAN:
                self.logger.debug(f"Storing {a} < {b} at {address}")
                self.memory[address] = 1 if a < b else 0
            elif op == Op.EQUALS:
                self.logger.debug(f"Storing {a} == {b} at {address}")
                self.memory[address] = 1 if a == b else 0
        elif op == Op.STORE:
            if modes[0] != ValueMode.POSITION:
                raise RuntimeError(f"Unexpected {modes[0]} in {op} at 0")
            address = params[0]
            value = next(self.inputs)
            self.logger.debug(f"Storing {value} at {address}")
            self.memory[address] = value
        elif op == Op.OUTPUT:
            value = self.get_value(params[0], modes[0])
            self.logger.debug(f"Outputting {value}")
            self.output = value
        elif op.is_jump:
            value = self.get_value(params[0], modes[0])
            address = self.get_value(params[1], modes[1])
            if op.should_jump(value):
                self.logger.debug(f"Jumping to {address}")
                self.ip = address
                ip_offset = 0
        elif op == Op.HALT:
            pass
        else:
            raise RuntimeError("Unexpected Operation {op}")

        self.ip += ip_offset
        return op, params

    def get_value(self, value, mode):
        return self.memory[value] if mode == ValueMode.POSITION else value

    def set_verb(self, verb):
        self.logger.debug(f"Setting verb to {verb}")
        if verb is not None:
            self.memory[2] = verb

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
    def diagnostic_code(self):
        self.output
