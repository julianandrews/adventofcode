from utils.intcode import VM
from utils import read_data


def always_n(n):
    yield n


def p1(program):
    vm = VM(program, inputs=always_n(1))
    for diagnostic_code in vm.outputs():
        if diagnostic_code:
            return diagnostic_code


def p2(program):
    vm = VM(program, inputs=always_n(5))
    for diagnostic_code in vm.outputs():
        if diagnostic_code:
            return diagnostic_code


if __name__ == "__main__":
    data = read_data(5)
    program = [int(x) for x in data.strip().split(',')]

    print("Part 1: {}".format(p1(program[:])))
    print("Part 2: {}".format(p2(program[:])))
