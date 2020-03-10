import queue
import threading

from utils import read_data
from utils.intcode import VM


class NetworkedVM:
    def __init__(self, program, address, network):
        self.address = address
        self.vm = VM(program, inputs=self.inputs())
        self.ready = threading.Event()
        self.queue = queue.Queue()
        self.network = network
        self.add_message((address, ))
        self.running = False

    def inputs(self):
        while self.running:
            self.ready.wait()
            try:
                values = self.queue.get(block=False)
            except queue.Empty:
                values = (-1, )
                self.ready.clear()
                self.network.nat.notify_idle()
            yield from values

    def add_message(self, message):
        self.queue.put(message)
        self.ready.set()

    def run(self):
        self.running = True
        outputs = self.vm.outputs()
        while self.running:
            try:
                destination = next(outputs)
                x = next(outputs)
                y = next(outputs)
            except StopIteration:
                self.running = False  # Was Network!
                break
            self.network.send_message(self.address, destination, (x, y))

    def stop(self):
        self.running = False
        self.ready.set()


class NAT:
    def __init__(self, network):
        self.network = network
        self.condition = threading.Condition()
        self.last_message = None
        self.running = False

    def notify_idle(self):
        with self.condition:
            self.condition.notify()

    def add_message(self, message):
        self.last_message = message

    def run(self):
        def all_vms_idle():
            idle_vms = [vm for vm in self.network.vms if not vm.ready.is_set()]
            return len(idle_vms) == len(self.network.vms)

        self.running = True
        while self.running:
            with self.condition:
                self.condition.wait_for(lambda: all_vms_idle() or not self.running)
            self.network.send_message(255, 0, self.last_message)

    def stop(self):
        self.running = False
        with self.condition:
            self.condition.notify()


class Network:
    def __init__(self, num_machines, program):
        self.vms = []
        self.nat = NAT(self)
        self.messages_sent = queue.Queue()
        for network_address in range(num_machines):
            self.vms.append(NetworkedVM(program[:], network_address, self))

    def send_message(self, source, destination, message):
        if not 0 <= destination < len(self.vms) and not destination == 255:
            raise RuntimeError(f"Invalid destination: {destination}")
        vm = self.nat if destination == 255 else self.vms[destination]
        self.messages_sent.put((source, destination, message))
        vm.add_message(message)

    def start(self):
        thread = threading.Thread(target=self.nat.run)
        thread.start()
        for vm in self.vms:
            thread = threading.Thread(target=vm.run)
            thread.start()

    def stop(self):
        self.nat.stop()
        for vm in self.vms:
            vm.stop()


def p1(program):
    network = Network(50, program)
    network.start()
    destination = None
    while destination != 255:
        source, destination, (x, y) = network.messages_sent.get()
    network.stop()
    return y


def p2(program):
    network = Network(50, program)
    network.start()
    last_y = None
    while True:
        source, destination, (x, y) = network.messages_sent.get()
        if (source, destination) == (255, 0):
            if y == last_y:
                break
            last_y = y
    network.stop()
    return y


if __name__ == "__main__":
    print("No tests run")

    data = read_data(23)
    program = [int(x) for x in data.strip().split(',')]
    print("Part 1: {}".format(p1(program)))
    print("Part 2: {}".format(p2(program)))
