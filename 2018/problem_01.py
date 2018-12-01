import itertools

import utils


offsets = list(utils.readints("data/input-01.txt"))

print("Part 1: %d" % sum(offsets))

seen = set()
for frequency in itertools.accumulate(itertools.cycle(offsets)):
    if frequency in seen:
        print("Part 2: %d" % frequency)
        break
    seen.add(frequency)
