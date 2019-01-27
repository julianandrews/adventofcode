from utils import read_data


def decompressed_length(s, recurse=False):
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
                    length += len(s) - i
                    i = j
                else:
                    try:
                        num, count = map(int, s[i + 1: j].split('x'))
                    except ValueError:
                        length += 1
                    else:
                        if recurse:
                            length += decompressed_length(s[j + 1: j + num + 1], True) * count
                        else:
                            length += num * count
                        i = j + num + 1
                break
        else:
            length += 1
            i += 1
    return length


if __name__ == '__main__':
    data = read_data(9).strip()

    assert decompressed_length('ADVENT') == len('ADVENT')
    assert decompressed_length('A(1x5)BC') == len('ABBBBBC')
    assert decompressed_length('(3x3)XYZ') == len('XYZXYZXYZ')
    assert decompressed_length('A(2x2)BCD(2x2)EFG') == len('ABCBCDEFEFG')
    assert decompressed_length('(6x1)(1x3)A') == len('(1x3)A')
    assert decompressed_length('X(8x2)(3x3)ABCY') == len('X(3x3)ABC(3x3)ABCY')
    assert decompressed_length('X(8x2)(3x3)ABCY', True) == len('XABCABCABCABCABCABCY')
    assert decompressed_length('(27x12)(20x12)(13x14)(7x10)(1x12)A', True) == 241920
    assert decompressed_length('(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN', True) == 445
    print('All tests passed')

    print(decompressed_length(data))
    print(decompressed_length(data, True))
