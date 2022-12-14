#![feature(box_patterns)]

use anyhow::{anyhow, bail, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let numbers = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(numbers.clone()));
    println!("Part 2: {}", part2(&numbers));
    Ok(())
}

fn part1(numbers: Vec<SnailNumber>) -> u32 {
    numbers
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap_or(SnailNumber::Regular(0))
        .magnitude()
}

fn part2(numbers: &[SnailNumber]) -> u32 {
    let mut m = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                m = (numbers[i].clone() + numbers[j].clone()).magnitude().max(m);
            }
        }
    }
    m
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailNumber {
    Regular(u32),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

impl SnailNumber {
    fn magnitude(&self) -> u32 {
        match self {
            SnailNumber::Regular(n) => *n,
            SnailNumber::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        self.explode_helper(0).0
    }

    fn explode_helper(&mut self, d: usize) -> (bool, u32, u32) {
        match self {
            SnailNumber::Regular(_) => (false, 0, 0),
            SnailNumber::Pair(left, right) => match (left, right) {
                (box SnailNumber::Regular(a), box SnailNumber::Regular(b)) if d >= 4 => {
                    let (a, b) = (*a, *b);
                    *self = SnailNumber::Regular(0);
                    (true, a, b)
                }
                (left, right) => {
                    let (changed, a, b) = left.explode_helper(d + 1);
                    if changed {
                        right.add_left(b);
                        (true, a, 0)
                    } else {
                        let (changed, a, b) = right.explode_helper(d + 1);
                        if changed {
                            left.add_right(a);
                            (true, 0, b)
                        } else {
                            (false, 0, 0)
                        }
                    }
                }
            },
        }
    }

    fn add_left(&mut self, n: u32) {
        match self {
            SnailNumber::Regular(m) => *m += n,
            SnailNumber::Pair(left, _) => left.add_left(n),
        }
    }

    fn add_right(&mut self, n: u32) {
        match self {
            SnailNumber::Regular(m) => *m += n,
            SnailNumber::Pair(_, right) => right.add_right(n),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailNumber::Regular(n) if *n < 10 => false,
            SnailNumber::Regular(n) => {
                *self = SnailNumber::Pair(
                    Box::new(SnailNumber::Regular(*n / 2)),
                    Box::new(SnailNumber::Regular((*n + 1) / 2)),
                );
                true
            }
            SnailNumber::Pair(left, right) => left.split() || right.split(),
        }
    }
}

impl std::ops::Add<SnailNumber> for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: SnailNumber) -> Self::Output {
        let mut n = SnailNumber::Pair(Box::new(self), Box::new(rhs));
        n.reduce();
        n
    }
}

impl std::str::FromStr for SnailNumber {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n, rem) = parse_input(s)?;
        if !rem.is_empty() {
            bail!("Failed to parse whole input");
        }
        Ok(n)
    }
}

fn parse_input(s: &str) -> Result<(SnailNumber, &str)> {
    if let Some(rem) = s.strip_prefix('[') {
        let (left, rem) = parse_input(rem)?;
        let rem = rem
            .strip_prefix(',')
            .ok_or_else(|| anyhow!("Expected ','"))?;
        let (right, rem) = parse_input(rem)?;
        let rem = rem
            .strip_prefix(']')
            .ok_or_else(|| anyhow!("Expected ']'"))?;
        Ok((SnailNumber::Pair(Box::new(left), Box::new(right)), rem))
    } else {
        let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
        Ok((SnailNumber::Regular(digits.parse()?), &s[digits.len()..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
            [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n\
            [[[5,[2,8]],4],[5,[[9,9],0]]]\n\
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n\
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n\
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n\
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n\
            [[[[5,4],[7,7]],8],[[8,3],8]]\n\
            [[9,3],[[9,9],[6,[4,9]]]]\n\
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n\
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn add_1() {
        let result = sum_list(&["[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"]);
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn add_2() {
        let result = sum_list(&["[1,1]", "[2,2]", "[3,3]", "[4,4]"]);
        let expected = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn add_3() {
        let result = sum_list(&["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"]);
        let expected = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn add_4() {
        let result = sum_list(&["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"]);
        let expected = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn add_5() {
        let result = sum_list(&[
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ]);
        let expected = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn magnitude_1() {
        let n: SnailNumber = "[[1,2],[[3,4],5]]".parse().unwrap();
        assert_eq!(n.magnitude(), 143);
    }

    #[test]
    fn magnitude_2() {
        let n: SnailNumber = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".parse().unwrap();
        assert_eq!(n.magnitude(), 1384);
    }

    #[test]
    fn magnitude_3() {
        let n: SnailNumber = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap();
        assert_eq!(n.magnitude(), 445);
    }

    #[test]
    fn magnitude_4() {
        let n: SnailNumber = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap();
        assert_eq!(n.magnitude(), 791);
    }

    #[test]
    fn magnitude_5() {
        let n: SnailNumber = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap();
        assert_eq!(n.magnitude(), 1137);
    }

    #[test]
    fn magnitude_6() {
        let n: SnailNumber = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap();
        assert_eq!(n.magnitude(), 3488);
    }

    #[test]
    fn full_part_1() {
        let numbers: Vec<SnailNumber> = parse_fields(TEST_DATA, '\n').unwrap();

        assert_eq!(part1(numbers), 4140);
    }

    #[test]
    fn full_part_2() {
        let numbers: Vec<SnailNumber> = parse_fields(TEST_DATA, '\n').unwrap();

        assert_eq!(part2(&numbers), 3993);
    }

    fn sum_list(strings: &[&str]) -> SnailNumber {
        let parsed: Vec<SnailNumber> = strings.into_iter().map(|s| s.parse().unwrap()).collect();
        parsed.into_iter().reduce(|a, b| a + b).unwrap()
    }
}
