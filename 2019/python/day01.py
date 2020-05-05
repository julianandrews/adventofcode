import utils


def simple_fuel(mass):
    return max(0, mass // 3 - 2)


def fuel(mass):
    total_mass = 0
    while True:
        mass = simple_fuel(mass)
        total_mass += mass
        if mass == 0:
            return total_mass


def p1(masses):
    return sum(simple_fuel(mass) for mass in masses)


def p2(masses):
    return sum(fuel(mass) for mass in masses)


def run_tests():
    assert simple_fuel(12) == 2
    assert simple_fuel(14) == 2
    assert simple_fuel(1969) == 654
    assert simple_fuel(100756) == 33583

    assert fuel(14) == 2
    assert fuel(1969) == 966
    assert fuel(100756) == 50346


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = utils.read_data()
    masses = [int(s.strip()) for s in utils.get_lines(data)]
    print("Part 1: %d" % p1(masses))
    print("Part 2: %d" % p2(masses))
