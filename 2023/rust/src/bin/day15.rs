use anyhow::{anyhow, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let sequence: Vec<Step> = parse_fields(input.trim(), ',')?;

    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", focusing_power(&sequence));

    Ok(())
}

fn part1(s: &str) -> u64 {
    s.split(',').map(|s| hash(s) as u64).sum()
}

fn focusing_power(sequence: &[Step]) -> usize {
    let mut map = HASHMAP::new();
    for step in sequence {
        match step {
            Step::Dash(label) => map.remove(label),
            Step::Equals(label, focal_length) => map.insert(label, *focal_length),
        }
    }
    map.focusing_power()
}

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
struct HASHMAP<'a> {
    boxes: [Vec<(&'a str, u8)>; 256],
}

impl<'a> HASHMAP<'a> {
    fn new() -> HASHMAP<'a> {
        HASHMAP {
            boxes: std::array::from_fn(|_| vec![]),
        }
    }

    fn remove(&mut self, label: &str) {
        self.boxes[hash(label) as usize].retain(|(l, _)| *l != label);
    }

    fn insert(&mut self, label: &'a str, focal_length: u8) {
        let b = &mut self.boxes[hash(label) as usize];
        match b.iter_mut().find(|(l, _)| *l == label) {
            Some(entry) => *entry = (label, focal_length),
            None => b.push((label, focal_length)),
        }
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .map(move |(j, (_, f))| (i + 1) * (j + 1) * *f as usize)
            })
            .sum()
    }
}

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |acc, b| acc.wrapping_add(b).wrapping_mul(17))
}

enum Step {
    Dash(String),
    Equals(String, u8),
}

impl std::str::FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_suffix('-') {
            return Ok(Step::Dash(s.to_string()));
        }
        let (label_part, value_part) = s
            .split_once('=')
            .ok_or_else(|| anyhow!("Invalid step: {}", s))?;
        Ok(Step::Equals(label_part.to_string(), value_part.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn simple_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn hash_initialization_sequence() {
        let result: Vec<_> = TEST_DATA.split(',').map(hash).collect();
        let expected = [30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];
        assert_eq!(result, expected);
    }

    #[test]
    fn find_focusing_power() {
        let sequence: Vec<Step> = parse_fields(TEST_DATA, ',').unwrap();
        assert_eq!(focusing_power(&sequence), 145);
    }
}
