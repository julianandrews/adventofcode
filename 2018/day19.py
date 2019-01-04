import collections
import fileinput


OpInfo = collections.namedtuple("OpInfo", ["op", "a_reg", "b_reg"])


class VirtualMachine:
    OPERATIONS = {
        "addr": OpInfo(int.__add__, True, True),
        "addi": OpInfo(int.__add__, True, False),
        "mulr": OpInfo(int.__mul__, True, True),
        "muli": OpInfo(int.__mul__, True, False),
        "banr": OpInfo(int.__and__, True, True),
        "bani": OpInfo(int.__and__, True, False),
        "borr": OpInfo(int.__or__, True, True),
        "bori": OpInfo(int.__or__, True, False),
        "setr": OpInfo(lambda a, b: a, True, False),
        "seti": OpInfo(lambda a, b: a, False, False),
        "gtir": OpInfo(lambda a, b: 1 if a > b else 0, False, True),
        "gtri": OpInfo(lambda a, b: 1 if a > b else 0, True, False),
        "gtrr": OpInfo(lambda a, b: 1 if a > b else 0, True, True),
        "eqir": OpInfo(lambda a, b: 1 if a == b else 0, False, True),
        "eqri": OpInfo(lambda a, b: 1 if a == b else 0, True, False),
        "eqrr": OpInfo(lambda a, b: 1 if a == b else 0, True, True),
    }

    def __init__(self, ip_register, registers=None):
        self.ip_register = ip_register
        self.registers = registers[:] if registers is not None else [0, 0, 0, 0, 0, 0]

    def operate(self, op_name, a, b, c):
        op_info = self.OPERATIONS[op_name]
        a_value = self.registers[a] if op_info.a_reg else a
        b_value = self.registers[b] if op_info.b_reg else b

        self.registers[c] = op_info.op(a_value, b_value)

    def run_program(self, program):
        ip = self.registers[self.ip_register]
        while ip < len(program):
            instruction = program[ip]
            self.registers[self.ip_register] = ip
            original_registers = self.registers[:]
            original_ip = ip
            self.operate(*instruction)
            ip = self.registers[self.ip_register]
            ip += 1
            # print("ip=%s %s %s %s" % (original_ip, original_registers, " ".join(map(str, instruction)), self.registers))


def parse_instruction(line):
    op_name, a, b, c = line.split()
    return (op_name, int(a), int(b), int(c))


def p1(ip_register, program):
    machine = VirtualMachine(ip_register)
    machine.run_program(program)

    return machine.registers[0]


""" p2

Core Loop:

1  seti 1 4 3	# r3 = 1
2  seti 1 5 1	# r1 = 1
3  mulr 3 1 5	# r5 = r3 * r1
4  eqrr 5 4 5	# if r5 == r4:
5  addr 5 2 2	#	Jump to 7
6  addi 2 1 2	# Jump to 8
7  addr 3 0 0	# r0 += r3
8  addi 1 1 1	# r1++
9  gtrr 1 4 5	# if r1 > r4:
10 addr 2 5 2	# 	Jump to 12
11 seti 2 9 2	# else: Jump to 3
12 addi 3 1 3	# r3++
13 gtrr 3 4 5	# if r3 > r4:
14 addr 5 2 2	# 	Jump to 16
15 seti 1 6 2	# else: Jump to 2
16 mulr 2 2 2	# Jump to 257 (quit)

In psudo-code:

r3 = 1
r1 = 1
while True:
	if r3 * r1 == r4:
		r0 += r3
	r1++
	if r1 > r4:
		r3++
		r1 = 1
		if r3 > r4:
			break

Logically this is:
r0 += sum_of_divisors(r4)

For my input r0 = 0, r4 = 10551394
"""


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    ip_register = int(lines[0].split()[1])
    program = [parse_instruction(line) for line in lines[1:]]
    print("Part 1: %s" % p1(ip_register, program))
