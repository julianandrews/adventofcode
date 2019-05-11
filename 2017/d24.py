import collections

from utils import read_data

BridgeStats = collections.namedtuple("BridgeStats", ["length", "strength"])


def best_bridge(pipes, sort_key):
    pipes_by_port = collections.defaultdict(set)
    for pipe in pipes:
        for port in pipe:
            pipes_by_port[port].add(pipe)

    def best_bridge_recurse(open_port):
        values = [BridgeStats(length=0, strength=0)]
        for pipe in pipes_by_port[open_port]:
            for port in pipe:
                pipes_by_port[port].discard(pipe)
            pipe_value = sum(pipe)
            next_open_port = pipe_value - open_port
            length, strength = best_bridge_recurse(next_open_port)
            values.append(
                BridgeStats(length=length + 1, strength=strength + pipe_value)
            )
            for port in pipe:
                pipes_by_port[port].add(pipe)

        return max(values, key=sort_key)

    return best_bridge_recurse(open_port=0)


def max_strength(pipes):
    return best_bridge(pipes, lambda x: x.strength).strength


def max_longest_strength(pipes):
    return best_bridge(pipes, lambda x: x).strength


def parse_lines(lines):
    return frozenset(
        tuple(sorted(int(x) for x in line.split("/")))
        for line in lines
    )


def run_tests():
    pipes = parse_lines(
        ["0/2", "2/2", "2/3", "3/4", "3/5", "0/1", "10/1", "9/10"]
    )

    assert max_strength(pipes) == 31
    assert max_longest_strength(pipes) == 19


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(24)
    pipes = parse_lines(data.strip().split("\n"))
    print("Part 1: {}".format(max_strength(pipes)))
    print("Part 2: {}".format(max_longest_strength(pipes)))
