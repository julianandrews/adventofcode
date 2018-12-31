import fileinput

from vm import VirtualMachine

def p1(ip_register, program):
    """Register zero is only used in instruction 29. If r5==r0 the program halts.

    The value of r5 the first time we hit instruction 29 is the solution."""

    virtual_machine = VirtualMachine(ip_register)
    virtual_machine.run_program(program, breakpoints={29})

    return virtual_machine.registers[5]


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    ip_register = int(lines[0].split()[1])
    program = [VirtualMachine.parse_instruction(line) for line in lines[1:]]
    print(p1(ip_register, program))
