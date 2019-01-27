import collections
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


# STARTING_HP = 50
# STARTING_MP = 500
STARTING_HP = 10
STARTING_MP = 250

class State:
    def __init__(self, hp, mp, boss_hp, boss_damage, effects, mana_spent):
        self.hp = hp
        self.mp = mp
        self.boss_hp = boss_hp
        self.boss_damage = boss_damage
        self.effects = effects
        self.mana_spent = mana_spent

    def valid_spells(self):
        return set(spell for spell in Spell if spell not in self.effects)

    def tick(self, spell):
        hp, mp, boss_hp, mana_spent = self.hp, self.mp, self.boss_hp, self.mana_spent
        effects = dict(self.effects)

        def tick_effects():
            nonlocal boss_hp, mp, effects
            for effect in list(effects.keys()):
                if effect == Spell.POISON:
                    boss_hp -= 3
                elif effect == Spell.RECHARGE:
                    mp += 101
                effects[effect] -= 1
                if effects[effect] == 0:
                    del effects[effect]

        # Player turn
        tick_effects()
        if boss_hp <= 0:
            return State(hp, mp, boss_hp, self.boss_damage, effects, mana_spent)
        if mp < MIN_SPELL_COST:
            return State(0, mp, boss_hp, self.boss_damage, effects, mana_spent)

        mp = self.mp - spell.cost()
        mana_spent += spell.cost()
        if spell == Spell.MAGIC_MISSILE:
            boss_hp -= 4
        elif spell == Spell.DRAIN:
            boss_hp -= 2
            hp += 2
        else:
            effects[spell] = EFFECT_DURATIONS[spell]

        # Boss turn
        tick_effects()
        if boss_hp <= 0:
            return State(hp, mp, boss_hp, self.boss_damage, effects, mana_spent)
        hp -= max(self.boss_damage - (7 if Spell.SHIELD in effects else 0), 1)

        return State(hp, mp, boss_hp, self.boss_damage, effects, mana_spent)

    def __repr__(self):
        return "State(%s, %s, %s, %s, %s, %s)" % (self.hp, self.mp, self.boss_hp, self.boss_damage, self.effects, self.mana_spent)


def part1(boss_hp, boss_damage):
    starting_state = State(STARTING_HP, STARTING_MP, boss_hp, boss_damage, {}, 0)

    def neighbors(state):
        if state.hp > 0 and state.boss_hp > 0:
            for spell in state.valid_spells():
                yield state.tick(spell)

    for node in bfs(starting_state, neighbors, lambda s: s.mana_spent):
        state = node.value
        print(node.depth, state)
        if state.hp > 0 and state.boss_hp <= 0:
            return state.mana_spent


def part2():
    pass


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input()]
    boss_hp, boss_damage = (int(line.split(": ")[1]) for line in lines)

    print("Part 1: %s" % part1(boss_hp, boss_damage))
    print("Part 2: %s" % part2())
