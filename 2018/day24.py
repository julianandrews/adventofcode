import fileinput
import re


class Group:
    GROUP_RE = re.compile(
        r"^(?P<units>\d+) units each with (?P<hp>\d+) hit points "
        r"(?:\((?P<info>[^)]+)\) )?"
        r"with an attack that does (?P<damage>\d+) "
        r"(?P<damage_type>.*?) damage at initiative (?P<initiative>\d+)$"
    )

    def __init__(self, line):
        match = self.GROUP_RE.match(line)
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
        units_lost = min(attacker.get_damage(self) // self.hp, self.units)
        self.units -= units_lost

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

    # def __str__(self):
    #     return "%s units each with %s hit points (immune to %s; weak to %s) with an attack that does %s %s damage at initiative %s" % (self.units, self.hp, ", ".join(self.immunities), ", ".join(self.weaknesses), self.attack_damage, self.damage_type, self.initiative)


def choose_targets(attackers, defenders):
    targets = []
    potential_targets = {defender for defender in defenders if defender.units}
    for attacker in sorted(attackers, key=lambda group: (-group.effective_power, -group.initiative)):
        if potential_targets:
            defender = attacker.choose_target(potential_targets)
            if defender is not None:
                potential_targets.remove(defender)
                targets.append((attacker, defender))

    return targets


def unit_count(army):
    return sum(group.units for group in army)


def run_battle(immune_army, infection_army):
    for group in immune_army + infection_army:
        group.units = group.starting_units
    immune_count = unit_count(immune_army)
    infection_count = unit_count(infection_army)
    while immune_count and infection_count:
        immune_targets = choose_targets(immune_army, infection_army)
        infection_targets = choose_targets(infection_army, immune_army)
        targets = sorted(
            immune_targets + infection_targets,
            key=lambda pair: -pair[0].initiative
        )

        for attacker, defender in targets:
            defender.take_damage(attacker)

        new_immune_count = unit_count(immune_army)
        new_infection_count = unit_count(infection_army)
        if (new_immune_count, new_infection_count) == (immune_count, infection_count):
            break
        immune_count = new_immune_count
        infection_count = new_infection_count


def p1(immune_army, infection_army):
    run_battle(immune_army, infection_army)
    winner = immune_army if unit_count(immune_army) else infection_army

    return unit_count(winner)


def p2(immune_army, infection_army):
    min_boost = 0
    max_boost = max(group.total_hp for group in infection_army)
    while True:
        boost = (min_boost + max_boost) // 2
        for group in immune_army:
            group.boost = boost
        run_battle(immune_army, infection_army)

        if unit_count(infection_army):
            min_boost = boost + 1
        elif boost == max_boost:
            break
        else:
            max_boost = boost

    return unit_count(immune_army)

if __name__ == "__main__":
    immune_part, infection_part = "".join(fileinput.input()).split("\n\n")
    immune_army = [Group(line.strip()) for line in immune_part.split("\n")[1:] if line]
    infection_army = [Group(line.strip()) for line in infection_part.split("\n")[1:] if line]
    print("Part 1: %s" % p1(immune_army, infection_army))
    print("Part 2: %s" % p2(immune_army, infection_army))
