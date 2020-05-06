from utils.intcode import VM
from utils import read_data


def p1(program):
    vm = VM(program, inputs=iter((1, )))
    for diagnostic_code in vm.outputs():
        if diagnostic_code:
            return diagnostic_code


def p2(program):
    vm = VM(program, inputs=iter((5, )))
    for diagnostic_code in vm.outputs():
        if diagnostic_code:
            return diagnostic_code


def run_tests():
    vm = VM([1002, 4, 3, 4, 33])
    vm.run()
    assert vm.memory == (1002, 4, 3, 4, 99)
    equals8 = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]
    assert next(VM(equals8[:], inputs=iter((8, ))).outputs()) == 1
    assert next(VM(equals8[:], inputs=iter((7, ))).outputs()) == 0
    lt8 = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]
    assert next(VM(lt8[:], inputs=iter((7, ))).outputs()) == 1
    assert next(VM(lt8[:], inputs=iter((8, ))).outputs()) == 0
    equals8im = [3, 3, 1108, -1, 8, 3, 4, 3, 99]
    assert next(VM(equals8im[:], inputs=iter((8, ))).outputs()) == 1
    assert next(VM(equals8im[:], inputs=iter((10, ))).outputs()) == 0
    lt8im = [3, 3, 1107, -1, 8, 3, 4, 3, 99]
    assert next(VM(lt8im[:], inputs=iter((7, ))).outputs()) == 1
    assert next(VM(lt8im[:], inputs=iter((10, ))).outputs()) == 0
    program = [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
        1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
        999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
    ]
    assert next(VM(program[:], inputs=iter((7,))).outputs()) == 999
    assert next(VM(program[:], inputs=iter((8,))).outputs()) == 1000
    assert next(VM(program[:], inputs=iter((10,))).outputs()) == 1001


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data()
    program = [int(x) for x in data.strip().split(',')]

    print("Part 1: {}".format(p1(program[:])))
    print("Part 2: {}".format(p2(program[:])))
