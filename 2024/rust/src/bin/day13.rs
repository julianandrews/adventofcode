#![cfg_attr(test, feature(assert_matches))]

use anyhow::{anyhow, Result};

use aoc::planar::Point;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let machines: Vec<ClawMachine> = aoc::utils::parse_fields(input.trim(), "\n\n")?;

    println!("Part 1: {}", part1(&machines));
    println!("Part 2: {}", part2(&machines));

    Ok(())
}

fn part1(machines: &[ClawMachine]) -> i64 {
    machines.iter().filter_map(ClawMachine::min_tokens).sum()
}

fn part2(machines: &[ClawMachine]) -> i64 {
    machines
        .iter()
        .filter_map(|machine| machine.corrected().min_tokens())
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    a: Point,
    b: Point,
    prize: Point,
}

impl ClawMachine {
    fn min_tokens(&self) -> Option<i64> {
        // Solve a couple linear equations, keep only integral solutions.
        let numerator_a = self.prize.x * self.b.y - self.prize.y * self.b.x;
        let numerator_b = self.prize.y * self.a.x - self.prize.x * self.a.y;
        let denominator = self.a.x * self.b.y - self.b.x * self.a.y;
        if numerator_a % denominator == 0 && numerator_b % denominator == 0 {
            Some((3 * numerator_a + numerator_b) / denominator)
        } else {
            None
        }
    }

    fn corrected(&self) -> Self {
        let mut corrected = *self;
        corrected.prize.x += 10000000000000;
        corrected.prize.y += 10000000000000;
        corrected
    }
}

impl std::str::FromStr for ClawMachine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        type Pair<'a> = (&'a str, &'a str);

        fn split_parts(s: &str) -> Option<(Pair, Pair, Pair)> {
            let (a_part, rest) = s.split_once('\n')?;
            let (b_part, prize_part) = rest.split_once('\n')?;
            let a = a_part.strip_prefix("Button A: X+")?.split_once(", Y+")?;
            let b = b_part.strip_prefix("Button B: X+")?.split_once(", Y+")?;
            let p = prize_part.strip_prefix("Prize: X=")?.split_once(", Y=")?;
            Some((a, b, p))
        }

        fn parse_point(pair: Pair) -> Result<Point> {
            Ok(Point {
                x: pair.0.parse()?,
                y: pair.1.parse()?,
            })
        }

        let (a, b, p) = split_parts(s).ok_or_else(|| anyhow!("Failed to split {:?}", s))?;
        Ok(ClawMachine {
            a: parse_point(a)?,
            b: parse_point(b)?,
            prize: parse_point(p)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    static TEST_DATA: &str = "\
        Button A: X+94, Y+34\n\
        Button B: X+22, Y+67\n\
        Prize: X=8400, Y=5400\n\
        \n\
        Button A: X+26, Y+66\n\
        Button B: X+67, Y+21\n\
        Prize: X=12748, Y=12176\n\
        \n\
        Button A: X+17, Y+86\n\
        Button B: X+84, Y+37\n\
        Prize: X=7870, Y=6450\n\
        \n\
        Button A: X+69, Y+23\n\
        Button B: X+27, Y+71\n\
        Prize: X=18641, Y=10279";

    #[test]
    fn limited_min_tokens() {
        let machines: Vec<ClawMachine> = aoc::utils::parse_fields(TEST_DATA, "\n\n").unwrap();

        assert_eq!(machines[0].min_tokens(), Some(280));
        assert_eq!(machines[1].min_tokens(), None);
        assert_eq!(machines[2].min_tokens(), Some(200));
        assert_eq!(machines[3].min_tokens(), None);
    }

    #[test]
    fn min_tokens() {
        let machines: Vec<ClawMachine> = aoc::utils::parse_fields(TEST_DATA, "\n\n").unwrap();
        let machines: Vec<_> = machines.iter().map(ClawMachine::corrected).collect();

        assert_matches!(machines[0].min_tokens(), None);
        assert_matches!(machines[1].min_tokens(), Some(459236326669));
        assert_matches!(machines[2].min_tokens(), None);
        assert_matches!(machines[3].min_tokens(), Some(416082282239));
    }
}
