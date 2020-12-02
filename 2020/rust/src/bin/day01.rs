use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};
use itertools::Itertools;
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let entries = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&entries)?);
    println!("Part 2: {}", part2(&entries)?);
    Ok(())
}

fn part1(entries: &[u64]) -> Result<u64> {
    let combination =
        find_combination_with_sum(entries, 2, 2020).ok_or(AOCError::new("No pair found"))?;

    Ok(combination.iter().product())
}

fn part2(entries: &[u64]) -> Result<u64> {
    let combination =
        find_combination_with_sum(entries, 3, 2020).ok_or(AOCError::new("No triple found"))?;

    Ok(combination.iter().product())
}

fn find_combination_with_sum(entries: &[u64], n: usize, sum: u64) -> Option<HashSet<u64>> {
    for combination in entries.iter().combinations(n) {
        if combination.iter().copied().sum::<u64>() == sum {
            return Some(combination.into_iter().copied().collect());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pair() {
        let entries = vec![1721, 979, 366, 299, 675];
        let expected: HashSet<_> = vec![1721, 299].into_iter().collect();

        let result = find_combination_with_sum(&entries, 2, 2020);
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn find_triple() {
        let entries = vec![1721, 979, 366, 299, 675];
        let expected: HashSet<_> = vec![979, 366, 675].into_iter().collect();

        let result = find_combination_with_sum(&entries, 3, 2020);

        assert_eq!(result, Some(expected));
    }
}
