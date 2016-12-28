TRAP_PATTERNS = {'^^.', '.^^', '^..', '..^'}


def build_next_row(row):
    extended = '.{}.'.format(row)
    return ''.join(
        '^' if extended[i:i+3] in TRAP_PATTERNS else '.' for i in range(len(row))
    )


def build_map(starting_row, count):
    rows = [starting_row]
    while len(rows) < count:
        rows.append(build_next_row(rows[-1]))
        pass
    return '\n'.join(rows)


def safe_count(starting_row, rows):
    return build_map(starting_row, rows).count('.')


if __name__ == '__main__':
    assert build_map('..^^.', 3) == """
..^^.
.^^^^
^^..^
    """.strip()
    assert build_map('.^^.^.^^^^', 10) == """
.^^.^.^^^^
^^^...^..^
^.^^.^.^^.
..^^...^^^
.^^^^.^^.^
^^..^.^^..
^^^^..^^^.
^..^^^^.^^
.^^^..^.^^
^^.^^^..^^
    """.strip()
    assert safe_count('.^^.^.^^^^', 10) == 38
    print("All tests passed")

    with open('data/d18.txt') as f:
        data = f.read().strip()
    print(safe_count(data, 40))
    print(safe_count(data, 400000))
