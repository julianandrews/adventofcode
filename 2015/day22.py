import enum
import fileinput

from utils.graphs import bfs


class Spell(enum.Enum):
    MAGIC_MISSILE = 0
    DRAIN = 1
    SHIELD = 2
    POISON = 3
    RECHARGE = 4

    def cost(self):
        return SPELL_COSTS[self]


SPELL_COSTS = {
    Spell.MAGIC_MISSILE: 53,
    Spell.DRAIN: 73,
    Spell.SHIELD: 113,
    Spell.POISON: 173,
    Spell.RECHARGE: 229,
}
MIN_SPELL_COST = min(SPELL_COSTS.values())
EFFECT_DURATIONS = {
    Spell.SHIELD: 6,
    Spell.POISON: 6,
    Spell.RECHARGE: 5,
}


STARTING_HP = 50
STARTING_MP = 500


class State:
    def __init__(self, hp, mp, boss_hp, boss_damage, tick_spell, mana_spent, effects):
        self.hp = hp
        self.mp = mp
        self.boss_hp = boss_hp
        self.boss_damage = boss_damage
        self.effects = effects
        self.mana_spent = mana_spent
        self.tick_spell = tick_spell

    def valid_spells(self):
        # Can recast spells if they'll wear out this turn
        return set(
            spell for spell in Spell
            if self.effects.get(spell, 0) <= 1 and spell.cost() < self.mp
        )

    def process_effects(self):
        for effect in list(self.effects.keys()):
            if effect == Spell.POISON:
                self.boss_hp -= 3
            elif effect == Spell.RECHARGE:
                self.mp += 101
            self.effects[effect] -= 1
            if self.effects[effect] == 0:
                del self.effects[effect]

    def tick(self):
        # Player turn
        self.process_effects()
        if self.boss_hp <= 0:
            return self
        if self.mp < MIN_SPELL_COST:
            self.hp = 0
            return self

        self.mp = self.mp - self.tick_spell.cost()
        self.mana_spent += self.tick_spell.cost()
        if self.tick_spell == Spell.MAGIC_MISSILE:
            self.boss_hp -= 4
        elif self.tick_spell == Spell.DRAIN:
            self.boss_hp -= 2
            self.hp += 2
        else:
            self.effects[self.tick_spell] = EFFECT_DURATIONS[self.tick_spell]

        # Boss turn
        self.process_effects()
        if self.boss_hp > 0:
            armor = 7 if Spell.SHIELD in self.effects else 0
            self.hp -= max(self.boss_damage - armor, 1)

    def next_state(self, spell, hardmode=False):
        new_state = State(
            self.hp,
            self.mp,
            self.boss_hp,
            self.boss_damage,
            spell,
            self.mana_spent,
            dict(self.effects),
        )
        if hardmode:
            new_state.hp -= 1
        if new_state.hp > 0:
            new_state.tick()

        return new_state

    def __repr__(self):
        return "State(%s, %s, %s, %s, %s, %s, %s)" % (
            self.hp,
            self.mp,
            self.boss_hp,
            self.boss_damage,
            self.tick_spell,
            self.mana_spent,
            self.effects,
        )


def run_fight(hp, mp, boss_hp, boss_damage, hardmode=False, debug=False):
    starting_state = State(hp, mp, boss_hp, boss_damage, None, 0, {})

    def neighbors(state):
        if state.hp > 0 and state.boss_hp > 0:
            for spell in state.valid_spells():
                yield state.next_state(spell, hardmode)

    for node in bfs(starting_state, neighbors, lambda s: s.mana_spent):
        state = node.value
        if state.hp > 0 and state.boss_hp <= 0:
            if debug:
                for s in reversed(list(node.get_path())):
                    print(s)
            return state


def part1(boss_hp, boss_damage):
    final_state = run_fight(STARTING_HP, STARTING_MP, boss_hp, boss_damage)

    return final_state.mana_spent


def part2(boss_hp, boss_damage):
    final_state = run_fight(
        STARTING_HP, STARTING_MP, boss_hp, boss_damage, hardmode=True
    )

    return final_state.mana_spent


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    boss_hp, boss_damage = (int(line.split(": ")[1]) for line in lines)

    assert run_fight(10, 250, 13, 8).mana_spent == 226
    assert run_fight(10, 250, 14, 8).mana_spent == 641
    print("All tests passed")

    print("Part 1: %s" % part1(boss_hp, boss_damage))
    print("Part 2: %s" % part2(boss_hp, boss_damage))
