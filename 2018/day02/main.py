import collections
import fileinput
import itertools


def p1(data):
  two_counts = 0
  three_counts = 0
  for line in data:
    counts = set(collections.Counter(line).values())
    if 2 in counts:
      two_counts += 1
    if 3 in counts:
      three_counts += 1
  return two_counts * three_counts


def p2(data):
  answers = []
  for (first, second) in itertools.combinations(data, 2):
    shared = [a for a, b in zip(first, second) if a == b]
    if len(shared) == len(first) - 1:
      answers.append("".join(shared))

  assert (len(answers) == 1)
  return answers[0]


if __name__ == "__main__":
  data = [line.strip() for line in fileinput.input()]

  print("Part 1: %d" % p1(data))
  print("Part 2: %s" % p2(data))
