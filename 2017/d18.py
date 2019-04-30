import asyncio
import collections
import queue

from utils import read_data


class VirtualMachine:
    def __init__(self, program_id, program, received, sent):
        self.program = program
        self.registers = collections.defaultdict(int)
        self.registers['p'] = program_id
        self.sent = sent
        self.received = received
        self.instruction_pointer = 0
        self.sent_count = 0
        self.waiting = False

    @property
    def stopped(self):
        return (self.waiting and self.received.empty()) or self.done

    @property
    def done(self):
        return not 0 <= self.instruction_pointer < len(self.program)

    async def run(self):
        while not self.done:
            await self.step()

    async def step(self):
        instruction, x, *rest = self.program[self.instruction_pointer].split()
        y = rest[0] if rest else None
        if instruction == "snd":
            self.sent_count += 1
            await self.sent.put(self.get_value(x))
        elif instruction == "set":
            self.registers[x] = self.get_value(y)
        elif instruction == "add":
            self.registers[x] += self.get_value(y)
        elif instruction == "mul":
            self.registers[x] *= self.get_value(y)
        elif instruction == "mod":
            self.registers[x] %= self.get_value(y)
        elif instruction == "rcv":
            self.waiting = True
            while self.received.empty():
                await asyncio.sleep(0)
            self.waiting = False
            self.registers[x] = await self.received.get()
        elif instruction == "jgz":
            if self.get_value(x) > 0:
                self.instruction_pointer += self.get_value(y) - 1
        self.instruction_pointer += 1

    def get_value(self, x):
        return self.registers[x] if x.isalpha() else int(x)


def get_first_rcv(program):
    q1, q2 = asyncio.Queue(), asyncio.Queue()
    vm = VirtualMachine(0, program, q1, q2)

    loop = asyncio.get_event_loop()
    task = loop.create_task(vm.run())

    async def run():
        while not vm.stopped and not task.done():
            await asyncio.sleep(0)

    async def get_result():
        while not vm.sent.empty():
            value = await vm.sent.get()

        return value

    loop.run_until_complete(run())
    task.cancel()
    return loop.run_until_complete(get_result())


def duet(program):
    q1, q2 = asyncio.Queue(), asyncio.Queue()
    vms = [VirtualMachine(0, program, q1, q2), VirtualMachine(1, program, q2, q1)]

    async def run():
        while not all(vm.stopped for vm in vms):
            await asyncio.sleep(0)
        for task in tasks:
            task.cancel()

    loop = asyncio.get_event_loop()
    tasks = [loop.create_task(vm.run()) for vm in vms]
    loop.run_until_complete(run())

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
