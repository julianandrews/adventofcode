import fileinput


MFCSAM_FACTS = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1,
}
MFCSAM_GT = {"cats", "trees"}
MFCSAM_LT = {"pomeranians", "goldfish"}


class SueFacts:
    def __init__(self, number, counts):
        self.number = number
        self.counts = counts

    def __repr__(self):
        return "Sue(%s, %s)" % (self.number, self.counts)

    @classmethod
    def from_string(cls, s):
        num_part, count_part = s.split(": ", 1)
        number = int(num_part[4:])
        counts = {a: int(b) for a, b in [x.split(": ") for x in count_part.split(', ')]}

        return cls(number, counts)


def find_sue(sues, count_matches):
    for sue in sues:
        if all(count_matches(accessory, count) for accessory, count in sue.counts.items()):
            return sue.number


def part1(sues):
    def count_matches(accessory, count):
        return MFCSAM_FACTS[accessory] == count

    return find_sue(sues, count_matches)


def part2(sues):
    def count_matches(accessory, count):
        if accessory in MFCSAM_GT:
            return count > MFCSAM_FACTS[accessory]
        elif accessory in MFCSAM_LT:
            return count < MFCSAM_FACTS[accessory]
        else:
            return MFCSAM_FACTS[accessory] == count

    return find_sue(sues, count_matches)


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    sues = [SueFacts.from_string(line) for line in lines]

    print("Part 1: %s" % part1(sues))
    print("Part 2: %s" % part2(sues))
