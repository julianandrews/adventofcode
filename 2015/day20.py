import collections
import fileinput
import functools
import math


def primes_to(n):
    is_prime = [False] * 2 + [True] * (n - 1)
    for i in range(int(n**0.5 + 1.5)):
        if is_prime[i]:
            for j in range(i*i, n+1, i):
                is_prime[j] = False

    return [i for i, prime in enumerate(is_prime) if prime]


def prime_factor_counts(primes, n):
    counts = collections.defaultdict(int)
    for p in primes:
        while n % p == 0:
            counts[p] += 1
            n //= p
        if n == 1:
            return counts


def sum_of_divisors(primes, n):
    counts = prime_factor_counts(primes, n)

    return functools.reduce(
        int.__mul__,
        ((p**(n + 1) - 1) // (p - 1) for p, n in counts.items()),
        1
    )


def part1(n):
    n = n // 10
    primes = primes_to(n)

    for house in range(1, n + 1):
        if sum_of_divisors(primes, house) >= n:
            return house


def part2(n):
    # target = math.ceil(n / 11.0)
    pass


if __name__ == "__main__":
    n = int(next(fileinput.input()).strip())

    # print("Part 1: %s" % part1(n))
    print("Part 2: %s" % part2(n))
