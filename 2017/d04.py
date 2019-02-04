import collections

from utils import read_data, get_lines


def is_valid(passphrase):
    word_counts = collections.Counter(passphrase.split())
    return all(count == 1 for count in word_counts.values())


def is_secure_valid(passphrase):
    words = passphrase.split()
    # Hashable sorted counts of letters for each word
    frequency_counts = {
        tuple(sorted(collections.Counter(word).items()))
        for word in words
    }

    return len(frequency_counts) == len(words)


def part1(passphrases):
    return sum(1 for passphrase in passphrases if is_valid(passphrase))


def part2(passphrases):
    return sum(1 for passphrase in passphrases if is_secure_valid(passphrase))


def run_tests():
    assert is_valid("aa bb cc dd ee")
    assert not is_valid("aa bb cc dd aa")
    assert is_valid("aa bb cc dd aaa")
    assert is_secure_valid("abcde fghij")
    assert not is_secure_valid("abcde xyz ecdab")
    assert is_secure_valid("a ab abc abd abf abj")
    assert is_secure_valid("iiii oiii ooii oooi oooo")
    assert not is_secure_valid("oiii ioii iioi iiio")


if __name__ == "__main__":
    passphrases = get_lines(read_data(4))
    run_tests()
    print("All tests passed")
    print("Part 1: {}".format(part1(passphrases)))
    print("Part 2: {}".format(part2(passphrases)))
