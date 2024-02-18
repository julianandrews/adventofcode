use anyhow::{anyhow, Result};

use aoc::intcode::{RegisterValue, VM};

fn main() -> Result<()> {
    env_logger::init();

    let input = aoc::utils::get_input()?;
    let program = aoc::intcode::parse_program(&input)?;

    println!("Part 1: {}", part1(&program)?);
    println!("Part 2: {}", part2(&program)?);
    Ok(())
}

fn part1(program: &[RegisterValue]) -> Result<RegisterValue> {
    let mut vm = VM::from_iterator(program.to_vec(), std::iter::once(1));
    for output in vm.outputs() {
        if output != 0 {
            return Ok(output);
        }
    }
    Err(anyhow!("Non zero diagnostic code not found"))
}

fn part2(program: &[RegisterValue]) -> Result<RegisterValue> {
    let mut vm = VM::from_iterator(program.to_vec(), std::iter::once(5));
    for output in vm.outputs() {
        if output != 0 {
            return Ok(output);
        }
    }
    Err(anyhow!("Non zero diagnostic code not found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_first_output(program: &[RegisterValue], input: RegisterValue) -> Option<RegisterValue> {
        VM::from_iterator(program.to_vec(), std::iter::once(input))
            .outputs()
            .next()
    }

    #[test]
    fn test_simple_program() {
        let mut vm: VM = VM::from_iterator(vec![1002, 4, 3, 4, 33], std::iter::empty());
        assert!(vm.step().is_ok());
        assert_eq!(vm.memory(), vec![1002, 4, 3, 4, 99].as_slice());
    }

    #[test]
    fn test_equals_8() {
        let equals_8 = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(get_first_output(&equals_8, 8), Some(1));
        assert_eq!(get_first_output(&equals_8, 7), Some(0));
    }

    #[test]
    fn test_lt_8() {
        let lt_8 = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(get_first_output(&lt_8, 7), Some(1));
        assert_eq!(get_first_output(&lt_8, 8), Some(0));
    }

    #[test]
    fn test_equals_8_immediate() {
        let equals_8_immediate = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(get_first_output(&equals_8_immediate, 8), Some(1));
        assert_eq!(get_first_output(&equals_8_immediate, 7), Some(0));
    }

    #[test]
    fn test_lt_8_immediate() {
        let lt_8_immediate = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(get_first_output(&lt_8_immediate, 7), Some(1));
        assert_eq!(get_first_output(&lt_8_immediate, 8), Some(0));
    }

    #[test]
    fn test_longer_program() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(get_first_output(&program, 7), Some(999));
        assert_eq!(get_first_output(&program, 8), Some(1000));
        assert_eq!(get_first_output(&program, 10), Some(1001));
    }
}
