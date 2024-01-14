use itertools::Itertools;

use aoc::utils::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let value: LookAndSay = input.trim().parse()?;

    println!("Part 1: {}", part1(&value));
    println!("Part 2: {}", part2(&value));

    Ok(())
}

fn part1(value: &LookAndSay) -> usize {
    value.nth(40).len()
}

fn part2(value: &LookAndSay) -> usize {
    value.nth(50).len()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LookAndSay(Vec<u8>);

impl LookAndSay {
    fn nth(&self, n: usize) -> LookAndSay {
        let mut value = self.clone();
        for _ in 0..n {
            value = value.next();
        }
        value
    }

    fn next(&self) -> LookAndSay {
        let mut values = vec![];
        let groups = self.0.iter().group_by(|&x| x);
        for (&value, group) in groups.into_iter() {
            values.push(group.count() as u8);
            values.push(value);
        }
        LookAndSay(values)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

mod parsing {
    use super::LookAndSay;

    use anyhow::{bail, Result};

    impl std::str::FromStr for LookAndSay {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let bytes = s
                .bytes()
                .map(|b| match b {
                    b'1'..=b'9' => Ok(b - b'0'),
                    _ => bail!("Invalid character in input"),
                })
                .collect::<Result<Vec<u8>>>()?;
            Ok(LookAndSay(bytes))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LookAndSay;

    #[test]
    fn nth() {
        let value: LookAndSay = "1".parse().unwrap();
        assert_eq!(value.nth(1), LookAndSay(vec![1, 1]));
        assert_eq!(value.nth(2), LookAndSay(vec![2, 1]));
        assert_eq!(value.nth(3), LookAndSay(vec![1, 2, 1, 1]));
        assert_eq!(value.nth(4), LookAndSay(vec![1, 1, 1, 2, 2, 1]));
        assert_eq!(value.nth(5), LookAndSay(vec![3, 1, 2, 2, 1, 1]));
    }
}
