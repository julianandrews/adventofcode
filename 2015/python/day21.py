import collections
import enum
import fileinput
import itertools
import math


EquipmentStats = collections.namedtuple("EquipmentStats", ["cost", "damage", "armor"])
Stats = collections.namedtuple("Stats", ["hp", "damage", "armor"])


class Equipment(enum.Enum):
    DAGGER = 0
    SHORTSWORD = 1
    WARHAMMER = 2
    LONGSWORD = 3
    GREATAXE = 4
    LEATHER = 5
    CHAINMAIL = 6
    SPLINTMAIL = 7
    BANDEDMAIL = 7
    PLATEMAIL = 8
    DAMAGE1 = 9
    DAMAGE2 = 10
    DAMAGE3 = 11
    DEFENSE1 = 12
    DEFENSE2 = 13
    DEFENSE3 = 14


EQUIPMENT_STATS = {
    Equipment.DAGGER: EquipmentStats(8, 4, 0),
    Equipment.SHORTSWORD: EquipmentStats(10, 5, 0),
    Equipment.WARHAMMER: EquipmentStats(25, 6, 0),
    Equipment.LONGSWORD: EquipmentStats(40, 7, 0),
    Equipment.GREATAXE: EquipmentStats(74, 8, 0),
    Equipment.LEATHER: EquipmentStats(13, 0, 1),
    Equipment.CHAINMAIL: EquipmentStats(31, 0, 2),
    Equipment.SPLINTMAIL: EquipmentStats(53, 0, 3),
    Equipment.BANDEDMAIL: EquipmentStats(75, 0, 4),
    Equipment.PLATEMAIL: EquipmentStats(102, 0, 5),
    Equipment.DAMAGE1: EquipmentStats(25, 1, 0),
    Equipment.DAMAGE2: EquipmentStats(50, 2, 0),
    Equipment.DAMAGE3: EquipmentStats(100, 3, 0),
    Equipment.DEFENSE1: EquipmentStats(20, 0, 1),
    Equipment.DEFENSE2: EquipmentStats(40, 0, 2),
    Equipment.DEFENSE3: EquipmentStats(80, 0, 3),
}
WEAPONS = {
    Equipment.DAGGER,
    Equipment.SHORTSWORD,
    Equipment.WARHAMMER,
    Equipment.LONGSWORD,
    Equipment.GREATAXE,
}
ARMOR = {
    Equipment.LEATHER,
    Equipment.CHAINMAIL,
    Equipment.SPLINTMAIL,
    Equipment.BANDEDMAIL,
    Equipment.PLATEMAIL,
}
RINGS = {
    Equipment.DAMAGE1,
    Equipment.DAMAGE2,
    Equipment.DAMAGE3,
    Equipment.DEFENSE1,
    Equipment.DEFENSE2,
    Equipment.DEFENSE3,
}
PLAYER_HP = 100


def player_wins(gear_stats, boss_stats):
    turns_to_die = math.ceil(PLAYER_HP / float(max(boss_stats.damage - gear_stats.armor, 1)))
    turns_to_kill = math.ceil(boss_stats.hp / float(max(gear_stats.damage - boss_stats.armor, 1)))
    return turns_to_kill <= turns_to_die


def possible_gear():
    armor_options = list(itertools.chain.from_iterable(
        itertools.combinations(ARMOR, n) for n in (0, 1)
    ))
    ring_options = list(itertools.chain.from_iterable(
        itertools.combinations(RINGS, n) for n in (0, 1, 2)
    ))
    for weapon in WEAPONS:
        for armors in armor_options:
            for rings in ring_options:
                yield tuple(itertools.chain((weapon, ), armors, rings))


def gear_stats(gear):
    return EquipmentStats(*map(sum, zip(*(EQUIPMENT_STATS[item] for item in gear))))


def part1(boss_stats):
    all_stats = [gear_stats(gear) for gear in possible_gear()]
    return min(stats.cost for stats in all_stats if player_wins(stats, boss_stats))


def part2():
    all_stats = [gear_stats(gear) for gear in possible_gear()]
    return max(stats.cost for stats in all_stats if not player_wins(stats, boss_stats))


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    boss_stats = Stats(*(int(line.split(": ")[1]) for line in lines))

    print("Part 1: %s" % part1(boss_stats))
    print("Part 2: %s" % part2())
