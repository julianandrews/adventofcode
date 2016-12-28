from utils import read_data, lines


def abas_in(s):
    abas = set()
    for c1, c2, c3 in zip(s[:-2], s[1:-1], s[2:]):
        if c1 == c3 and c1 != c2:
            abas.add(c1 + c2 + c3)
    return abas


def has_abba(s):
    for c1, c2, c3, c4 in zip(s[:-3], s[1:-2], s[2:-1], s[3:]):
        if c1 == c4 and c2 == c3 and c1 != c2:
            return True
    return False


def get_subsequences(address):
    pairs = address.split('[')
    hypernet_sequences = []
    supernet_sequences = []
    for pair in pairs:
        vals = pair.split(']')
        if len(vals) == 1:
            supernet_sequences.append(vals[0])
        else:
            hypernet_sequences.append(vals[0])
            supernet_sequences.append(vals[1])
    return hypernet_sequences, supernet_sequences


def supports_tls(address):
    hypernet_sequences, supernet_sequences = get_subsequences(address)
    return not any(
        has_abba(x) for x in hypernet_sequences
    ) and any(
        has_abba(x) for x in supernet_sequences
    )


def supports_ssl(address):
    hypernet_sequences, supernet_sequences = get_subsequences(address)
    supernet_abas = set().union(*(abas_in(x) for x in supernet_sequences))
    hypernet_abas = set().union(*(abas_in(x) for x in hypernet_sequences))
    hypernet_babs = set(x[1] + x[0] + x[1] for x in hypernet_abas)
    return bool(hypernet_babs.intersection(supernet_abas))


def tls_count(data):
    return sum(1 for address in lines(data) if supports_tls(address))


def ssl_count(data):
    return sum(1 for address in lines(data) if supports_ssl(address))


if __name__ == '__main__':
    data = read_data(7)

    assert has_abba('bddb')
    assert has_abba('ioxxoj')
    assert not has_abba('zxcvbn')
    assert tls_count(
        """
        abba[mnop]qrst
        abcd[bddb]xyyx
        aaaa[qwer]tyui
        ioxxoj[asdfgh]zxcvbn
        """
    ) == 2
    assert abas_in('zazbz') == {'zaz', 'zbz'}
    assert ssl_count("aba[bab]xyz") == 1
    assert ssl_count("xyx[xyx]xyx") == 0
    assert ssl_count("aaa[kek]eke") == 1
    assert ssl_count("zazbz[bzb]cdb") == 1
    print('All tests passed')

    print(tls_count(data))
    print(ssl_count(data))
