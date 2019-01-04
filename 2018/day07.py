import collections
import fileinput
import heapq

from utils.graphs import toposort


MAX_WORKERS = 5
BASE_STEP_TIME = 60


def p1(pairs):
    edges = collections.defaultdict(set)
    for a, b in pairs:
        edges[a].add(b)

    return toposort(
        edges.keys(),
        lambda x: edges[x],
    )


def p2(pairs):
    edges = collections.defaultdict(set)
    indegrees = collections.defaultdict(int)
    for a, b in pairs:
        edges[a].add(b)
        indegrees[b] += 1
    working_nodes = [node for node in edges.keys() if indegrees[node] == 0]
    heapq.heapify(working_nodes)
    tasks = {}

    t = 0
    while working_nodes or tasks:
        for node, end_time in list(tasks.items()):
            if t >= end_time:
                del tasks[node]
                for edge in edges[node]:
                    indegrees[edge] -= 1
                    if indegrees[edge] == 0:
                        heapq.heappush(working_nodes, edge)

        while working_nodes and len(tasks) < MAX_WORKERS:
            node = heapq.heappop(working_nodes)
            tasks[node] = t + BASE_STEP_TIME + 1 + ord(node) - ord("A")
        t += 1

    return t - 1


def parse_line(line):
    words = line.split()
    return (words[1], words[7])


if __name__ == "__main__":
    lines = [parse_line(line) for line in fileinput.input()]
    print("Part 1: %s" % "".join(p1(lines)))
    print("Part 2: %s" % p2(lines))
