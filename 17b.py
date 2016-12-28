import hashlib

try:
    from queue import Queue
except ImportError:
    from Queue import Queue


def open_doors(s, x, y):
    digest = hashlib.md5(s.encode()).hexdigest()
    open_doors = ''.join(d for d, x in zip('UDLR', digest) if x in 'bcdef')
    if x == 0:
        open_doors = open_doors.replace('L', '')
    elif x == 3:
        open_doors = open_doors.replace('R', '')
    if y == 0:
        open_doors = open_doors.replace('U', '')
    elif y == 3:
        open_doors = open_doors.replace('D', '')
    return open_doors


def path(passphrase):
    queue = Queue()
    longest = None
    for door in open_doors(passphrase, 0, 0):
        queue.put((door, 0, 0, ''))

    while not queue.empty():
        door, x, y, path = queue.get()
        path += door
        if door == 'U':
            y -= 1
        elif door == 'D':
            y += 1
        elif door == 'L':
            x -= 1
        elif door == 'R':
            x += 1
        if x == y == 3:
            longest = path
        else:
            for door in open_doors(passphrase + path, x, y):
                queue.put((door, x, y, path))

    return len(longest)


if __name__ == '__main__':
    assert path('ihgpwlah') == 370
    assert path('kglvqrro') == 492
    assert path('ulqzkmiv') == 830
    print('All tests passed')

    print(path('pgflpeqp'))
