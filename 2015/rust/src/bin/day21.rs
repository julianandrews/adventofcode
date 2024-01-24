#![feature(try_blocks)]

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let boss: Character = input.trim().parse()?;

    println!("Part 1: {}", part1(&boss)?);
    println!("Part 2: {}", part2(&boss)?);

    Ok(())
}

fn part1(boss: &Character) -> Result<u64> {
    let mut best = None;
    for gear in Equipment::outfits() {
        let hero = Character::with_gear(100, &gear);
        if hero.defeats(boss) {
            best = Some(gear.cost.min(best.unwrap_or(u64::MAX)));
        }
    }
    best.ok_or_else(|| anyhow!("No winning outfit found."))
}

fn part2(boss: &Character) -> Result<u64> {
    let mut best = None;
    for gear in Equipment::outfits() {
        let hero = Character::with_gear(100, &gear);
        if !hero.defeats(boss) {
            best = Some(gear.cost.max(best.unwrap_or(0)));
        }
    }
    best.ok_or_else(|| anyhow!("No losing outfit found."))
}

#[derive(Debug, Clone)]
struct Character {
    hp: u64,
    damage: u64,
    armor: u64,
}

impl Character {
    fn with_gear(hp: u64, gear: &Gear) -> Character {
        Character {
            hp,
            damage: gear.damage,
            armor: gear.armor,
        }
    }

    fn defeats(&self, foe: &Character) -> bool {
        foe.turns_to_die(self.damage) <= self.turns_to_die(foe.damage)
    }

    fn turns_to_die(&self, damage: u64) -> u64 {
        let hit = 1.max(damage.saturating_sub(self.armor));
        (self.hp + hit - 1) / hit
    }
}

#[derive(Debug, Clone, Default)]
struct Gear {
    cost: u64,
    damage: u64,
    armor: u64,
}

impl Gear {
    fn new(cost: u64, damage: u64, armor: u64) -> Gear {
        Gear {
            cost,
            damage,
            armor,
        }
    }
}

impl std::iter::Sum for Gear {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Gear::default(), |a, b| a + b)
    }
}

impl std::ops::Add for Gear {
    type Output = Gear;

    fn add(self, rhs: Gear) -> Gear {
        Gear {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Equipment {
    Dagger,
    Shortsword,
    Warhammer,
    Longsword,
    Greataxe,
    Leather,
    Chainmail,
    Splintmail,
    Bandedmail,
    Platemail,
    DamageRing1,
    DamageRing2,
    DamageRing3,
    DefenseRing1,
    DefenseRing2,
    DefenseRing3,
}

impl Equipment {
    const WEAPONS: [Equipment; 5] = [
        Equipment::Dagger,
        Equipment::Shortsword,
        Equipment::Warhammer,
        Equipment::Longsword,
        Equipment::Greataxe,
    ];

    const ARMOR: [Equipment; 5] = [
        Equipment::Leather,
        Equipment::Chainmail,
        Equipment::Splintmail,
        Equipment::Bandedmail,
        Equipment::Platemail,
    ];

    const RINGS: [Equipment; 6] = [
        Equipment::DamageRing1,
        Equipment::DamageRing2,
        Equipment::DamageRing3,
        Equipment::DefenseRing1,
        Equipment::DefenseRing2,
        Equipment::DefenseRing3,
    ];

    fn outfits() -> impl Iterator<Item = Gear> {
        use itertools::Itertools;

        let weapon_options = Equipment::WEAPONS.iter().combinations(1);
        let armor_options = Equipment::ARMOR
            .iter()
            .combinations(0)
            .chain(Equipment::ARMOR.iter().combinations(1));
        let ring_options = Equipment::RINGS
            .iter()
            .combinations(0)
            .chain(Equipment::RINGS.iter().combinations(1))
            .chain(Equipment::RINGS.iter().combinations(2));
        weapon_options
            .cartesian_product(armor_options)
            .cartesian_product(ring_options)
            .map(|((weapons, armor), rings)| {
                let gear = weapons.into_iter().chain(armor).chain(rings);
                gear.map(|g| g.stats()).sum()
            })
    }

    fn stats(&self) -> Gear {
        match self {
            Equipment::Dagger => Gear::new(8, 4, 0),
            Equipment::Shortsword => Gear::new(10, 5, 0),
            Equipment::Warhammer => Gear::new(25, 6, 0),
            Equipment::Longsword => Gear::new(40, 7, 0),
            Equipment::Greataxe => Gear::new(74, 8, 0),
            Equipment::Leather => Gear::new(13, 0, 1),
            Equipment::Chainmail => Gear::new(31, 0, 2),
            Equipment::Splintmail => Gear::new(53, 0, 3),
            Equipment::Bandedmail => Gear::new(75, 0, 4),
            Equipment::Platemail => Gear::new(102, 0, 5),
            Equipment::DamageRing1 => Gear::new(25, 1, 0),
            Equipment::DamageRing2 => Gear::new(50, 2, 0),
            Equipment::DamageRing3 => Gear::new(100, 3, 0),
            Equipment::DefenseRing1 => Gear::new(20, 0, 1),
            Equipment::DefenseRing2 => Gear::new(40, 0, 2),
            Equipment::DefenseRing3 => Gear::new(80, 0, 3),
        }
    }
}

mod parsing {
    use super::Character;

    impl std::str::FromStr for Character {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Option<_> = try {
                let (hp, rest) = s.split_once('\n')?;
                let (damage, armor) = rest.split_once('\n')?;
                let hp = hp.strip_prefix("Hit Points: ")?;
                let damage = damage.strip_prefix("Damage: ")?;
                let armor = armor.strip_prefix("Armor: ")?;
                (hp, damage, armor)
            };
            let (hp, damage, armor) =
                parts.ok_or_else(|| anyhow::anyhow!("Invalid character\n{}.", s))?;
            Ok(Character {
                hp: hp.parse()?,
                damage: damage.parse()?,
                armor: armor.parse()?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Character;

    #[test]
    fn fight() {
        let hero = Character {
            hp: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Character {
            hp: 12,
            damage: 7,
            armor: 2,
        };

        assert_eq!(hero.turns_to_die(boss.damage), 4);
        assert_eq!(boss.turns_to_die(hero.damage), 4);
        assert!(hero.defeats(&boss));
    }
}
