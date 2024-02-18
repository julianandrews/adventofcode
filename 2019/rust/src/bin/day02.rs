use anyhow::{anyhow, Result};

use aoc::intcode::{OpType, RegisterValue, VM};

fn main() -> Result<()> {
    env_logger::init();

    let input = aoc::utils::get_input()?;
    let program = aoc::intcode::parse_program(&input)?;

    println!("Part 1: {}", part1(&program)?);
    println!("Part 2: {}", part2(&program)?);
    Ok(())
}

fn part1(program: &[RegisterValue]) -> Result<RegisterValue> {
    Ok(run(program.to_vec(), Some(12), Some(2))?.diagnostic_code())
}

fn part2(program: &[RegisterValue]) -> Result<RegisterValue> {
    for a in 0..99 {
        for b in 0..99 {
            if run(program.to_vec(), Some(a), Some(b))?.diagnostic_code() == 19690720 {
                return Ok(100 * a + b);
            }
        }
    }
    Err(anyhow!("Correct inputs not found"))
}

fn run<'a>(
    program: Vec<RegisterValue>,
    noun: Option<RegisterValue>,
    verb: Option<RegisterValue>,
) -> Result<VM<'a>> {
    let mut vm = VM::from_iterator(program, std::iter::empty());
    if let Some(noun) = noun {
        vm.set_memory(1, noun)
    };
    if let Some(verb) = verb {
        vm.set_memory(2, verb)
    };
    loop {
        let (op_type, _) = vm.step()?;
        if op_type == OpType::Halt {
            break;
        }
    }

    Ok(vm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let result = run(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            Some(9),
            Some(10),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().diagnostic_code(), 3500);

        let result = run(vec![1, 0, 0, 0, 99], None, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().memory(), vec![2, 0, 0, 0, 99].as_slice());

        let result = run(vec![2, 3, 0, 3, 99], None, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().memory(), vec![2, 3, 0, 6, 99].as_slice());

        let result = run(vec![2, 4, 4, 5, 99, 0], None, None);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().memory(),
            vec![2, 4, 4, 5, 99, 9801].as_slice()
        );

        let result = run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], None, None);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().memory(),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99].as_slice()
        );
    }
}
