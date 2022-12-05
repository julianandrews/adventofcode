#![feature(try_blocks)]

use anyhow::{anyhow, Context, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let (stacks, moves) = match input.split_once("\n\n") {
        Some(value) => value,
        None => anyhow::bail!("Invalid input"),
    };
    let stacks: Stacks = stacks.parse()?;
    let moves: Vec<Move> = parse_fields(moves.trim(), '\n')?;

    println!("Part 1: {}", part1(stacks.clone(), &moves)?);
    println!("Part 2: {}", part2(stacks, &moves)?);

    Ok(())
}

fn part1(mut stacks: Stacks, moves: &[Move]) -> Result<String> {
    for mv in moves {
        stacks.do_wrong_move(mv)?;
    }
    stacks.read_top().ok_or_else(|| anyhow!("Empty stack"))
}

fn part2(mut stacks: Stacks, moves: &[Move]) -> Result<String> {
    for mv in moves {
        stacks.do_real_move(mv)?;
    }
    stacks.read_top().ok_or_else(|| anyhow!("Empty stack"))
}

#[derive(Debug, Clone, PartialEq)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn do_wrong_move(&mut self, mv: &Move) -> Result<()> {
        let option: Option<()> = try {
            for _ in 0..mv.number {
                let value: char = self.0.get_mut(mv.from)?.pop()?;
                self.0.get_mut(mv.to)?.push(value);
            }
        };
        option.ok_or_else(|| anyhow!("Failed to do move"))
    }

    fn do_real_move(&mut self, mv: &Move) -> Result<()> {
        let option: Option<()> = try {
            let from_stack = &mut self.0.get_mut(mv.from)?;
            let mut values = from_stack.split_off(from_stack.len() - mv.number);
            self.0.get_mut(mv.to)?.append(&mut values);
        };
        option.ok_or_else(|| anyhow!("Failed to do move"))
    }

    fn read_top(&self) -> Option<String> {
        self.0
            .iter()
            .map(|v| v.iter().last())
            .collect::<Option<String>>()
    }
}

impl std::str::FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&[u8]> = s.split('\n').map(str::as_bytes).collect();
        let len = lines.iter().map(|s| s.len()).max().unwrap_or(0);
        let mut stacks = vec![vec![]; len / 4 + 1];
        for line in lines.iter().rev().skip(1) {
            if line.len() != len {
                anyhow::bail!("Unequal line lengths in stack");
            }
            for (index, stack) in stacks.iter_mut().enumerate() {
                let b = line[index * 4 + 1];
                match b {
                    b'A'..=b'Z' => stack.push(b as char),
                    b' ' => {}
                    _ => anyhow::bail!("Unexpected character in {}", String::from_utf8_lossy(line)),
                }
            }
        }
        Ok(Self(stacks))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Move {
    number: usize,
    from: usize,
    to: usize,
}

impl std::str::FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();
        if words.len() != 6 || words[0] != "move" || words[2] != "from" || words[4] != "to" {
            anyhow::bail!("Invalid move: {}", s);
        }
        let result: Result<_> = {
            let number = words[1].parse::<usize>()?;
            let from = words[3]
                .parse::<usize>()?
                .checked_sub(1)
                .ok_or_else(|| anyhow!("Invalid from"))?;
            let to = words[5]
                .parse::<usize>()?
                .checked_sub(1)
                .ok_or_else(|| anyhow!("Invalid to"))?;
            Ok(Move { number, from, to })
        };
        result.with_context(|| format!("Failed to parse Move from '{}'", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_STACKS: &str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 "
    );
    static TEST_MOVES: &[&str] = &[
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ];

    #[test]
    fn parse_stacks() {
        let stacks: Stacks = TEST_STACKS.parse().unwrap();
        let expected = Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        assert_eq!(stacks, expected)
    }

    #[test]
    fn parse_moves() {
        let moves: Vec<Move> = TEST_MOVES.iter().map(|s| s.parse().unwrap()).collect();
        let expected = vec![
            Move {
                number: 1,
                from: 1,
                to: 0,
            },
            Move {
                number: 3,
                from: 0,
                to: 2,
            },
            Move {
                number: 2,
                from: 1,
                to: 0,
            },
            Move {
                number: 1,
                from: 0,
                to: 1,
            },
        ];
        assert_eq!(moves, expected)
    }

    #[test]
    fn do_wrong_moves() {
        let mut stacks: Stacks = TEST_STACKS.parse().unwrap();
        let moves: Vec<Move> = TEST_MOVES.iter().map(|s| s.parse().unwrap()).collect();
        for mv in moves {
            stacks.do_wrong_move(&mv).unwrap();
        }
        let expected = Stacks(vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']]);
        assert_eq!(stacks, expected);
    }

    #[test]
    fn read_top_1() {
        let stacks = Stacks(vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']]);
        assert_eq!(stacks.read_top().unwrap(), "CMZ")
    }

    #[test]
    fn do_real_moves() {
        let mut stacks: Stacks = TEST_STACKS.parse().unwrap();
        let moves: Vec<Move> = TEST_MOVES.iter().map(|s| s.parse().unwrap()).collect();
        for mv in moves {
            stacks.do_real_move(&mv).unwrap();
        }
        let expected = Stacks(vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']]);
        assert_eq!(stacks, expected);
        assert_eq!(stacks.read_top().unwrap(), "MCD")
    }

    #[test]
    fn read_top_2() {
        let stacks = Stacks(vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']]);
        assert_eq!(stacks.read_top().unwrap(), "MCD")
    }
}
