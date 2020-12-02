#![feature(str_split_once)]

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let lines: Vec<_> = input.lines().map(parse_line).collect::<Result<_>>()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
    Ok(())
}

fn parse_line(line: &str) -> Result<(PasswordPolicy, &str)> {
    let (policy_part, password) = line
        .split_once(": ")
        .ok_or(AOCError::new("Invalid entry"))?;
    Ok((policy_part.parse()?, password))
}

fn part1(lines: &[(PasswordPolicy, &str)]) -> usize {
    lines
        .iter()
        .filter(|(policy, password)| policy.validate(password))
        .count()
}

fn part2(lines: &[(PasswordPolicy, &str)]) -> usize {
    lines
        .iter()
        .filter(|(policy, password)| policy.really_validate(password))
        .count()
}

struct PasswordPolicy {
    letter: char,
    start: usize,
    end: usize,
}

impl FromStr for PasswordPolicy {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (range_part, char_part) = s.split_once(' ').ok_or(AOCError::new("Invalid policy"))?;
        let letter = if char_part.len() != 1 || !char_part.is_ascii() {
            return Err(AOCError::new("Invalid policy"))?;
        } else {
            char_part.chars().next().unwrap()
        };
        let (start, end) = {
            let (start, end) = range_part
                .split_once('-')
                .ok_or(AOCError::new("Invalid policy"))?;
            (start.parse()?, end.parse()?)
        };

        Ok(PasswordPolicy { letter, start, end })
    }
}

impl PasswordPolicy {
    fn validate(&self, password: &str) -> bool {
        let count = password.chars().filter(|&c| c == self.letter).count();

        count >= self.start && count <= self.end
    }

    fn really_validate(&self, password: &str) -> bool {
        let start = password.chars().nth(self.start - 1);
        let end = password.chars().nth(self.end - 1);

        start != end && (start == Some(self.letter) || end == Some(self.letter))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_1() {
        let (policy, password) = parse_line("1-3 a: abcde").unwrap();
        assert!(policy.validate(password));
    }

    #[test]
    fn validate_2() {
        let (policy, password) = parse_line("1-3 b: cdefg").unwrap();
        assert!(!policy.validate(password));
    }

    #[test]
    fn validate_3() {
        let (policy, password) = parse_line("2-9 c: ccccccccc").unwrap();
        assert!(policy.validate(password));
    }

    #[test]
    fn really_validate_1() {
        let (policy, password) = parse_line("1-3 a: abcde").unwrap();
        assert!(policy.really_validate(password));
    }

    #[test]
    fn really_validate_2() {
        let (policy, password) = parse_line("1-3 b: cdefg").unwrap();
        assert!(!policy.really_validate(password));
    }

    #[test]
    fn really_validate_3() {
        let (policy, password) = parse_line("2-9 c: ccccccccc").unwrap();
        assert!(!policy.really_validate(password));
    }
}
