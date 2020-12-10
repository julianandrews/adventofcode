use std::collections::HashMap;

use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let adaptors: Vec<usize> = parse_fields(&input.trim(), '\n')?;

    println!("Part 1: {}", part1(&adaptors)?);
    println!("Part 2: {}", part2(&adaptors));
    Ok(())
}

fn part1(adaptors: &[usize]) -> Result<usize> {
    let distribution = joltage_distribution(adaptors).ok_or(AOCError::new("Incomplete chain"))?;
    Ok(distribution.get(&1).unwrap_or(&0) * distribution.get(&3).unwrap_or(&0))
}

fn part2(adaptors: &[usize]) -> usize {
    arrangement_count(adaptors)
}

fn joltage_distribution(adaptors: &[usize]) -> Option<HashMap<usize, usize>> {
    let mut joltages: Vec<_> = std::iter::once(0).chain(adaptors.iter().copied()).collect();
    joltages.sort_unstable();
    joltages.push(joltages.last().expect("joltages always has 0") + 3);

    let mut distribution = HashMap::new();
    for pair in joltages.windows(2) {
        let jump = pair[1] - pair[0];
        if jump > 3 {
            return None;
        }
        *distribution.entry(jump).or_insert(0) += 1;
    }

    Some(distribution)
}

fn arrangement_count(adaptors: &[usize]) -> usize {
    let mut joltages: Vec<_> = std::iter::once(0).chain(adaptors.iter().copied()).collect();
    joltages.sort_unstable();
    joltages.push(joltages.last().expect("joltages always has 0") + 3);

    let mut tail_counts = vec![0; joltages.len()];
    tail_counts[joltages.len() - 1] = 1;
    for i in (0..joltages.len() - 1).rev() {
        tail_counts[i] = (i + 1..joltages.len())
            .take_while(|&j| joltages[j] - joltages[i] <= 3)
            .map(|j| tail_counts[j])
            .sum();
    }

    tail_counts[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_1: [usize; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    static TEST_DATA_2: [usize; 31] = [
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    #[test]
    fn joltage_distribution_1() {
        let result = joltage_distribution(&TEST_DATA_1).unwrap();
        assert_eq!(result.get(&1), Some(&7));
        assert_eq!(result.get(&3), Some(&5));
    }

    #[test]
    fn joltage_distribution_2() {
        let result = joltage_distribution(&TEST_DATA_2).unwrap();
        assert_eq!(result.get(&1), Some(&22));
        assert_eq!(result.get(&3), Some(&10));
    }

    #[test]
    fn arrangement_count_1() {
        let result = arrangement_count(&TEST_DATA_1);
        assert_eq!(result, 8);
    }

    #[test]
    fn arrangement_count_2() {
        let result = arrangement_count(&TEST_DATA_2);
        assert_eq!(result, 19208);
    }
}
