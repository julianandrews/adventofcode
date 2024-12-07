use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let equations: Vec<Equation> = aoc::utils::parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&equations));
    println!("Part 2: {}", part2(&equations));

    Ok(())
}

fn part1(equations: &[Equation]) -> usize {
    valid_total(equations, &[Op::Add, Op::Multiply])
}

fn part2(equations: &[Equation]) -> usize {
    valid_total(equations, &[Op::Add, Op::Multiply, Op::Concatenate])
}

fn valid_total(equations: &[Equation], ops: &[Op]) -> usize {
    equations
        .iter()
        .filter(|eq| eq.is_valid(ops))
        .map(|eq| eq.test_value)
        .sum()
}

#[derive(Debug, Clone)]
struct Equation {
    test_value: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn is_valid(&self, ops: &[Op]) -> bool {
        fn is_valid_recurse(value: usize, numbers: &[usize], ops: &[Op]) -> bool {
            if let Some((&n, rest)) = numbers.split_last() {
                for op in ops {
                    if let Some(new_value) = op.inverse(value, n) {
                        if is_valid_recurse(new_value, rest, ops) {
                            return true;
                        }
                    }
                }
            }
            value == 0
        }

        is_valid_recurse(self.test_value, &self.numbers, ops)
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Multiply,
    Concatenate,
}

impl Op {
    /// Return the inverse of Op(a, b) if it exists.
    fn inverse(&self, a: usize, b: usize) -> Option<usize> {
        match self {
            Op::Add => a.checked_sub(b),
            Op::Multiply => {
                if a % b == 0 {
                    Some(a / b)
                } else {
                    None
                }
            }
            Op::Concatenate => {
                let mut tens = 10;
                while tens < b {
                    tens = tens.saturating_mul(10);
                }
                if a % tens == b {
                    Some(a / tens)
                } else {
                    None
                }
            }
        }
    }
}

impl std::str::FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Failed to parse '{}'", s))?;
        let test_value = left.parse()?;
        let numbers: Vec<usize> = aoc::utils::parse_fields(right, ' ')?;
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
        let ops = [Op::Add, Op::Multiply];
        assert!(equations[0].is_valid(&ops));
        assert!(equations[1].is_valid(&ops));
        assert!(!equations[2].is_valid(&ops));
        assert!(!equations[3].is_valid(&ops));
        assert!(!equations[4].is_valid(&ops));
        assert!(!equations[5].is_valid(&ops));
        assert!(!equations[6].is_valid(&ops));
        assert!(!equations[7].is_valid(&ops));
        assert!(equations[8].is_valid(&ops));
        assert_eq!(valid_total(&equations, &ops), 3749);
    }

    #[test]
    fn is_valid() {
        let equations: Vec<Equation> = aoc::utils::parse_fields(TEST_DATA, '\n').unwrap();
        let ops = [Op::Add, Op::Multiply, Op::Concatenate];
        assert!(equations[0].is_valid(&ops));
        assert!(equations[1].is_valid(&ops));
        assert!(!equations[2].is_valid(&ops));
        assert!(equations[3].is_valid(&ops));
        assert!(equations[4].is_valid(&ops));
        assert!(!equations[5].is_valid(&ops));
        assert!(equations[6].is_valid(&ops));
        assert!(!equations[7].is_valid(&ops));
        assert!(equations[8].is_valid(&ops));
        assert_eq!(valid_total(&equations, &ops), 11387);
    }
}
