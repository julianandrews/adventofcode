def is_prime(n):
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return n > 1


def prime_factors(n):
    i = 2
    while i <= n:
        while n and n % i == 0:
            yield i
            n //= i
        i += 1


def primes_upto(limit):
    is_prime = [False] * 2 + [True] * (limit - 1)
    for n in range(int(limit**0.5 + 1.5)):
        if is_prime[n]:
            for i in range(n * n, limit + 1, n):
                is_prime[i] = False
    for i in range(limit + 1):
        if is_prime[i]:
            yield i
