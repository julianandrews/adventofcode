#![feature(try_blocks)]

use std::collections::BinaryHeap;

use anyhow::{anyhow, Result};
use strum::IntoEnumIterator;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let (boss_hp, boss_damage) = parsing::parse_boss(input.trim())?;

    println!("Part 1: {}", part1(boss_hp, boss_damage)?);
    println!("Part 2: {}", part2(boss_hp, boss_damage)?);

    Ok(())
}

fn part1(boss_hp: u32, boss_damage: u32) -> Result<u32> {
    let start = FightState::new(50, boss_hp, 500);
    let win_state = run_fight(start, boss_damage, false).ok_or(anyhow!("Failed to win fight"))?;
    Ok(win_state.mp_spent)
}

fn part2(boss_hp: u32, boss_damage: u32) -> Result<u32> {
    let start = FightState::new(50, boss_hp, 500);
    let win_state = run_fight(start, boss_damage, true).ok_or(anyhow!("Failed to win fight"))?;
    Ok(win_state.mp_spent)
}

fn run_fight(start: FightState, boss_damage: u32, hard_mode: bool) -> Option<FightState> {
    let mut heap = BinaryHeap::new();
    heap.push(start);

    while let Some(state) = heap.pop() {
        if state.hp > 0 && state.boss_hp == 0 {
            return Some(state);
        }
        for new_state in state.options(boss_damage, hard_mode) {
            heap.push(new_state);
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FightState {
    hp: u32,
    boss_hp: u32,
    mp: u32,
    effects: Effects,
    mp_spent: u32,
}

impl FightState {
    fn new(hp: u32, boss_hp: u32, mp: u32) -> FightState {
        FightState {
            hp,
            boss_hp,
            mp,
            effects: Effects::default(),
            mp_spent: 0,
        }
    }

    fn options(&self, boss_damage: u32, hard_mode: bool) -> impl Iterator<Item = FightState> + '_ {
        Spell::iter().filter_map(move |spell| self.tick(spell, boss_damage, hard_mode))
    }

    fn tick(&self, spell: Spell, boss_damage: u32, hard_mode: bool) -> Option<FightState> {
        let mut state = *self;
        if hard_mode {
            state.hp -= 1;
            if state.hp == 0 {
                return None;
            }
        }

        // Player Turn
        state.process_effects();
        if state.boss_hp == 0 {
            return Some(state);
        }
        if spell.cost() > state.mp || state.effects[spell] != 0 {
            return None;
        }
        state.mp -= spell.cost();
        state.mp_spent += spell.cost();
        match spell {
            Spell::MagicMissile => state.boss_hp = state.boss_hp.saturating_sub(4),
            Spell::Drain => {
                state.boss_hp = state.boss_hp.saturating_sub(2);
                state.hp += 2
            }
            spell => state.effects[spell] = spell.duration(),
        }

        // Boss Turn
        state.process_effects();
        if state.boss_hp == 0 {
            return Some(state);
        }
        let shielded = state.effects[Spell::Shield] > 0;
        let armor = if shielded { 7 } else { 0 };
        let damage = boss_damage.saturating_sub(armor).max(1);
        state.hp = state.hp.saturating_sub(damage);
        if state.hp == 0 {
            return None;
        }

        Some(state)
    }

    fn process_effects(&mut self) {
        if self.effects[Spell::Poison] > 0 {
            self.boss_hp = self.boss_hp.saturating_sub(3);
        }
        if self.effects[Spell::Recharge] > 0 {
            self.mp += 101;
        }
        for spell in Spell::iter() {
            self.effects[spell] = self.effects[spell].saturating_sub(1);
        }
    }
}

impl std::cmp::PartialOrd for FightState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for FightState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.mp_spent.cmp(&self.mp_spent)
    }
}

#[derive(Debug, Clone, Copy, strum_macros::EnumIter)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const COSTS: [u32; 5] = [53, 73, 113, 173, 229];
    const DURATIONS: [u8; 5] = [0, 0, 6, 6, 5];

    fn cost(&self) -> u32 {
        Spell::COSTS[*self as usize]
    }

    fn duration(&self) -> u8 {
        Spell::DURATIONS[*self as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Effects([u8; 5]);

impl std::ops::Index<Spell> for Effects {
    type Output = u8;

    fn index(&self, index: Spell) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl std::ops::IndexMut<Spell> for Effects {
    fn index_mut(&mut self, index: Spell) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

mod parsing {
    pub fn parse_boss(s: &str) -> anyhow::Result<(u32, u32)> {
        let parts: Option<_> = try {
            let (hp, damage) = s.split_once('\n')?;
            let hp = hp.strip_prefix("Hit Points: ")?;
            let damage = damage.strip_prefix("Damage: ")?;
            (hp, damage)
        };
        let (hp, damage) = parts.ok_or_else(|| anyhow::anyhow!("Invalid boss\n{}.", s))?;
        Ok((hp.parse()?, damage.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::{run_fight, Effects, FightState};

    #[test]
    fn fight1() {
        let start = FightState::new(10, 13, 250);
        let boss_damage = 8;
        let result = run_fight(start, boss_damage, false);

        let expected = FightState {
            hp: 2,
            boss_hp: 0,
            mp: 24,
            effects: Effects([0, 0, 0, 3, 0]),
            mp_spent: 226,
        };

        assert_eq!(result, Some(expected));
    }

    #[test]
    fn fight2() {
        let start = FightState::new(10, 14, 250);
        let boss_damage = 8;
        let result = run_fight(start, boss_damage, false);

        let expected = FightState {
            hp: 1,
            boss_hp: 0,
            mp: 114,
            effects: Effects([0, 0, 0, 3, 0]),
            mp_spent: 641,
        };

        assert_eq!(result, Some(expected));
    }
}
