from utils import read_data
from utils.intcode import VM


def run_commands(program, commands):
    def inputs():
        for c in "\n".join(commands) + "\n":
            yield ord(c)

    vm = VM(program[:], inputs=inputs())
    outputs = list(vm.outputs())
    if outputs[-1] <= 128:
        print("".join(map(chr, outputs[:-1])))
        raise RuntimeError("Failure!")

    # print(''.join(chr(c) for c in outputs[:-1]))
    return outputs[-1]


def p1(program):
    commands = [
        "OR C J",
        "AND A J",
        "NOT J J",
        "AND D J",
        "WALK",
    ]

    return run_commands(program, commands)


def p2(program):
    # !(A & B & C) & D & (E ^ H)
    commands = [
        "NOT A J",
        "NOT J J",
        "AND B J",
        "AND C J",
        "NOT J J",
        "AND D J",
        "NOT E T",
        "NOT T T",
        "OR H T",
        "AND T J",
        "RUN",
    ]

    return run_commands(program, commands)


def run_tests():
    print("No tests run")


if __name__ == "__main__":
    run_tests()

    data = read_data(21)
    program = [int(x) for x in data.strip().split(',')]
    print("Part 1: {}".format(p1(program)))
    print("Part 2: {}".format(p2(program)))
