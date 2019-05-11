def cycle_detect(iterable):
    """Returns the index of the start of a cycle in the iterable, and the
    length of the cycle.

    Values from `iterable` must implement `__hash__` and `__eq__`.
    Fast, but uses O(n) memory.
    """
    seen = {}
    for i, value in enumerate(iterable):
        if value in seen:
            return seen[value], i - seen[value]
        seen[value] = i


def floyd(f, initial):
    """Returns the index of the start of a cycle and the lenfth of the cycle
    from repeated function application.

    Return values of `f` must be valid arguments to `f` and must implement
    `__eq__`.

    This function is slower than cycle_detect, but uses O(1) memory. Prefer
    cycle_detect applied to a generator if memory isn't an issue.
    """
    tortoise = f(initial)
    hare = f(tortoise)
    while tortoise != hare:
        tortoise = f(tortoise)
        hare = f(f(hare))

    cycle_start = 0
    tortoise = initial
    while tortoise != hare:
        tortoise = f(tortoise)
        hare = f(hare)
        cycle_start += 1

    cycle_length = 1
    hare = f(tortoise)
    while tortoise != hare:
        hare = f(hare)
        cycle_length += 1

    return cycle_start, cycle_length
