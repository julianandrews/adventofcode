extern crate log;

use aoc::aoc_error::AOCError;
use aoc::intcode::{RegisterValue, VM};
use std::iter;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn run_with_single_input(program: &[RegisterValue], input: RegisterValue) -> Result<RegisterValue> {
    let mut vm = VM::new(program.to_vec(), Some(Box::new(iter::once(input))));
    let value = vm.outputs().next().ok_or("No output generated")?;
    if vm.outputs().next().is_some() {
        Err(AOCError::new("Unexpected output"))?
    }

    Ok(value)
}

fn part1(program: &[RegisterValue]) -> Result<RegisterValue> {
    run_with_single_input(program, 1)
}

fn part2(program: &[RegisterValue]) -> Result<RegisterValue> {
    run_with_single_input(program, 2)
}

fn main() -> Result<()> {
    env_logger::init();

    let input = aoc::utils::get_input()?;
    let program = aoc::intcode::parse_program(&input)?;

    println!("Part 1: {}", part1(&program)?);
    println!("Part 2: {}", part2(&program)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_program_1() {
        init();

        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let outputs: Vec<RegisterValue> = VM::new(program.clone(), None).outputs().collect();
        assert_eq!(outputs, program);
    }

    #[test]
    fn test_program_2() {
        init();

        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let outputs: Vec<RegisterValue> = VM::new(program, None).outputs().collect();
        assert_eq!(outputs.len(), 1);
        assert!(outputs[0] >= 1_000_000_000_000_000 && outputs[0] <= 9_999_999_999_999_999);
    }

    #[test]
    fn test_program_3() {
        init();

        let program = vec![104, 112589906842624, 99];
        let outputs: Vec<RegisterValue> = VM::new(program.clone(), None).outputs().collect();
        assert_eq!(outputs[..], program[1..2]);
    }
}
