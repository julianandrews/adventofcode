use anyhow::{bail, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let instructions: Vec<Instruction> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2:\n{}", part2(&instructions));

    Ok(())
}

fn part1(instructions: &[Instruction]) -> i64 {
    signal_strengths(instructions).sum()
}

fn part2(instructions: &[Instruction]) -> String {
    static WIDTH: usize = 40;
    static HEIGHT: usize = 6;
    let mut output: Vec<char> = vec![];
    for (i, x) in register_values(instructions).enumerate() {
        let pos = (i % WIDTH) as i64;
        let pixel = if (x - pos).abs() < 2 { '█' } else { '·' };
        output.push(pixel);
        if pos == (WIDTH - 1) as i64 {
            output.push('\n');
        }
    }
    output.into_iter().take((WIDTH + 1) * HEIGHT - 1).collect()
}

fn signal_strengths(instructions: &[Instruction]) -> impl Iterator<Item = i64> {
    register_values(instructions)
        .enumerate()
        .map(|(i, x)| x * (i as i64 + 1))
        .skip(19)
        .step_by(40)
}

fn register_values(instructions: &[Instruction]) -> impl Iterator<Item = i64> {
    let mut register_values = vec![1];

    for instruction in instructions {
        let last_x = *register_values.last().unwrap();
        register_values.push(last_x);
        match instruction {
            Instruction::AddX(x) => register_values.push(last_x + x),
            Instruction::Noop => {}
        }
    }

    register_values.into_iter()
}

#[derive(Debug, Clone)]
enum Instruction {
    AddX(i64),
    Noop,
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else if let Some(value) = s.strip_prefix("addx ") {
            Ok(Self::AddX(value.parse()?))
        } else {
            bail!("Invalid instruction: {}", s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SHORT_PROGRAM: &[&str] = &["noop", "addx 3", "addx -5"];

    static LONG_PROGRAM: &[&str] = &[
        "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
        "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1",
        "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1", "addx 16",
        "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop", "addx -3", "addx 9",
        "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop", "noop", "noop", "noop",
        "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop", "addx 2", "addx 6", "noop",
        "noop", "noop", "noop", "noop", "addx 1", "noop", "noop", "addx 7", "addx 1", "noop",
        "addx -13", "addx 13", "addx 7", "noop", "addx 1", "addx -33", "noop", "noop", "noop",
        "addx 2", "noop", "noop", "noop", "addx 8", "noop", "addx -1", "addx 2", "addx 1", "noop",
        "addx 17", "addx -9", "addx 1", "addx 1", "addx -3", "addx 11", "noop", "noop", "addx 1",
        "noop", "addx 1", "noop", "noop", "addx -13", "addx -19", "addx 1", "addx 3", "addx 26",
        "addx -30", "addx 12", "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9",
        "addx 18", "addx 1", "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1",
        "addx 2", "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22",
        "addx -6", "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop",
        "addx 20", "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
    ];

    #[test]
    fn register_values_short() {
        let instructions: Vec<Instruction> =
            SHORT_PROGRAM.iter().map(|s| s.parse().unwrap()).collect();
        let result: Vec<_> = register_values(&instructions).collect();
        let expected = vec![1, 1, 1, 4, 4, -1];
        assert_eq!(result, expected);
    }

    #[test]
    fn signal_strengths_long() {
        let instructions: Vec<Instruction> =
            LONG_PROGRAM.iter().map(|s| s.parse().unwrap()).collect();
        let result: Vec<_> = signal_strengths(&instructions).collect();
        let expected = vec![420, 1140, 1800, 2940, 2880, 3960];
        assert_eq!(result, expected);
    }

    #[test]
    fn draw() {
        let instructions: Vec<Instruction> =
            LONG_PROGRAM.iter().map(|s| s.parse().unwrap()).collect();
        let expected = "\
            ██··██··██··██··██··██··██··██··██··██··\n\
            ███···███···███···███···███···███···███·\n\
            ████····████····████····████····████····\n\
            █████·····█████·····█████·····█████·····\n\
            ██████······██████······██████······████\n\
            ███████·······███████·······███████·····";
        let result = part2(&instructions);

        assert_eq!(result, expected);
    }
}
