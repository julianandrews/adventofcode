import fileinput


PATTERN_RANGE = 2


def p1(state, patterns, iterations):
    # Buffer for negative pots
    state = state << (iterations * PATTERN_RANGE)
    for i in range(iterations):
        new_state = 0
        for j in range(state.bit_length() + PATTERN_RANGE):
            pattern = state >> j & 31
            if pattern in patterns:
                new_state += 1 << (j + PATTERN_RANGE)
        state = new_state

    return state_score(state, iterations * PATTERN_RANGE)


def state_score(state, offset):
    score = 0
    for i in range(state.bit_length()):
        if state & (1 << i):
            score += i - offset

    return score



def pattern_to_int(pattern):
    return sum(1 << i for (i, c) in enumerate(pattern) if c == "#")


if __name__ == "__main__":
    lines = iter(fileinput.input())
    initial_state = pattern_to_int(next(lines).split(": ")[1])
    patterns = {
        pattern_to_int(line.split()[0])
        for line in lines
        if line.strip().endswith("#")
    }

    print(p1(initial_state, patterns, 20))
