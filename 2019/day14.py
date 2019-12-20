import collections
import math

from utils import get_lines, read_data
from utils.graphs import toposort


Reaction = collections.namedtuple("Reaction", ["inputs", "output"])
Material = collections.namedtuple("Material", ["kind", "quantity"])


def parse_reactions(data):
    def parse_line(s):
        input_string, output_string = s.split(" => ", maxsplit=1)
        return Reaction(
            inputs=[parse_component(c) for c in input_string.split(", ")],
            output=parse_component(output_string)
        )

    def parse_component(c):
        num, kind = c.split(" ", maxsplit=1)
        return Material(kind=kind, quantity=int(num))

    reactions = {}
    for line in get_lines(data):
        reaction = parse_line(line)
        if reaction.output.kind in reactions:
            raise ValueError("Unexpected repeated output")
        reactions[reaction.output.kind] = reaction

    return reactions


def raw_inputs(reactions, target_kind, target_quantity):
    all_kinds = {
        material.kind
        for output in reactions
        for material in reactions[output].inputs
    } | reactions.keys()

    def requirements(kind):
        inputs = reactions[kind].inputs if kind in reactions else []
        return [material.kind for material in inputs]

    needed_materials = collections.defaultdict(int)
    needed_materials[target_kind] = target_quantity

    for kind in toposort(all_kinds, requirements):
        if kind in needed_materials and kind in reactions:
            needed_quantity = needed_materials[kind]
            reaction = reactions[kind]
            multiple = int(math.ceil(float(needed_quantity) / reaction.output.quantity))
            for material in reaction.inputs:
                needed_materials[material.kind] += material.quantity * multiple

    return {
        kind: quantity
        for (kind, quantity) in needed_materials.items()
        if kind not in reactions
    }


def required_ore(reactions, target_kind, target_quantity=1):
    return raw_inputs(reactions, target_kind, target_quantity)["ORE"]


def ore_fuel_yield(reactions, available_ore):
    min_fuel = 0    # Always less than or equal to the fuel yield
    ceiling = None  # Always greater than the fuel yield

    while ceiling is None or ceiling - min_fuel > 1:
        fuel = max(2 * min_fuel, 1) if ceiling is None else (min_fuel + ceiling) // 2
        ore = required_ore(reactions, "FUEL", fuel)
        if ore > available_ore:
            ceiling = fuel
        else:
            min_fuel = fuel

    return min_fuel


def p1(reactions):
    return required_ore(reactions, "FUEL")


def p2(reactions):
    return ore_fuel_yield(reactions, 1000000000000)


def run_tests():
    reactions = parse_reactions("""
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL
    """)
    assert required_ore(reactions, "FUEL") == 165
    assert ore_fuel_yield(reactions, 100) == 0

    reactions = parse_reactions("""
        157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
    """)
    assert required_ore(reactions, "FUEL") == 13312
    assert ore_fuel_yield(reactions, 1000000000000) == 82892753

    reactions = parse_reactions("""
        2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF
    """)
    assert required_ore(reactions, "FUEL") == 180697
    assert ore_fuel_yield(reactions, 1000000000000) == 5586022

    reactions = parse_reactions("""
        171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX
    """)
    assert required_ore(reactions, "FUEL") == 2210736
    assert ore_fuel_yield(reactions, 1000000000000) == 460664


if __name__ == "__main__":
    run_tests()
    print("All tests passed")

    data = read_data(14)
    reactions = parse_reactions(data)
    print("Part 1: {}".format(p1(reactions)))
    print("Part 2: {}".format(p2(reactions)))
