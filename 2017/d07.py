import collections
import functools

from utils import get_lines, read_data
from utils.graphs import toposort


ProgramInfo = collections.namedtuple("ProgramInfo", ["name", "weight", "children"])


def parse_line(line):
    children = line.split(" -> ")[1].split(", ") if " -> " in line else []
    name, weight_string = line.split(")")[0].split(" (")
    return ProgramInfo(name, int(weight_string), children)


def bottom_program(programs):
    child_map = {program.name: program.children for program in programs}
    return toposort([program.name for program in programs], child_map.get)[0]


def unbalanced_child(programs):
    program_map = {program.name: program for program in programs}

    @functools.lru_cache(maxsize=None)
    def get_weight(program_name):
        program = program_map[program_name]
        return program.weight + sum(map(get_weight, program.children))

    program_name = bottom_program(programs)
    right, wrong = None, None
    while True:
        program = program_map[program_name]
        child_weights = [get_weight(child) for child in program.children]
        if len(set(child_weights)) == 1:
            # We've found the culprit.
            return program.weight + right - wrong
        else:
            counts = collections.Counter(child_weights)
            right, wrong = [weight for weight, count in counts.most_common(2)]
            program_name = [
                child for child in program.children
                if get_weight(child) == wrong
            ][0]


def run_tests():
    test_lines = [
        "pbga (66)",
        "xhth (57)",
        "ebii (61)",
        "havc (66)",
        "ktlj (57)",
        "fwft (72) -> ktlj, cntj, xhth",
        "qoyq (66)",
        "padx (45) -> pbga, havc, qoyq",
        "tknk (41) -> ugml, padx, fwft",
        "jptl (61)",
        "ugml (68) -> gyxo, ebii, jptl",
        "gyxo (61)",
        "cntj (57)",
    ]
    programs = [parse_line(line) for line in test_lines]
    assert bottom_program(programs) == "tknk"
    assert unbalanced_child(programs) == 60


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    programs = [parse_line(line) for line in get_lines(read_data(7))]
    print("Part 1: {}".format(bottom_program(programs)))
    print("Part 2: {}".format(unbalanced_child(programs)))
