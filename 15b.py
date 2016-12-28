from fractions import gcd


def lcm(a, b):
    return (a * b) // gcd(a, b)


def doit(data):
    lines = [line.strip() for line in data.strip().split('\n')]
    lines.append('Disc #7 has 11 positions; at time=0, it is at position 0.')
    t = 0
    jump = 1
    for i, line in enumerate(lines):
        words = line.strip('.').split(' ')
        disk_size = int(words[3])
        initial = int(words[-1])
        while (initial + i + 1 + t) % disk_size != 0:
            t += jump
        jump = lcm(jump, disk_size)
    return t


if __name__ == '__main__':
    print("All tests passed")

    with open('data/d15.txt') as f:
        s = f.read()
    print(doit(s))
