from utils import *

try:
    from itertools import zip_longest
except ImportError:
    from itertools import izip_longest as zip_longest


def is_triangle(triple):
    a, b, c = sorted(triple)
    return c < a + b

def parse_in_columns(data):
    return [map(int, line.split()) for line in lines(data)]

def count_row_triangles(data):
    triples = parse_in_columns(data)
    return len(list(filter(is_triangle, triples)))

def count_col_triangles(data):
    triples = parse_in_columns(data)
    flattened = sum((list(x) for x in zip_longest(*triples)), [])
    real_triples = [flattened[i: i + 3] for i in range(0, len(flattened), 3)]
    return len(list(filter(is_triangle, real_triples)))

if __name__ == '__main__':
    data = read_data(3)

    assert count_row_triangles("5 10 25") == 0
    assert count_row_triangles("5 10 12") == 1
    assert count_col_triangles("""
        101 301 501
        102 302 502
        103 303 503
        201 401 601
        202 402 602
        203 403 603
    """) == 6
    print('All tests passed')

    print(count_row_triangles(data))
    print(count_col_triangles(data))
