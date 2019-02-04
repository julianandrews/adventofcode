from utils import read_data
from utils.iterables import cycle_detect, repeat_apply


def redistribute(memory_banks):
    result = list(memory_banks)

    max_value = max(memory_banks)
    max_ix = memory_banks.index(max_value)
    result[max_ix] = 0
    value, remainder = divmod(max_value, len(memory_banks))
    for i in range(len(memory_banks)):
        result[i] += value
    for i in range(1, remainder + 1):
        result[(max_ix + i) % len(memory_banks)] += 1

    return tuple(result)


def steps_to_repeat(memory_banks):
    return sum(cycle_detect(repeat_apply(redistribute, memory_banks)))


def cycle_length(memory_banks):
    return cycle_detect(repeat_apply(redistribute, memory_banks))[1]


def run_tests():
    assert redistribute((0, 2, 7, 0)) == (2, 4, 1, 2)
    assert redistribute((2, 4, 1, 2)) == (3, 1, 2, 3)
    assert redistribute((3, 1, 2, 3)) == (0, 2, 3, 4)
    assert steps_to_repeat((0, 2, 7, 0)) == 5
    assert cycle_length((0, 2, 7, 0)) == 4


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    memory_banks = tuple(int(x) for x in read_data(6).split())
    print("Part 1: {}".format(steps_to_repeat(memory_banks)))
    print("Part 2: {}".format(cycle_length(memory_banks)))
