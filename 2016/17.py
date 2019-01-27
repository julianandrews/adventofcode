from utils import bfs, read_data

import hashlib


def move(door, x, y):
    if door == 'L':
        return (x - 1, y)
    elif door == 'R':
        return (x + 1, y)
    elif door == 'U':
        return (x, y + 1)
    elif door == 'D':
        return (x, y - 1)
    assert False


def path(passphrase, longest=False):
    def open_doors(value):
        x, y, path = value
        if x == 3 and y == 0:
            return
        digest = hashlib.md5(bytes(passphrase + path)).hexdigest()
        for door in ''.join(d for d, x in zip('UDLR', digest) if x in 'bcdef'):
            new_x, new_y = move(door, x, y)
            if 0 <= new_x <= 3 and 0 <= new_y <= 3:
                yield (new_x, new_y, path + door)

    best = None
    for node in bfs((0, 3, ''), open_doors):
        x, y, path = node.value
        if x == 3 and y == 0:
            if longest:
                best = path
            else:
                return path
    return best


if __name__ == '__main__':
    assert path('ihgpwlah') == 'DDRRRD'
    assert path('kglvqrro') == 'DDUDRLRRUDRD'
    assert path('ulqzkmiv') == 'DRURDRUDDLLDLUURRDULRLDUUDDDRR'
    assert len(path('ihgpwlah', longest=True)) == 370
    assert len(path('kglvqrro', longest=True)) == 492
    assert len(path('ulqzkmiv', longest=True)) == 830
    print('All tests passed')

    pass_hash = read_data(17).strip()

    print(path(pass_hash))
    print(len(path(pass_hash, longest=True)))
