import utils
from utils.intcode import VM


def run_with_inputs(program, noun, verb):
    vm = VM.with_inputs(program[:], noun, verb)
    vm.run()

    return vm.output


def p1(program):
    return run_with_inputs(program, 12, 2)


def p2(program):
    for a in range(99):
        for b in range(99):
            if run_with_inputs(program, a, b) == 19690720:
                return 100 * a + b


if __name__ == "__main__":
    data = utils.read_data(2)
    program = [int(x) for x in data.strip().split(',')]
    test_program = [int(x) for x in "1,9,10,3,2,3,11,0,99,30,40,50".split(",")]
    assert run_with_inputs(test_program, 9, 10) == 3500
    print("Part 1: %d" % p1(program))
    print("Part 2: %d" % p2(program))
