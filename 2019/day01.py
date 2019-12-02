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


if __name__ == "__main__":
    data = utils.read_data(1)
    masses = [int(s.strip()) for s in utils.get_lines(data)]
    print("Part 1: %d" % p1(masses))
    print("Part 2: %d" % p2(masses))
