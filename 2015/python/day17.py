import fileinput


def partitions(total, buckets):
    def partitions_recurse(total, min_ix):
        if total == 0:
            yield []
        elif total < 0:
            raise StopIteration
        else:
            for i, bucket in enumerate(buckets[min_ix:]):
                for partition in partitions_recurse(total - bucket, min_ix + i + 1):
                        yield [bucket] + partition

    return partitions_recurse(total, 0)


def part1(buckets):
    parts = list(partitions(150, buckets))

    return len(parts)


def part2(buckets):
    parts = list(partitions(150, buckets))
    min_buckets = min(len(p) for p in parts)

    return sum(1 for p in parts if len(p) == min_buckets)


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    buckets = [int(line) for line in lines]

    print("Part 1: %s" % part1(buckets))
    print("Part 2: %s" % part2(buckets))
