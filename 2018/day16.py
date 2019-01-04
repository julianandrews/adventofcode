import collections
import fileinput


from utils.vm import VirtualMachine


def get_sample_ops(sample_data):
    samples = [chunk.split("\n") for chunk in sample_data.split("\n\n")]

    sample_ops = []
    for sample in samples:
        initial_registers = [int(x) for x in sample[0][9:-1].split(", ")]
        op_code, a, b, c = tuple(int(x) for x in sample[1].split())
        final_registers = [int(x) for x in sample[2][9:-1].split(", ")]

        expected = VirtualMachine(0, final_registers)
        potential_ops = []
        for op_name in VirtualMachine.OPERATIONS:
            state = VirtualMachine(0, initial_registers)
            state.operate(op_name, a, b, c)
            if state == expected:
                potential_ops.append(op_name)
        sample_ops.append((op_code, potential_ops))

    return sample_ops


def run_program(op_map, program):
    state = VirtualMachine(0)
    for line in program.strip().split("\n"):
        op_code, a, b, c = tuple(int(x) for x in line.split())
        state.operate(op_map[op_code], a, b, c)

    return state


def p1(sample_data):
    sample_ops = get_sample_ops(sample_data)
    return len([n for n, l in sample_ops if len(l) >= 3])


def p2(sample_data, program):
    # Eliminate impossile op_names base on the samples.
    potential_ops = {
        op_code: set(VirtualMachine.OPERATIONS.keys()) for op_code in range(16)
    }
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
    print("Part 1: %s" % p1(sample_data))
    print("Part 2: %s" % p2(sample_data, program))
