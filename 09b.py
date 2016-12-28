def decompressed_length(s):
    length = 0
    i = 0
    while i < len(s):
        c = s[i]
        if c == '(':
            j = i + 1
            while True:
                if j <= len(s) and s[j] != ')':
                    j += 1
                    continue
                elif j > len(s):
                    length += len(s[i:])
                    i = j
                else:
                    try:
                        num, count = map(int, s[i + 1: j].split('x'))
                    except ValueError:
                        length += 1
                    else:
                        length += decompressed_length(s[j + 1: j + num + 1]) * count
                        i = j + num + 1
                break
        else:
            length += 1
            i += 1
    return length


if __name__ == '__main__':
    assert decompressed_length('X(8x2)(3x3)ABCY') == len('XABCABCABCABCABCABCY')
    assert decompressed_length('(27x12)(20x12)(13x14)(7x10)(1x12)A') == 241920
    assert decompressed_length('(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN') == 445
    print("All tests passed")

    with open('data/d9.txt') as f:
        s = f.read()
    print(decompressed_length(s.strip()))
