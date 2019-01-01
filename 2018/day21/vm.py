import collections


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
        self.paused = False

    def operate(self, op_name, a, b, c):
        op_info = self.OPERATIONS[op_name]
        a_value = self.registers[a] if op_info.a_reg else a
        b_value = self.registers[b] if op_info.b_reg else b

        self.registers[c] = op_info.op(a_value, b_value)

    def run_program(self, program, breakpoints=None, debug=False):
        breakpoints = set(breakpoints if breakpoints is not None else [])
        ip = self.registers[self.ip_register]
        while ip < len(program):
            if ip in breakpoints:
                if self.paused:
                    self.paused = False
                else:
                    self.paused = True
                    return ip
            instruction = program[ip]
            self.registers[self.ip_register] = ip
            original_registers = self.registers[:]
            original_ip = ip
            self.operate(*instruction)
            ip = self.registers[self.ip_register]
            ip += 1
            if debug:
                print("ip=%s %s %s %s" % (
                    original_ip,
                    original_registers,
                    " ".join(map(str, instruction)),
                    self.registers))

    @staticmethod
    def parse_instruction(line):
        op_name, a, b, c = line.split()
        return (op_name, int(a), int(b), int(c))
