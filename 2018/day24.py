import fileinput
import re


class Army:
    def __init__(self, battle_groups):
        self.battle_groups = battle_groups

    def battle(self, other):
        self.reset()
        other.reset()

        friendly_count = self.unit_count()
        enemy_count = other.unit_count()
        while friendly_count and enemy_count:
            targets = sorted(
                self.choose_targets(other) + other.choose_targets(self),
                key=lambda pair: -pair[0].initiative
            )

            for attacker, defender in targets:
                defender.take_damage(attacker)

            new_friendly_count = self.unit_count()
            new_enemy_count = other.unit_count()
            if (new_friendly_count, new_enemy_count) == (friendly_count, enemy_count):
                return None
            friendly_count = new_friendly_count
            enemy_count = new_enemy_count

        return self if friendly_count else other

    def choose_targets(self, defending_army):
        targets = []
        potential_targets = {
            defender for defender in defending_army.battle_groups
            if defender.units
        }
        sorted_battle_groups = sorted(
            self.battle_groups,
            key=lambda group: (-group.effective_power, -group.initiative)
        )
        for attacker in sorted_battle_groups:
            if potential_targets:
                defender = attacker.choose_target(potential_targets)
                if defender is not None:
                    potential_targets.remove(defender)
                    targets.append((attacker, defender))

        return targets

    def reset(self):
        for battle_group in self.battle_groups:
            battle_group.units = battle_group.starting_units

    def unit_count(self):
        return sum(battle_group.units for battle_group in self.battle_groups)


class BattleGroup:
    GROUP_RE = re.compile(
        r"^(?P<units>\d+) units each with (?P<hp>\d+) hit points "
        r"(?:\((?P<info>[^)]+)\) )?"
        r"with an attack that does (?P<damage>\d+) "
        r"(?P<damage_type>.*?) damage at initiative (?P<initiative>\d+)$"
    )

    def __init__(self, string):
        match = self.GROUP_RE.match(string)
        self.units = self.starting_units = int(match.group("units"))
        self.hp = int(match.group("hp"))
        self.starting_damage = int(match.group("damage"))
        self.boost = 0
        self.damage_type = match.group("damage_type")
        self.initiative = int(match.group("initiative"))
        self.weaknesses = set()
        self.immunities = set()
        if match.group("info"):
            for info in match.group("info").split("; "):
                if info.startswith("weak to"):
                    self.weaknesses = set(info[8:].split(", "))
                elif info.startswith("immune to"):
                    self.immunities = set(info[10:].split(", "))

    def get_damage(self, other):
        if self.damage_type in other.immunities:
            return 0
        elif self.damage_type in other.weaknesses:
            return 2 * self.effective_power
        else:
            return self.effective_power

    def take_damage(self, attacker):
        self.units -= min(attacker.get_damage(self) // self.hp, self.units)

    def choose_target(self, others):
        choice = max(
            others,
            key=lambda other: (self.get_damage(other),
                               other.effective_power,
                               other.initiative)
        )
        return choice if self.get_damage(choice) else None

    def unit_count(self):
        return sum(group.units for group in self.groups)

    @property
    def attack_damage(self):
        return self.starting_damage + self.boost

    @property
    def effective_power(self):
        return self.units * self.attack_damage

    @property
    def total_hp(self):
        return self.units * self.hp


def p1(immune_army, infection_army):
    winner = immune_army.battle(infection_army)

    return winner.unit_count()


def p2(immune_army, infection_army):
    min_boost = 0
    max_boost = max(group.total_hp for group in infection_army.battle_groups)
    while True:
        boost = (min_boost + max_boost) // 2
        for group in immune_army.battle_groups:
            group.boost = boost
        immune_army.battle(infection_army)

        if infection_army.unit_count():
            min_boost = boost + 1
        elif boost == max_boost:
            break
        else:
            max_boost = boost

    return immune_army.unit_count()

if __name__ == "__main__":
    immune_part, infection_part = "".join(fileinput.input()).strip().split("\n\n")
    immune_army = Army([
        BattleGroup(line.strip())
        for line in immune_part.split("\n")[1:]
    ])
    infection_army = Army([
        BattleGroup(line.strip())
        for line in infection_part.split("\n")[1:]
    ])
    print("Part 1: %s" % p1(immune_army, infection_army))
    print("Part 2: %s" % p2(immune_army, infection_army))
