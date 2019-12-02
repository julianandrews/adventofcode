import utils
from utils.intcode import VM


def run_with_inputs(program, noun=None, verb=None):
    vm = VM(program[:], noun, verb)
    vm.run()

    return vm


def p1(program):
    return run_with_inputs(program, 12, 2).output


def p2(program):
    for a in range(99):
        for b in range(99):
            if run_with_inputs(program, a, b).output == 19690720:
                return 100 * a + b


if __name__ == "__main__":
    # import logging; logging.basicConfig(level=logging.DEBUG)
    data = utils.read_data(2)
    program = [int(x) for x in data.strip().split(',')]
    assert run_with_inputs([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], 9, 10).output == 3500
    assert run_with_inputs([1, 0, 0, 0, 99]).memory == [2, 0, 0, 0, 99]
    assert run_with_inputs([2, 3, 0, 3, 99]).memory == [2, 3, 0, 6, 99]
    assert run_with_inputs([2, 4, 4, 5, 99, 0]).memory == [2, 4, 4, 5, 99, 9801]
    assert run_with_inputs([1, 1, 1, 4, 99, 5, 6, 0, 99]).memory == [30, 1, 1, 4, 2, 5, 6, 0, 99]
    print("Part 1: %d" % p1(program))
    print("Part 2: %d" % p2(program))
