#![feature(iterator_try_collect)]

use anyhow::{anyhow, Result};
use rustc_hash::FxHashMap;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let (left, right) = parse_input(&input)?;

    println!("{}", part1(&left, &right));
    println!("{}", part2(&left, &right));

    Ok(())
}

fn parse_input(input: &str) -> Result<(Vec<u64>, Vec<u64>)> {
    input
        .lines()
        .map(
            |line| match &line.split_whitespace().collect::<Vec<_>>()[..] {
                [l, r] => Ok((l.parse::<u64>()?, r.parse::<u64>()?)),
                _ => Err(anyhow!("Expected exactly two entries on a line")),
            },
        )
        .try_collect()
}

fn part1(left: &[u64], right: &[u64]) -> u64 {
    fn sort(slice: &[u64]) -> Vec<u64> {
        let mut sorted = slice.to_vec();
        sorted.sort_unstable();
        sorted
    }

    let (left, right) = (sort(left), sort(right));
    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part2(left: &[u64], right: &[u64]) -> u64 {
    let mut counts: FxHashMap<u64, u64> = FxHashMap::default();
    for &r in right {
        *counts.entry(r).or_default() += 1;
    }
    left.iter().map(|l| l * *counts.get(l).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        3   4\n\
        4   3\n\
        2   5\n\
        1   3\n\
        3   9\n\
        3   3";

    #[test]
    fn parse() {
        let result = parse_input(TEST_DATA).unwrap();
        let expected = (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(result, expected);
    }

    #[test]
    fn total_distance() {
        let (left, right) = parse_input(TEST_DATA).unwrap();
        assert_eq!(part1(&left, &right), 11);
    }

    #[test]
    fn similarity_score() {
        let (left, right) = parse_input(TEST_DATA).unwrap();
        assert_eq!(part2(&left, &right), 31);
    }
}
