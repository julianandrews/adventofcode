import fileinput
import functools

from summed_area_table import SummedAreaTable


GRID_SIZE = 300


def power_level(x, y, serial_number):
    return (((x + 10) * y + serial_number) * (x + 10)) // 100 % 10 - 5


def p1(table):
    x, y = max(
        ((x, y) for x in range(GRID_SIZE - 3) for y in range(GRID_SIZE - 3)),
        key=lambda p: table.intensity(p[0], p[1], 3, 3)
    )

    return x + 1, y + 1


def p2(table):
    x, y, s = max(
        (
            (x, y, s)
            for s in range(1, GRID_SIZE + 1)
            for x in range(GRID_SIZE - s)
            for y in range(GRID_SIZE - s)
        ),
        key=lambda p: table.intensity(p[0], p[1], p[2], p[2])
    )

    return x + 1, y + 1, s


if __name__ == "__main__":
    serial_number = int(fileinput.input()[0])
    table = SummedAreaTable(
        lambda x, y: power_level(x + 1, y + 1, serial_number),
        GRID_SIZE,
        GRID_SIZE
    )

    print("%s,%s" % p1(table))
    print("%s,%s,%s" % p2(table))
