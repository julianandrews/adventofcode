import enum
import logging


class ValueMode(enum.Enum):
    POSITION = 0
    IMMEDIATE = 1
    RELATIVE = 2


class OpType(enum.Enum):
    ADD = 1
    MULTIPLY = 2
    STORE = 3
    OUTPUT = 4
    JUMP_IF_TRUE = 5
    JUMP_IF_FALSE = 6
    LESS_THAN = 7
    EQUALS = 8
    ADJUST_REL_OFFSET = 9
    HALT = 99

    @property
    def num_params(self):
        if self in {OpType.ADD, OpType.MULTIPLY, OpType.LESS_THAN, OpType.EQUALS}:
            return 3
        elif self in {OpType.JUMP_IF_TRUE, OpType.JUMP_IF_FALSE}:
            return 2
        elif self in {OpType.STORE, OpType.OUTPUT, OpType.ADJUST_REL_OFFSET}:
            return 1
        elif self == OpType.HALT:
            return 0
        else:
            raise RuntimeError(f"Unexpected Operation {self}")


class VM:
    logger = logging.getLogger(__name__)

    def __init__(self, memory, inputs=None):
        self.logger.debug(f"Initializing VM")
        self.inputs = inputs if inputs is not None else iter([])
        self._memory = VM.VMMemory(memory)
        self._ip = 0
        self._relative_base = 0

    def outputs(self):
        while True:
            try:
                output = self._step()
            except StopIteration:
                break
            if output is not None:
                yield output

    def run(self):
        while True:
            try:
                self._step()
            except StopIteration:
                break

    @property
    def memory(self):
        return tuple(self._memory[:])

    def _step(self):
        op_type = self._get_op_type()
        params = self._get_params()
        modes = self._get_modes()

        self.logger.debug(f"Executing {op_type} with {params} and {modes}")

        if op_type == OpType.ADD:
            self._add(params, modes)
        elif op_type == OpType.MULTIPLY:
            self._multiply(params, modes)
        elif op_type == OpType.LESS_THAN:
            self._less_than(params, modes)
        elif op_type == OpType.EQUALS:
            self._equals(params, modes)
        elif op_type == OpType.STORE:
            self._store(params, modes)  # raises StopIteration if inputs exhausted
        elif op_type == OpType.OUTPUT:
            return self._output(params, modes)
        elif op_type == OpType.JUMP_IF_TRUE:
            self._jump_if_true(params, modes)
        elif op_type == OpType.JUMP_IF_FALSE:
            self._jump_if_false(params, modes)
        elif op_type == OpType.ADJUST_REL_OFFSET:
            self._adjust_rel_offset(params, modes)
        elif op_type == OpType.HALT:
            raise StopIteration
        else:
            raise RuntimeError("Unexpected Operation {op_type}")

        return None

    def _get_op_type(self):
        return OpType(self._memory[self._ip] % 100)

    def _get_params(self):
        op_type = self._get_op_type()
        return self._memory[self._ip + 1:self._ip + op_type.num_params + 1]

    def _get_modes(self):
        op_type = self._get_op_type()
        mode_str = (str(self._memory[self._ip] // 100) or '').zfill(op_type.num_params)
        return [ValueMode(int(c)) for c in reversed(mode_str)]

    def _get_binary_operands(self, params, modes):
        a = self._get_value(params[0], modes[0])
        b = self._get_value(params[1], modes[1])
        if modes[2] == ValueMode.IMMEDIATE:
            raise RuntimeError(f"Invalid {modes[2]} for address in binary op")
        address = self._get_address(params[2], modes[2])
        return a, b, address

    def _get_jump_operands(self, params, modes):
        value = self._get_value(params[0], modes[0])
        address = self._get_value(params[1], modes[1])
        return value, address

    def _add(self, params, modes):
        a, b, address = self._get_binary_operands(params, modes)
        self.logger.debug(f"Storing {a} + {b} at {address}")
        self._memory[address] = a + b
        self._ip += len(params) + 1

    def _multiply(self, params, modes):
        a, b, address = self._get_binary_operands(params, modes)
        self.logger.debug(f"Storing {a} * {b} at {address}")
        self._memory[address] = a * b
        self._ip += len(params) + 1

    def _less_than(self, params, modes):
        a, b, address = self._get_binary_operands(params, modes)
        self.logger.debug(f"Storing {a} < {b} at {address}")
        self._memory[address] = 1 if a < b else 0
        self._ip += len(params) + 1

    def _equals(self, params, modes):
        a, b, address = self._get_binary_operands(params, modes)
        self.logger.debug(f"Storing {a} == {b} at {address}")
        self._memory[address] = 1 if a == b else 0
        self._ip += len(params) + 1

    def _store(self, params, modes):
        if modes[0] == ValueMode.IMMEDIATE:
            raise RuntimeError(f"Invalid {modes[0]} for address in store")
        address = self._get_address(params[0], modes[0])
        value = next(self.inputs)
        self.logger.debug(f"Storing {value} at {address}")
        self._memory[address] = value
        self._ip += len(params) + 1

    def _output(self, params, modes):
        value = self._get_value(params[0], modes[0])
        self.logger.debug(f"Outputting {value}")
        self._ip += len(params) + 1
        return value

    def _adjust_rel_offset(self, params, modes):
        value = self._get_value(params[0], modes[0])
        self.logger.debug(f"Adjusting relative base by {value}")
        self._relative_base += value
        self._ip += len(params) + 1

    def _jump_if_true(self, params, modes):
        value, address = self._get_jump_operands(params, modes)
        if value:
            self.logger.debug(f"Jumping to {address}")
            self._ip = address
        else:
            self._ip += len(params) + 1

    def _jump_if_false(self, params, modes):
        value, address = self._get_jump_operands(params, modes)
        if not value:
            self.logger.debug(f"Jumping to {address}")
            self._ip = address
        else:
            self._ip += len(params) + 1

    def _get_value(self, value, mode):
        if mode == ValueMode.POSITION:
            return self._memory[value]
        elif mode == ValueMode.IMMEDIATE:
            return value
        elif mode == ValueMode.RELATIVE:
            return self._memory[self._relative_base + value]
        else:
            raise RuntimeError("Unrecognized ValueMode {mode}")

    def _get_address(self, base_address, mode):
        if mode == ValueMode.POSITION:
            return base_address
        else:
            return base_address + self._relative_base

    class VMMemory:
        def __init__(self, memory):
            self.memory = memory

        def __getitem__(self, index):
            if isinstance(index, slice):
                start = index.start if index.start is not None else 0
                stop = index.stop if index.stop is not None else len(self.memory)
                step = index.step if index.step is not None else 1
                return [
                    self.memory[i] if i < len(self.memory) else 0
                    for i in range(start, stop, step)
                ]
            else:
                return self.memory[index] if index < len(self.memory) else 0

        def __setitem__(self, index, value):
            if index >= len(self.memory):
                self.memory.extend(0 for _ in range(index - len(self.memory) + 1))
            self.memory[index] = value
