use anyhow::{bail, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let numbers: Vec<Snafu> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&numbers));

    Ok(())
}

fn part1(numbers: &[Snafu]) -> String {
    numbers.iter().fold(Snafu(0), |a, b| a + *b).to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Snafu(i64);

impl std::str::FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.bytes().try_fold(0, |total, b| {
            let digit = match b {
                b'=' => -2,
                b'-' => -1,
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                _ => bail!("Unrecognized character in {}", s),
            };
            Ok(total * 5 + digit)
        })?;

        Ok(Snafu(value))
    }
}

impl std::ops::Add for Snafu {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Snafu(self.0 + rhs.0)
    }
}

impl std::fmt::Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n = self.0;
        let mut digits = vec![];
        while n != 0 {
            let d = (n + 2) % 5 - 2;
            digits.push(d);
            n = (n - d) / 5;
        }

        for digit in digits.iter().rev() {
            let c = match digit {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => unreachable!(),
            };
            write!(f, "{}", c)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_PAIRS: [(&str, i64); 14] = [
        ("1=-0-2", 1747),
        ("12111", 906),
        ("2=0=", 198),
        ("21", 11),
        ("2=01", 201),
        ("111", 31),
        ("20012", 1257),
        ("112", 32),
        ("1=-1=", 353),
        ("1-12", 107),
        ("12", 7),
        ("1=", 3),
        ("122", 37),
        ("2=-1=0", 4890),
    ];

    #[test]
    fn parse() {
        for (s, n) in TEST_PAIRS {
            assert_eq!(s.parse::<Snafu>().unwrap(), Snafu(n));
        }
    }

    #[test]
    fn to_string() {
        for (s, n) in TEST_PAIRS {
            assert_eq!(Snafu(n).to_string(), s);
        }
    }
}
