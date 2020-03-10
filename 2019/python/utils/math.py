from fractions import gcd


def dot_product(x, y):
    return sum(a * b for a, b in zip(x, y))


def integer_sqrt(n):
    top = n
    bottom = 0
    x = (top + bottom) // 2
    while top > bottom:
        square = x * x
        if square < n:
            bottom = x + 1
        else:
            top = x
        x = (top + bottom) // 2
    if x * x == n:
        return x


def lcm(a, b):
    return (a * b) // gcd(a, b)


class Quadratic:
    def __init__(self, a, b, c):
        self.a = a
        self.b = b
        self.c = c

    def value(self, x):
        return self.a * x * x + self.b * x + self.c

    def integer_solutions(self):
        a, b, c = self.a, self.b, self.c

        if a == b == c == 0:
            raise ValueError("At least one coefficient must be non-zero.")
        if a == 0:
            if b != 0 and -c % b == 0:
                return [-c // b]
        else:
            determinant = b * b - 4 * a * c
            sqrt = integer_sqrt(determinant)
            if sqrt is not None:
                return [
                    dividend // (2 * a)
                    for dividend in (-b + sqrt, -b - sqrt)
                    if dividend % a == 0
                ]

        return []


def extended_gcd(a, b):
    """return (g, x, y) such that a*x + b*y = g = gcd(a, b)"""
    x0, x1, y0, y1 = 0, 1, 1, 0
    while a != 0:
        (q, a), b = divmod(b, a), a
        y0, y1 = y1, y0 - q * y1
        x0, x1 = x1, x0 - q * x1
    return b, x0, y0


def modinverse(a, b):
    """return x such that (x * a) % b == 1"""
    g, x, _ = extended_gcd(a, b)
    if g != 1:
        raise ValueError('gcd(a, b) != 1')
    return x % b
