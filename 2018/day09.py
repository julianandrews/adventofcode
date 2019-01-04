import collections
import fileinput


def p1(num_players, max_marble):
  scores = collections.defaultdict(int)
  circle = collections.deque([0])
  for marble in range(1, max_marble + 1):
    if marble % 23:
      circle.rotate(-1)
      circle.append(marble)
    else:
      circle.rotate(7)
      scores[marble % num_players] += marble + circle.pop()
      circle.rotate(-1)

  return max(scores.values())


if __name__ == '__main__':
  words = fileinput.input()[0].split()
  num_players, max_marble = int(words[0]), int(words[6])
  print(p1(num_players, max_marble))
  print(p1(num_players, max_marble * 100))
