import collections
import fileinput


def get_sleep_times(data):
  sleep_times = collections.defaultdict(list)
  current_guard = None
  sleep_start = None

  for line in sorted(data):
    minute = int(line[15:17])
    if "begins shift" in line:
      current_guard = int(line.split("#")[1].split()[0])
    elif "falls asleep" in line:
      sleep_start = minute
    elif "wakes up" in line:
      sleep_times[current_guard].append((sleep_start, minute))

  return sleep_times


def p1(data):
  sleep_times = get_sleep_times(data)
  total_sleep_times = {
      guard_id: sum(end - start for (start, end) in foo)
      for guard_id, foo in sleep_times.items()
  }
  sleepiest_guard = max(total_sleep_times, key=total_sleep_times.get)
  sleep_counts = collections.Counter(
      m for m in range(60) for (start, end) in sleep_times[sleepiest_guard]
      if m >= start and m < end)
  return sleepiest_guard * sleep_counts.most_common(1)[0][0]


def p2(data):
  sleep_times = get_sleep_times(data)
  sleep_counts = collections.Counter((m, g) for m in range(60)
                                     for g in sleep_times
                                     for (start, end) in sleep_times[g]
                                     if m >= start and m < end)
  minute, guard = sleep_counts.most_common(1)[0][0]
  return minute * guard


if __name__ == "__main__":
  data = [line.strip() for line in fileinput.input()]
  print("Part 1: %s" % p1(data))
  print("Part 2: %s" % p2(data))
