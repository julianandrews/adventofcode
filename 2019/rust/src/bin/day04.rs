use anyhow::Result;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let (start, end) = parsing::parse_input(input.trim())?;

    println!("Part 1: {}", part1(start, end)?);
    println!("Part 2: {}", part2(start, end)?);
    Ok(())
}

fn part1(start: u64, end: u64) -> Result<usize> {
    Ok((start..end + 1).filter(|&n| is_simple_candidate(n)).count())
}

fn part2(start: u64, end: u64) -> Result<usize> {
    Ok((start..end + 1).filter(|&n| is_candidate(n)).count())
}

fn is_simple_candidate(n: u64) -> bool {
    let mut adjacent_pair = false;

    let digits = aoc::nums::digits(&n.to_string()).unwrap_or_default();
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

    let digits = aoc::nums::digits(&n.to_string()).unwrap_or_default();
    for (d1, d2) in digits[..digits.len() - 1].iter().zip(digits[1..].iter()) {
        match d2.cmp(d1) {
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Equal => run_length += 1,
            std::cmp::Ordering::Greater => {
                if run_length == 2 {
                    adjacent_pair = true;
                }
                run_length = 1;
            }
        }
    }

    adjacent_pair || run_length == 2
}

mod parsing {
    use anyhow::{anyhow, Result};

    pub fn parse_input(s: &str) -> Result<(u64, u64)> {
        let (start, end) = s
            .split_once('-')
            .ok_or_else(|| anyhow!("Invalid input {}.", s))?;
        Ok((start.parse()?, end.parse()?))
    }
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
