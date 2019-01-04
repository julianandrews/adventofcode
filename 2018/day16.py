import collections
import fileinput


OpInfo = collections.namedtuple("OpInfo", ["op", "a_reg", "b_reg"])


class State:
    OPERATIONS = {
        "addr": OpInfo(int.__add__, True, True),
        "addi": OpInfo(int.__add__, True, False),
        "mulr": OpInfo(int.__mul__, True, True),
        "muli": OpInfo(int.__mul__, True, False),
        "banr": OpInfo(int.__and__, True, True),
        "bani": OpInfo(int.__and__, True, False),
        "borr": OpInfo(int.__or__, True, True),
        "bori": OpInfo(int.__or__, True, False),
        "setr": OpInfo(lambda a, b: a, True, True),
        "seti": OpInfo(lambda a, b: a, False, True),
        "gtir": OpInfo(lambda a, b: 1 if a > b else 0, False, True),
        "gtri": OpInfo(lambda a, b: 1 if a > b else 0, True, False),
        "gtrr": OpInfo(lambda a, b: 1 if a > b else 0, True, True),
        "eqir": OpInfo(lambda a, b: 1 if a == b else 0, False, True),
        "eqri": OpInfo(lambda a, b: 1 if a == b else 0, True, False),
        "eqrr": OpInfo(lambda a, b: 1 if a == b else 0, True, True),
    }

    def __init__(self, registers=None):
        self.registers = list(registers) if registers is not None else [0, 0, 0, 0]

    def operate(self, op_name, a, b, c):
        op_info = self.OPERATIONS[op_name]
        a_value = self.registers[a] if op_info.a_reg else a
        b_value = self.registers[b] if op_info.b_reg else b

        self.registers[c] = op_info.op(a_value, b_value)

    def __eq__(self, other):
        return self.registers == other.registers

    def __repr__(self):
        return "State(%s)" % (self.registers,)


def get_sample_ops(sample_data):
    samples = [chunk.split("\n") for chunk in sample_data.split("\n\n")]

    sample_ops = []
    for sample in samples:
        initial_registers = tuple(int(x) for x in sample[0][9:-1].split(", "))
        op_code, a, b, c = tuple(int(x) for x in sample[1].split())
        final_registers = tuple(int(x) for x in sample[2][9:-1].split(", "))

        expected = State(final_registers)
        potential_ops = []
        for op_name in State.OPERATIONS:
            state = State(initial_registers)
            state.operate(op_name, a, b, c)
            if state == expected:
                potential_ops.append(op_name)
        sample_ops.append((op_code, potential_ops))

    return sample_ops


def run_program(op_map, program):
    state = State()
    for line in program.strip().split("\n"):
        op_code, a, b, c = tuple(int(x) for x in line.split())
        state.operate(op_map[op_code], a, b, c)

    return state


def p1(sample_data):
    sample_ops = get_sample_ops(sample_data)
    return len([n for n, l in sample_ops if len(l) >= 3])


def p2(sample_data, program):
    # Eliminate impossile op_names base on the samples.
    potential_ops = {op_code: set(State.OPERATIONS.keys()) for op_code in range(16)}
    for op_code, op_names in get_sample_ops(sample_data):
        potential_ops[op_code] &= set(op_names)
    op_map = {}

    # Strip out the uniquely determined op_codes until none remain.
    while potential_ops:
        unique_op_codes = [
            (op_code, op_names.pop())
            for op_code, op_names in potential_ops.items()
            if len(op_names) == 1
        ]
        if not unique_op_codes:
            raise ValueError("Insufficient sample data.")
        for op_code, op_name in unique_op_codes:
            try:
                del potential_ops[op_code]
            except KeyError:
                raise ValueError("Inconsistent sample data.")
            op_map[op_code] = op_name
            for op_names in potential_ops.values():
                if op_name in op_names:
                    op_names.remove(op_name)

    if len(op_map) != 16:
        raise ValueError("Insufficient sample data.")

    state = run_program(op_map, program)
    return state.registers[0]


if __name__ == "__main__":
    all_data = "".join(fileinput.input())
    sample_data, program = all_data.split("\n\n\n")
    print(p1(sample_data))
    print(p2(sample_data, program))
