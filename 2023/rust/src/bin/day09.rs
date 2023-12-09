use anyhow::Result;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let sequences: Vec<Sequence> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&sequences));
    println!("Part 2: {}", part2(&sequences));

    Ok(())
}

fn part1(sequences: &[Sequence]) -> i64 {
    sequences.iter().map(|sequence| sequence.next()).sum()
}

fn part2(sequences: &[Sequence]) -> i64 {
    sequences.iter().map(|sequence| sequence.previous()).sum()
}

struct Sequence(Vec<i64>);

impl Sequence {
    fn next(&self) -> i64 {
        match self.0.last() {
            Some(n) => n + self.diffs().next(),
            None => 0,
        }
    }

    fn previous(&self) -> i64 {
        match self.0.first() {
            Some(n) => n - self.diffs().previous(),
            None => 0,
        }
    }

    fn diffs(&self) -> Sequence {
        Sequence(self.0.windows(2).map(|pair| pair[1] - pair[0]).collect())
    }
}

impl std::str::FromStr for Sequence {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Sequence(parse_fields(s, ' ')?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45";

    #[test]
    fn forwards() {
        let sequences: Vec<Sequence> = parse_fields(TEST_DATA, '\n').unwrap();
        let result: Vec<_> = sequences.iter().map(|s| s.next()).collect();
        let expected = vec![18, 28, 68];
        assert_eq!(result, expected);
    }

    #[test]
    fn backwards() {
        let sequences: Vec<Sequence> = parse_fields(TEST_DATA, '\n').unwrap();
        let result: Vec<_> = sequences.iter().map(|s| s.previous()).collect();
        let expected = vec![-3, 0, 5];
        assert_eq!(result, expected);
    }
}
