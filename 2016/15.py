from utils import lcm, lines, read_data

from collections import namedtuple


Disk = namedtuple('Disk', ('size', 'initial'))


def disks(data, hard):
    for line in lines(data):
        words = line.strip('.').split(' ')
        yield Disk(int(words[3]), int(words[-1]))
    if hard:
        yield Disk(11, 0)


def drop_time(data, hard=False):
    t = 0
    jump = 1
    for i, disk in enumerate(disks(data, hard)):
        while (disk.initial + i + 1 + t) % disk.size != 0:
            t += jump
        jump = lcm(jump, disk.size)
    return t


if __name__ == '__main__':
    data = read_data(15)
    assert drop_time(
        """
        Disc #1 has 5 positions; at time=0, it is at position 4.
        Disc #2 has 2 positions; at time=0, it is at position 1.
        """
    ) == 5
    print("All tests passed")

    print(drop_time(data))
    print(drop_time(data, hard=True))
