use anyhow::Result;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let pairs: Vec<AssignmentPair> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));

    Ok(())
}

fn part1(pairs: &[AssignmentPair]) -> usize {
    pairs.iter().filter(|pair| pair.fully_overlaps()).count()
}

fn part2(pairs: &[AssignmentPair]) -> usize {
    pairs.iter().filter(|pair| pair.overlaps()).count()
}

#[derive(Debug, Clone)]
struct AssignmentPair(Assignment, Assignment);

impl AssignmentPair {
    fn fully_overlaps(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn overlaps(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

impl std::str::FromStr for AssignmentPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').ok_or(anyhow::anyhow!("Invalid pair"))?;
        Ok(Self(left.parse()?, right.parse()?))
    }
}

#[derive(Debug, Clone)]
struct Assignment(u32, u32);

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }
}

impl std::str::FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('-').ok_or(anyhow::anyhow!("Invalid range"))?;
        Ok(Self(left.parse()?, right.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &[&str] = &[
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];

    #[test]
    fn complete_overlap() {
        let pairs: Vec<AssignmentPair> = TEST_DATA.iter().map(|s| s.parse().unwrap()).collect();
        let overlaps: Vec<bool> = pairs.iter().map(|p| p.fully_overlaps()).collect();

        assert_eq!(&overlaps, &[false, false, false, true, true, false]);
    }

    #[test]
    fn partial_overlap() {
        let pairs: Vec<AssignmentPair> = TEST_DATA.iter().map(|s| s.parse().unwrap()).collect();
        let overlaps: Vec<bool> = pairs.iter().map(|p| p.overlaps()).collect();

        assert_eq!(&overlaps, &[false, false, true, true, true, true]);
    }
}
