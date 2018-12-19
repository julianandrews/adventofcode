import collections
import fileinput
import heapq

MAX_WORKERS = 5
BASE_STEP_TIME = 60


class Graph:

  def __init__(self, pairs):
    self.edges = collections.defaultdict(set)
    self.indegrees = collections.defaultdict(int)
    for (a, b) in pairs:
      self.edges[a].add(b)
      self.indegrees[b] += 1

  def initial_nodes(self):
    return [node for node in self.edges.keys() if self.indegrees[node] == 0]

  def remove_node(self, node):
    for edge in self.edges[node]:
      self.indegrees[edge] -= 1
    del self.edges[node]


def p1(pairs):
  graph = Graph(pairs)
  working_nodes = graph.initial_nodes()
  heapq.heapify(working_nodes)
  sorted_nodes = []

  while working_nodes:
    node = heapq.heappop(working_nodes)
    sorted_nodes.append(node)
    for edge in graph.edges[node]:
      if graph.indegrees[edge] == 1:
        heapq.heappush(working_nodes, edge)
    graph.remove_node(sorted_nodes[-1])

  return sorted_nodes if not graph.edges else None


def p2(pairs):
  graph = Graph(pairs)
  working_nodes = graph.initial_nodes()
  heapq.heapify(working_nodes)
  tasks = {}

  t = 0
  while working_nodes or tasks:
    for node, end_time in list(tasks.items()):
      if t >= end_time:
        del tasks[node]
        for edge in graph.edges[node]:
          if graph.indegrees[edge] == 1:
            heapq.heappush(working_nodes, edge)
        graph.remove_node(node)

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
  print("".join(p1(lines)))
  print(p2(lines))
