import collections
from functools import lru_cache
import hashlib
import re

from utils import read_data


TRIPLET_RE = re.compile(r'(.)\1\1')
QUINT_RE = re.compile(r'(.)\1\1\1\1')


@lru_cache(1001)
def hash_value(seed, i, stretch):
    value = (seed + str(i))
    for i in range(2017 if stretch else 1):
        value = hashlib.md5(value.encode()).hexdigest()
    return value


def get_pad_length(seed, stretch=False):
    hashes = []

    quints = collections.defaultdict(int)
    def add_quints(i):
        for c in QUINT_RE.findall(hash_value(seed, i, stretch)):
            quints[c] = i
    for i in range(1001):
        add_quints(i)

    i = -1
    while len(hashes) < 64:
        i += 1
        digest = hash_value(seed, i, stretch)
        match = TRIPLET_RE.search(digest)
        c = match.group(1) if match is not None else None
        if c is not None and i < quints[c] <= i + 1000:
            hashes.append(i)

        add_quints(i + 1000)

    return i


if __name__ == '__main__':
    assert get_pad_length('abc') == 22728
    assert hash_value('abc', 0, True) == 'a107ff634856bb300138cac6568c0f24'
    assert get_pad_length('abc', stretch=True) == 22551
    print("All tests passed")

    salt = read_data(14).strip()

    print(get_pad_length(salt))
    print(get_pad_length(salt, stretch=True))
