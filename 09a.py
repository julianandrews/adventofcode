def decompress(s):
    output = ""
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
                    output += s[i:]
                    i = j
                else:
                    try:
                        num, count = map(int, s[i + 1: j].split('x'))
                    except ValueError:
                        output += c
                    else:
                        output += s[j + 1: j + num + 1] * count
                        i = j + num + 1
                break
        else:
            output += c
            i += 1
    return output


def doit(s):
    d = decompress(s)
    return len(''.join(d.split()))


if __name__ == '__main__':
    assert decompress('ADVENT') == 'ADVENT'
    assert decompress('A(1x5)BC') == 'ABBBBBC'
    assert decompress('(3x3)XYZ') == 'XYZXYZXYZ'
    assert decompress('A(2x2)BCD(2x2)EFG') == 'ABCBCDEFEFG'
    assert decompress('(6x1)(1x3)A') == '(1x3)A'
    assert decompress('X(8x2)(3x3)ABCY') == 'X(3x3)ABC(3x3)ABCY'
    print('All tests passed')

    with open('data/d9.txt') as f:
        s = f.read()
    print(doit(s))
