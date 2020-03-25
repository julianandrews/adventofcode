extern crate aoc;

use aoc::aoc_error::AOCError;
use aoc::utils::parse_fields;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn is_simple_candidate(n: u64) -> bool {
    let mut adjacent_pair = false;

    let digits = aoc::nums::digits(&n.to_string()).unwrap_or(vec![]);
    for (d1, d2) in digits[..digits.len() - 1].iter().zip(digits[1..].iter()) {
        if d2 < d1 {
            return false;
        }
        if d1 == d2 {
            adjacent_pair = true;
        }
    }

    adjacent_pair
}

fn is_candidate(n: u64) -> bool {
    let mut run_length = 1;
    let mut adjacent_pair = false;

    let digits = aoc::nums::digits(&n.to_string()).unwrap_or(vec![]);
    for (d1, d2) in digits[..digits.len() - 1].iter().zip(digits[1..].iter()) {
        if d2 < d1 {
            return false;
        } else if d1 == d2 {
            run_length += 1;
        } else {
            if run_length == 2 {
                adjacent_pair = true;
            }
            run_length = 1;
        }
    }

    adjacent_pair || run_length == 2
}

fn part1(start: u64, end: u64) -> Result<usize> {
    Ok((start..end + 1).filter(|&n| is_simple_candidate(n)).count())
}

fn part2(start: u64, end: u64) -> Result<usize> {
    Ok((start..end + 1).filter(|&n| is_candidate(n)).count())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let nums = parse_fields(input.trim(), '-')?;
    if nums.len() != 2 {
        Err(AOCError::new("Invalid input"))?;
    }
    let start = nums[0];
    let end = nums[1];

    println!("Part 1: {}", part1(start, end)?);
    println!("Part 2: {}", part2(start, end)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_simple_candidate() {
        assert!(is_simple_candidate(111111));
        assert!(!is_simple_candidate(23450));
        assert!(!is_simple_candidate(123789));
    }

    #[test]
    fn test_is_candidate() {
        assert!(is_candidate(112233));
        assert!(!is_candidate(123444));
        assert!(is_candidate(111122));
    }
}
