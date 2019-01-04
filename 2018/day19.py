import collections
import fileinput

from utils.vm import VirtualMachine


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
    program = [VirtualMachine.parse_instruction(line) for line in lines[1:]]
    print("Part 1: %s" % p1(ip_register, program))
