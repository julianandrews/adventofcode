extern crate aoc;
extern crate log;

use aoc::aoc_error::AOCError;
use aoc::intcode::{OpType, RegisterValue, VM};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn run(
    program: Vec<RegisterValue>,
    noun: Option<RegisterValue>,
    verb: Option<RegisterValue>,
) -> Result<VM> {
    let mut vm = VM::new(program, None);
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

fn part1(program: &Vec<RegisterValue>) -> Result<RegisterValue> {
    Ok(run(program.clone(), Some(12), Some(2))?.diagnostic_code())
}

fn part2(program: &Vec<RegisterValue>) -> Result<RegisterValue> {
    for a in 0..99 {
        for b in 0..99 {
            if run(program.clone(), Some(a), Some(b))?.diagnostic_code() == 19690720 {
                return Ok(100 * a + b);
            }
        }
    }
    Err(AOCError::new("Correct inputs not found"))?
}

fn main() -> Result<()> {
    env_logger::init();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program = aoc::intcode::parse_program(&input)?;

    println!("Part 1: {}", part1(&program)?);
    println!("Part 2: {}", part2(&program)?);
    Ok(())
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
