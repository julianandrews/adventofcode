import fileinput

from vm import VirtualMachine


def p1(ip_register, program):
    """Register zero is only used in instruction 29. If r5==r0 the program halts.

    The value of r5 the first time we hit instruction 29 is the solution."""

    virtual_machine = VirtualMachine(ip_register)
    virtual_machine.run_program(program, breakpoints={29})

    return virtual_machine.registers[5]


def p2():
    """Reverse engineered code."""
    seen = set()
    r5 = 0
    last = r5
    while True:
        r2 = 65536 | r5
        r5 = 10362650
        while True:
            r5 += r2 & 255
            r5 &= 16777215
            r5 *= 65899
            r5 &= 16777215
            if r2 <= 256:
                if r5 in seen:
                    return last
                last = r5
                seen.add(r5)
                break
            else:
                r2 = r2 // 256

# def p2(ip_register, program):
#     virtual_machine = VirtualMachine(ip_register)
#     seen_states = set()
#     while not tuple(virtual_machine.registers) in seen_states:
#         seen_states.add(tuple(virtual_machine.registers))
#         virtual_machine.run_program(program, breakpoints={29})
#         print(virtual_machine.registers)


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    ip_register = int(lines[0].split()[1])
    program = [VirtualMachine.parse_instruction(line) for line in lines[1:]]
    print(p1(ip_register, program))
    print(p2())
