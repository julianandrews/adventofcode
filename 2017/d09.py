from utils import read_data


def parse_stream(stream):
    i = 0
    depth = 0
    score = 0
    garbage_count = 0
    in_garbage = False
    while i < len(stream):
        c = stream[i]
        if c == "!":
            i += 2
            continue

        if in_garbage:
            if c == ">":
                in_garbage = False
            else:
                garbage_count += 1
        elif c == "<":
            in_garbage = True
        elif c == "{":
            depth += 1
        elif c == "}":
            score += depth
            depth -= 1
        i += 1

    return score, garbage_count


def score(stream):
    return parse_stream(stream)[0]


def garbage_count(stream):
    return parse_stream(stream)[1]


def run_tests():
    assert score("{}") == 1
    assert score("{{{}}}") == 6
    assert score("{{},{}}") == 5
    assert score("{{{},{},{{}}}}") == 16
    assert score("{<{},{},{{}}>}") == 1
    assert score("{<a>,<a>,<a>,<a>}") == 1
    assert score("{{<ab>},{<ab>},{<ab>},{<ab>}}") == 9
    assert score("{{<!!>},{<!!>},{<!!>},{<!!>}}") == 9
    assert score("{{<a!>},{<a!>},{<a!>},{<ab>}}") == 3
    assert garbage_count("<>") == 0
    assert garbage_count("<random characters>") == 17
    assert garbage_count("<<<<>") == 3
    assert garbage_count("<{!>}>") == 2
    assert garbage_count("<!!>") == 0
    assert garbage_count("<!!!>>") == 0
    assert garbage_count("<{o\"i!a,<{i<a>") == 10


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    stream = read_data(9).strip()
    print("Part 1: {}".format(score(stream)))
    print("Part 2: {}".format(garbage_count(stream)))
