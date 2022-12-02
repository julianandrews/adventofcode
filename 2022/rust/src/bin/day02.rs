use anyhow::Result;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let rounds: Vec<Round> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&rounds));
    println!("Part 2: {}", part2(&rounds));

    Ok(())
}

fn part1(rounds: &[Round]) -> u32 {
    rounds.iter().map(Round::wrong_score).sum()
}

fn part2(rounds: &[Round]) -> u32 {
    rounds.iter().map(Round::score).sum()
}

#[derive(Debug, Clone, Copy)]
enum Round {
    AX,
    BX,
    CX,
    AY,
    BY,
    CY,
    AZ,
    BZ,
    CZ,
}

impl Round {
    fn wrong_score(&self) -> u32 {
        match self {
            Self::AX => 4,
            Self::BX => 1,
            Self::CX => 7,
            Self::AY => 8,
            Self::BY => 5,
            Self::CY => 2,
            Self::AZ => 3,
            Self::BZ => 9,
            Self::CZ => 6,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::AX => 3,
            Self::BX => 1,
            Self::CX => 2,
            Self::AY => 4,
            Self::BY => 5,
            Self::CY => 6,
            Self::AZ => 8,
            Self::BZ => 9,
            Self::CZ => 7,
        }
    }
}

impl std::str::FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A X" => Ok(Self::AX),
            "B X" => Ok(Self::BX),
            "C X" => Ok(Self::CX),
            "A Y" => Ok(Self::AY),
            "B Y" => Ok(Self::BY),
            "C Y" => Ok(Self::CY),
            "A Z" => Ok(Self::AZ),
            "B Z" => Ok(Self::BZ),
            "C Z" => Ok(Self::CZ),
            _ => Err(anyhow::anyhow!("Invalid round")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = "A Y\nB X\nC Z";

    #[test]
    fn wrong_score() {
        let rounds: Vec<Round> = parse_fields(TEST_DATA, '\n').unwrap();
        let scores: Vec<_> = rounds.iter().map(Round::wrong_score).collect();
        assert_eq!(scores, vec![8, 1, 6]);
    }

    #[test]
    fn score() {
        let rounds: Vec<Round> = parse_fields(TEST_DATA, '\n').unwrap();
        let scores: Vec<_> = rounds.iter().map(Round::score).collect();
        assert_eq!(scores, vec![4, 1, 7]);
    }
}
