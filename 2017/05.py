import hashlib
import sys


def write_pa55w0rd(password, filler):
    pa55w0rd = ''.join(
        filler[j] if c is None else
        '\033[1;37m{}\033[0;0m'.format(c) for j, c in enumerate(password)
    )
    sys.stdout.write('\r{}'.format(pa55w0rd))
    sys.stdout.flush()


def crack_simple_password(seed):
    password = ''
    i = 0
    base_hash = hashlib.md5(seed.encode())
    while len(password) < 8:
        updated = base_hash.copy()
        updated.update(str(i).encode())
        hexdigest = updated.hexdigest()
        if hexdigest.startswith('00000'):
            password += hexdigest[5]
        i += 1
    return password


def crack_hard_password(seed, leet=False):
    password = [None] * 8
    count = 0
    i = 0
    base_hash = hashlib.md5(seed.encode())
    while count < 8:
        new_hash = base_hash.copy()
        new_hash.update(str(i).encode())
        hexdigest = new_hash.hexdigest()
        if hexdigest.startswith('00000') and hexdigest[5] in '01234567':
            if password[int(hexdigest[5])] is None:
                password[int(hexdigest[5])] = hexdigest[6]
                count += 1
        if leet and i % 20000 == 0:
            write_pa55w0rd(password, hexdigest)
        i += 1
    if leet:
        write_pa55w0rd(password, hexdigest)
        sys.stdout.write('\n')
    return ''.join(password)


if __name__ == '__main__':
    assert crack_simple_password('abc') == '18f47a30'
    assert crack_hard_password('abc') == '05ace8e3'
    print('All tests passed')

    print(crack_simple_password('cxdnnyjw'))
    crack_hard_password('cxdnnyjw', leet=True)
