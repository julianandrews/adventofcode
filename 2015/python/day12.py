import fileinput
import json


def int_sum(data):
    if type(data) == list:
        return sum(int_sum(d) for d in data)
    elif type(data) == dict:
        return sum(int_sum(d) for d in data.values())
    elif type(data) == int:
        return data
    else:
        return 0


def no_red_int_sum(data):
    if type(data) == list:
        return sum(no_red_int_sum(d) for d in data)
    elif type(data) == dict:
        if any(value == "red" for value in data.values()):
            return 0
        else:
            return sum(no_red_int_sum(d) for d in data.values())
    elif type(data) == int:
        return data
    else:
        return 0


def part1(data):
    return int_sum(data)


def part2(data):
    return no_red_int_sum(data)


if __name__ == "__main__":
    data = next(fileinput.input()).strip()
    data = json.loads(data)

    print("Part 1: %s" % part1(data))
    print("Part 2: %s" % part2(data))
