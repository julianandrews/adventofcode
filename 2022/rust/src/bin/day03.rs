use anyhow::Result;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let packs: Vec<Pack> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&packs)?);
    println!("Part 2: {}", part2(&packs)?);

    Ok(())
}

fn part1(packs: &[Pack]) -> Result<u32> {
    packs
        .iter()
        .try_fold(0u32, |total, pack| total.checked_add(misplaced_item(pack)?))
        .ok_or_else(|| anyhow::anyhow!("No misplaced item found for a pack"))
}

fn part2(packs: &[Pack]) -> Result<u32> {
    packs
        .chunks(3)
        .try_fold(0u32, |total, packs| total.checked_add(badge(packs)?))
        .ok_or_else(|| anyhow::anyhow!("No badge found for a group"))
}

fn misplaced_item(pack: &Pack) -> Option<u32> {
    pack.left.intersection(&pack.right).only_item()
}

fn badge(packs: &[Pack]) -> Option<u32> {
    packs
        .iter()
        .map(|pack| pack.left.union(&pack.right))
        .reduce(|a, b| a.intersection(&b))?
        .only_item()
}

#[derive(Debug, Clone, Copy)]
struct Pack {
    left: Compartment,
    right: Compartment,
}

impl std::str::FromStr for Pack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            anyhow::bail!("Unequal compartment lengths for {}", s);
        }
        let (left, right) = s.split_at(s.len() / 2);
        Ok(Pack {
            left: left.parse()?,
            right: right.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Compartment(u64);

impl Compartment {
    fn intersection(&self, other: &Compartment) -> Compartment {
        Compartment(self.0 & other.0)
    }

    fn union(&self, other: &Compartment) -> Compartment {
        Compartment(self.0 | other.0)
    }

    fn only_item(&self) -> Option<u32> {
        if self.0.count_ones() != 1 {
            None
        } else {
            Some(self.0.trailing_zeros())
        }
    }
}

impl std::str::FromStr for Compartment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = 0;
        for b in s.bytes() {
            let priority = match b {
                b'a'..=b'z' => b - b'a' + 1,
                b'A'..=b'Z' => b - b'A' + 27,
                _ => anyhow::bail!("Invalid item {}", b),
            };
            value |= 1 << priority;
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &[&str] = &[
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    #[test]
    fn misplaced() {
        let packs: Vec<Pack> = TEST_DATA.iter().map(|s| s.parse().unwrap()).collect();
        println!("{:?}", packs);
        let priorities: Vec<u32> = packs
            .iter()
            .map(|pack| misplaced_item(pack).unwrap())
            .collect();

        assert_eq!(&priorities, &[16, 38, 42, 22, 20, 19]);
    }

    #[test]
    fn badge_1() {
        let packs: Vec<Pack> = TEST_DATA[0..3].iter().map(|s| s.parse().unwrap()).collect();
        let priority = badge(&packs).unwrap();

        assert_eq!(priority, 18);
    }

    #[test]
    fn badge_2() {
        let packs: Vec<Pack> = TEST_DATA[3..6].iter().map(|s| s.parse().unwrap()).collect();
        let priority = badge(&packs).unwrap();

        assert_eq!(priority, 52);
    }
}
