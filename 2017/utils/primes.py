def prime_factors(n):
    i = 2
    while i <= n:
        while n and n % i == 0:
            yield i
            n //= i
        i += 1
