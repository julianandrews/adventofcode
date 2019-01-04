import fileinput


PATTERN_LOOKAROUND = 2


class PotLine:
    def __init__(self, diagram, patterns):
        if 0 in patterns:
            raise ValueError("Crazy patterns!")

        self.offset = 0
        self.state = pattern_to_int(diagram)
        self.patterns = patterns

    def find_cycle(self, max_iterations):
        seen = {}
        for t in range(max_iterations):
            seen[self.state] = t, self.offset
            self.evolve()
            if self.state in seen:
                cycle_start, initial_offset = seen[self.state]
                return t + 1, cycle_start, initial_offset, self.offset

    def evolve(self):
        new_state = 0
        state = self.state << (2 * PATTERN_LOOKAROUND)
        for shift in range(state.bit_length() + 2 * PATTERN_LOOKAROUND):
            pattern = state >> shift & 31
            if pattern in patterns:
                new_state += 1 << shift
        self.offset += PATTERN_LOOKAROUND
        self.state = new_state
        while self.state and not self.state & 1:
            self.state = self.state >> 1
            self.offset -= 1

    def score(self):
        score = 0
        for i in range(self.state.bit_length()):
            if self.state & (1 << i):
                score += i - self.offset

        return score


def get_score(diagram, patterns, iterations):
    pot_line = PotLine(diagram, patterns)
    result = pot_line.find_cycle(iterations)

    if result is not None:
        time, cycle_start, initial_offset, offset = result
        cycles, remainder = divmod(iterations - time, time - cycle_start)
        for i in range(remainder):
            pot_line.evolve()
        pot_line.offset += (offset - initial_offset) * cycles

    return pot_line.score()


def pattern_to_int(pattern):
    return sum(1 << i for (i, c) in enumerate(pattern) if c == "#")


if __name__ == "__main__":
    lines = iter(fileinput.input())
    diagram = next(lines).split(": ")[1]
    patterns = {
        pattern_to_int(line.split()[0])
        for line in lines
        if line.strip().endswith("#")
    }
    print(get_score(diagram, patterns, 20))
    print(get_score(diagram, patterns, 50000000000))
