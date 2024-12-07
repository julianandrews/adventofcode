use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let equations: Vec<Equation> = aoc::utils::parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&equations));
    println!("Part 2: {}", part2(&equations));

    Ok(())
}

fn part1(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|eq| eq.wrong_is_valid())
        .map(|eq| eq.test_value)
        .sum()
}

fn part2(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|eq| eq.is_valid())
        .map(|eq| eq.test_value)
        .sum()
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn wrong_is_valid(&self) -> bool {
        let operator_count = self.numbers.len() - 1;
        let it = (0..operator_count)
            .map(|_| [Op::Add, Op::Multiply].into_iter())
            .multi_cartesian_product();
        for ops in it {
            let mut value = self.numbers[0];
            for (n, op) in self.numbers.iter().skip(1).zip(ops) {
                match op {
                    Op::Add => value += n,
                    Op::Multiply => value *= n,
                    _ => unreachable!(),
                }
            }
            if value == self.test_value {
                return true;
            }
        }
        false
    }

    fn is_valid(&self) -> bool {
        let operator_count = self.numbers.len() - 1;
        let it = (0..operator_count)
            .map(|_| [Op::Add, Op::Multiply, Op::Concatenate].into_iter())
            .multi_cartesian_product();
        for ops in it {
            let mut value = self.numbers[0];
            for (n, op) in self.numbers.iter().skip(1).zip(ops) {
                match op {
                    Op::Add => value += n,
                    Op::Multiply => value *= n,
                    Op::Concatenate => {
                        let digits = n.to_string().len() as u32;
                        value = value * 10_i32.pow(digits) as usize + n
                    }
                }
            }
            if value == self.test_value {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Multiply,
    Concatenate,
}

impl std::str::FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Failed to parse '{}'", s))?;
        let test_value = left.parse()?;
        let numbers: Vec<usize> = aoc::utils::parse_fields(right, ' ')?;
        if numbers.len() == 0 {
            bail!("No numbers found in '{}'", s);
        }
        Ok(Equation {
            test_value,
            numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        190: 10 19\n\
        3267: 81 40 27\n\
        83: 17 5\n\
        156: 15 6\n\
        7290: 6 8 6 15\n\
        161011: 16 10 13\n\
        192: 17 8 14\n\
        21037: 9 7 18 13\n\
        292: 11 6 16 20";

    #[test]
    fn wrong_is_valid() {
        let equations: Vec<Equation> = aoc::utils::parse_fields(TEST_DATA, '\n').unwrap();
        assert!(equations[0].wrong_is_valid());
        assert!(equations[1].wrong_is_valid());
        assert!(!equations[2].wrong_is_valid());
        assert!(!equations[3].wrong_is_valid());
        assert!(!equations[4].wrong_is_valid());
        assert!(!equations[5].wrong_is_valid());
        assert!(!equations[6].wrong_is_valid());
        assert!(!equations[7].wrong_is_valid());
        assert!(equations[8].wrong_is_valid());
        assert_eq!(part1(&equations), 3749);
    }

    #[test]
    fn is_valid() {
        let equations: Vec<Equation> = aoc::utils::parse_fields(TEST_DATA, '\n').unwrap();
        assert!(equations[0].is_valid());
        assert!(equations[1].is_valid());
        assert!(!equations[2].is_valid());
        assert!(equations[3].is_valid());
        assert!(equations[4].is_valid());
        assert!(!equations[5].is_valid());
        assert!(equations[6].is_valid());
        assert!(!equations[7].is_valid());
        assert!(equations[8].is_valid());
        assert_eq!(part2(&equations), 11387);
    }
}
