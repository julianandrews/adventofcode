import collections

from utils import read_data


Action = collections.namedtuple("Action", ["value", "offset", "next_state"])


class TuringMachine:
    def __init__(self, rules, state, checksum_time):
        self.tape = set()
        self.cursor = 0
        self.time = 0
        self.rules = rules
        self.state = state
        self.checksum_time = checksum_time

    @staticmethod
    def _parse_action(lines):
        value = int(lines[0].strip().split(" ")[-1][:-1])
        offset = 1 if lines[1].strip().split(" ")[-1] == "right." else -1
        next_state = lines[2].strip().split(" ")[-1][:-1]

        return Action(value, offset, next_state)

    @classmethod
    def from_data(cls, data):
        blocks = [
            [line.strip() for line in block.split("\n")]
            for block in data.strip().split("\n\n")
        ]
        initial_state = blocks[0][0].split(" ")[-1][:-1]
        checksum_time = int(blocks[0][1].split(" ")[-2])

        rules = {}
        for lines in blocks[1:]:
            state = lines[0].split(" ")[-1][:-1]
            rules[state] = {
                0: cls._parse_action(lines[2:5]),
                1: cls._parse_action(lines[6:9]),
            }

        return cls(rules, initial_state, checksum_time)

    def run(self):
        while self.time < self.checksum_time:
            self.tick()

    def tick(self):
        current_value = 1 if self.cursor in self.tape else 0
        action = self.rules[self.state][current_value]
        if action.value:
            self.tape.add(self.cursor)
        else:
            self.tape.discard(self.cursor)
        self.cursor += action.offset
        self.state = action.next_state
        self.time += 1

    def diagnostic_checksum(self):
        return len(self.tape)


def run_tests():
    data = """
        Begin in state A.
        Perform a diagnostic checksum after 6 steps.

        In state A:
          If the current value is 0:
            - Write the value 1.
            - Move one slot to the right.
            - Continue with state B.
          If the current value is 1:
            - Write the value 0.
            - Move one slot to the left.
            - Continue with state B.

        In state B:
          If the current value is 0:
            - Write the value 1.
            - Move one slot to the left.
            - Continue with state A.
          If the current value is 1:
            - Write the value 1.
            - Move one slot to the right.
            - Continue with state A.
    """

    machine = TuringMachine.from_data(data)
    machine.run()

    assert machine.diagnostic_checksum() == 3


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(25)
    machine = TuringMachine.from_data(data)
    machine.run()
    print("Part 1: {}".format(machine.diagnostic_checksum()))
