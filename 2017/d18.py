import collections
import queue

from utils import read_data


class VirtualMachine:
    def __init__(self, program_id, program):
        self.program = program
        self.registers = collections.defaultdict(int)
        self.registers['p'] = program_id
        self.waiting = None
        self.sent = queue.Queue()
        self.instruction_pointer = 0
        self.sent_count = 0

    @property
    def stopped(self):
        return (
            self.waiting or
            not 0 <= self.instruction_pointer < len(self.program)
        )

    def accept(self, value):
        self.registers[self.waiting] = value
        self.waiting = False

    def step(self):
        instruction, *args = self.program[self.instruction_pointer].split()
        x = args[0]
        y = args[1] if len(args) > 1 else None
        if instruction == "snd":
            self.sent_count += 1
            self.sent.put(self.get_value(x))
        elif instruction == "set":
            self.registers[x] = self.get_value(y)
        elif instruction == "add":
            self.registers[x] += self.get_value(y)
        elif instruction == "mul":
            self.registers[x] *= self.get_value(y)
        elif instruction == "mod":
            self.registers[x] %= self.get_value(y)
        elif instruction == "rcv":
            self.waiting = x
        elif instruction == "jgz":
            if self.get_value(x) > 0:
                self.instruction_pointer += self.get_value(y) - 1
        self.instruction_pointer += 1

    def get_value(self, x):
        return self.registers[x] if x.isalpha() else int(x)


def get_first_rcv(program):
    vm = VirtualMachine(0, program)
    while vm.waiting is None:
        vm.step()

    while not vm.sent.empty():
        value = vm.sent.get()

    return value


def duet(program):
    vms = [VirtualMachine(0, program), VirtualMachine(1, program)]
    i = 0
    while not all(vm.stopped for vm in vms):
        current = vms[i]
        other = vms[(i + 1) % 2]
        while not current.stopped:
            current.step()
            if current.waiting and not other.sent.empty():
                current.accept(other.sent.get())
        if other.waiting and not current.sent.empty():
            other.accept(current.sent.get())
        i = (i + 1) % 2

    return vms[1].sent_count


def run_tests():
    test_program = [
        "set a 1",
        "add a 2",
        "mul a a",
        "mod a 5",
        "snd a",
        "set a 0",
        "rcv a",
        "jgz a -1",
        "set a 1",
        "jgz a -2",
    ]
    assert get_first_rcv(test_program) == 4
    duet_test_program = [
        "snd 1",
        "snd 2",
        "snd p",
        "rcv a",
        "rcv b",
        "rcv c",
        "rcv d",
    ]
    assert duet(duet_test_program) == 3


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(18)
    program = [line.strip() for line in data.strip().split("\n")]
    print("Part 1: {}".format(get_first_rcv(program)))
    print("Part 2: {}".format(duet(program)))
