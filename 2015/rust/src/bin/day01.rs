use anyhow::{anyhow, Result};

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let instructions: Vec<Instruction> = parsing::parse_instructions(input.trim())?;

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions)?);

    Ok(())
}

fn part1(instructions: &[Instruction]) -> i64 {
    end_floor(instructions)
}

fn part2(instructions: &[Instruction]) -> Result<usize> {
    basement_time(instructions).ok_or(anyhow!("Never entered basement."))
}

fn end_floor(instructions: &[Instruction]) -> i64 {
    instructions.iter().map(Instruction::offset).sum()
}

fn basement_time(instructions: &[Instruction]) -> Option<usize> {
    let mut floor: i64 = 0;

    for (i, instruction) in instructions.iter().enumerate() {
        floor += instruction.offset();
        if floor < 0 {
            return Some(i + 1);
        }
    }

    None
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Up,
    Down,
}

impl Instruction {
    fn offset(&self) -> i64 {
        match self {
            Instruction::Up => 1,
            Instruction::Down => -1,
        }
    }
}

mod parsing {
    use super::Instruction;

    use anyhow::{anyhow, Result};

    pub fn parse_instructions(s: &str) -> Result<Vec<Instruction>> {
        s.chars()
            .map(|c| match c {
                '(' => Ok(Instruction::Up),
                ')' => Ok(Instruction::Down),
                c => Err(anyhow!("Unrecognized instruction {}", c)),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{basement_time, end_floor, parsing};

    #[test]
    fn end_floor_1() {
        let instructions = parsing::parse_instructions("(())").unwrap();

        assert_eq!(end_floor(&instructions), 0);
    }

    #[test]
    fn end_floor_2() {
        let instructions = parsing::parse_instructions("()()").unwrap();

        assert_eq!(end_floor(&instructions), 0);
    }

    #[test]
    fn end_floor_3() {
        let instructions = parsing::parse_instructions("(((").unwrap();

        assert_eq!(end_floor(&instructions), 3);
    }

    #[test]
    fn end_floor_4() {
        let instructions = parsing::parse_instructions("(()(()(").unwrap();

        assert_eq!(end_floor(&instructions), 3);
    }

    #[test]
    fn end_floor_5() {
        let instructions = parsing::parse_instructions("))(((((").unwrap();

        assert_eq!(end_floor(&instructions), 3);
    }

    #[test]
    fn end_floor_6() {
        let instructions = parsing::parse_instructions("())").unwrap();

        assert_eq!(end_floor(&instructions), -1);
    }

    #[test]
    fn end_floor_7() {
        let instructions = parsing::parse_instructions("))(").unwrap();

        assert_eq!(end_floor(&instructions), -1);
    }

    #[test]
    fn end_floor_8() {
        let instructions = parsing::parse_instructions(")))").unwrap();

        assert_eq!(end_floor(&instructions), -3);
    }

    #[test]
    fn end_floor_9() {
        let instructions = parsing::parse_instructions(")())())").unwrap();

        assert_eq!(end_floor(&instructions), -3);
    }

    #[test]
    fn basement_time_1() {
        let instructions = parsing::parse_instructions(")").unwrap();

        assert_eq!(basement_time(&instructions), Some(1));
    }

    #[test]
    fn basement_time_2() {
        let instructions = parsing::parse_instructions("()())").unwrap();

        assert_eq!(basement_time(&instructions), Some(5));
    }
}
