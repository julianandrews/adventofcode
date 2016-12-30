from utils import read_data, lines


def swap_position(password, x, y):
    return ''.join(
        password[x] if i == y else (password[y] if i == x else c)
        for i, c in enumerate(password)
    )


def swap_letters(password, a, b):
    return ''.join(a if c == b else (b if c == a else c) for c in password)


def reverse(password, x, y):
    return password[:x] + ''.join(reversed(password[x:y + 1])) + password[y + 1:]


def rotate(password, n, left, unscramble=False):
    if unscramble:
        left = not left
    n = n % len(password)
    if left:
        return password[n:] + password[:n]
    else:
        return password[-n:] + password[:-n]


def move(password, x, y, unscramble=False):
    if unscramble:
        x, y = y, x
    l = list(password)
    c = l[x]
    del l[x]
    return ''.join(l[:y] + [c] + l[y:])


def rotate_on_letter(password, a, unscramble=False):
    i = password.find(a)
    if unscramble:
        assert len(password) % 2 == 0, "Only even length passwords are reliably reversible"
        if i % 2 == 1 or i == 0:
            j = (i - 1) // 2
        else:
            j = (i + len(password) - 2) // 2

        offset = (i - j) % len(password)
        return rotate(password, offset, left=True)
    else:
        offset = (i + (1 if i < 4 else 2)) % len(password)
        return rotate(password, offset, left=False)


def scramble(data, password, unscramble=False):
    instructions = lines(data)
    if unscramble:
        instructions = reversed(instructions)
    for line in instructions:
        words = line.split()
        if line.startswith('swap position'):
            password = swap_position(password, int(words[2]), int(words[5]))
        elif line.startswith('swap letter'):
            password = swap_letters(password, words[2], words[5])
        elif line.startswith('rotate based'):
            password = rotate_on_letter(password, words[-1], unscramble)
        elif line.startswith('rotate'):
            password = rotate(password, int(words[2]), words[1] == 'left', unscramble)
        elif line.startswith('reverse'):
            password = reverse(password, int(words[2]), int(words[4]))
        elif line.startswith('move'):
            password = move(password, int(words[2]), int(words[5]), unscramble)

    return password


if __name__ == '__main__':
    data = read_data(21)
    assert swap_position('abcde', 0, 4) == 'ebcda'
    assert swap_letters('ebcda', 'b', 'd') == 'edcba'
    assert reverse('edcba', 0, 4) == 'abcde'
    assert rotate('abcde', 1, True) == 'bcdea'
    assert rotate('bcdea', 1, True, True) == 'abcde'
    assert move('bcdea', 1, 4) == 'bdeac'
    assert move('bdeac', 1, 4, True) == 'bcdea'
    assert move('bdeac', 3, 0) == 'abdec'
    assert move('abdec', 3, 0, True) == 'bdeac'
    assert rotate_on_letter('abdec', 'b') == 'ecabd'
    assert rotate_on_letter('ecabd', 'd') == 'decab'
    assert rotate_on_letter('abcdefgh', 'e') == 'cdefghab'
    s = 'abcdefgh'
    for c in s:
        rotated = rotate_on_letter(s, c)
        assert rotate_on_letter(rotated, c, True) == s

    test_commands = """
        swap position 4 with position 0
        swap letter d with letter b
        reverse positions 0 through 4
        rotate left 1 step
        move position 1 to position 4
        move position 3 to position 0
        rotate based on position of letter b
        rotate based on position of letter d
    """
    assert scramble(test_commands, 'abcde') == 'decab'
    print("All tests passed")

    print(scramble(data, 'abcdefgh'))
    print(scramble(data, 'fbgdceah', True))
