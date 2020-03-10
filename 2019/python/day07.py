import itertools
from utils import read_data
from utils.intcode import VM, Op


def p1(program):
    best = 0
    for perm in itertools.permutations(list(range(5))):
        signal = 0

        def inputs(phase):
            yield phase
            yield signal

        for phase in perm:
            vm = VM(program[:], inputs=inputs(phase))
            signal = next(vm.outputs())
        best = max(best, signal)

    return best


def p2(program):
    best = 0
    for perm in itertools.permutations(list(range(5, 10))):
        signal = 0
        vms = []

        def inputs(phase):
            yield phase
            while True:
                yield signal

        vms = [VM(program[:], inputs=inputs(phase)) for phase in perm]

        for vm in itertools.cycle(vms):
            try:
                signal = next(vm.outputs())
            except StopIteration:
                best = max(best, vms[-1].output)
                break

    return best


def run_tests():
    pg1_1 = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
    assert p1([int(x) for x in pg1_1.split(',')]) == 43210
    pg1_2 = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
    assert p1([int(x) for x in pg1_2.split(',')]) == 54321
    pg1_3 = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,"\
        "1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
    assert p1([int(x) for x in pg1_3.split(',')]) == 65210
    pg2_1 = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,"\
        "27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
    assert p2([int(x) for x in pg2_1.split(',')]) == 139629729
    pg2_2 = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,"\
        "-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,"\
        "53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
    assert p2([int(x) for x in pg2_2.split(',')]) == 18216


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(7)
    program = [int(x) for x in data.strip().split(',')]

    print("Part 1: {}".format(p1(program[:])))
    print("Part 2: {}".format(p2(program[:])))
