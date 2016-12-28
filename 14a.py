import collections
import hashlib


def get_first_triplet(s):
    for i, c in enumerate(s[:-2]):
        if s[i + 1] == s[i + 2] == c:
            return c
    return None


def get_quints(s):
    for i, c in enumerate(s[:-4]):
        if s[i + 1] == s[i + 2] == s[i + 3] == s[i + 4] == c:
            yield c


def doit(seed):
    digests = {}
    hashes = []
    quints = collections.defaultdict(int)

    def add_digest(i):
        new_digest = hashlib.md5((seed + str(i)).encode()).hexdigest()
        digests[i] = new_digest
        for c in get_quints(new_digest):
            quints[c] = i

    for i in range(1001):
        add_digest(i)

    i = 0
    while len(hashes) < 64:
        digest = digests[i]
        c = get_first_triplet(digest)
        if c is not None and quints[c] > i:
            hashes.append(i)

        add_digest(i + 1000)
        i += 1

    return i - 1


if __name__ == '__main__':
    assert doit('abc') == 22728
    print("All tests passed")

    print(doit('yjdafjpo'))
