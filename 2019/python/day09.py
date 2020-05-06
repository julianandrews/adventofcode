from utils import read_data
from utils.intcode import VM


def p1(program):
    vm = VM(program[:], inputs=iter((1,)))
    outputs = list(vm.outputs())
    assert len(outputs) == 1
    return outputs[0]


def p2(program):
    vm = VM(program[:], inputs=iter((2,)))
    outputs = list(vm.outputs())
    assert len(outputs) == 1
    return outputs[0]


def run_tests():
    program1 = [
        int(x)
        for x in "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".split(",")
    ]
    assert all(a == b for a, b in zip(program1, VM(program1).outputs()))
    program2 = [int(x) for x in "1102,34915192,34915192,7,4,7,99,0".split(",")]
    assert len(str(next(VM(program2).outputs()))) == 16
    program3 = [int(x) for x in "104,1125899906842624,99".split(",")]
    assert next(VM(program3).outputs()) == 1125899906842624


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data()
    program = [int(x) for x in data.strip().split(',')]
    print("Part 1: {}".format(p1(program)))
    print("Part 2: {}".format(p2(program)))
