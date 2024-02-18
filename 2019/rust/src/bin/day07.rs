use std::iter;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use itertools::Itertools;

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
    let mut best = 0;
    for perm in (0..5).permutations(5) {
        let mut signal = 0;
        for phase in perm {
            let inputs = iter::once(phase).chain(iter::once(signal));
            let mut vm = VM::from_iterator(program.to_vec(), inputs);
            signal = vm.outputs().next().ok_or(anyhow!("Outputs exhausted"))?;
        }
        best = std::cmp::max(best, signal);
    }

    Ok(best)
}

fn part2(program: &[RegisterValue]) -> Result<RegisterValue> {
    let mut best = 0;
    for perm in (5..10).permutations(5) {
        let signal = Arc::new(Mutex::new(Some(0)));
        let mut vms: Vec<_> = perm
            .iter()
            .map(|&phase| {
                let inputs = iter::once(phase).chain(iter::from_fn(|| *signal.lock().unwrap()));
                VM::from_iterator(program.to_vec(), inputs)
            })
            .collect();

        while signal.lock().unwrap().is_some() {
            for vm in vms.iter_mut() {
                *signal.lock().unwrap() = vm.outputs().next();
                if signal.lock().unwrap().is_none() {
                    break;
                }
            }
        }
        best = std::cmp::max(
            best,
            vms[vms.len() - 1]
                .last_output()
                .ok_or(anyhow!("No output generated"))?,
        );
    }

    Ok(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part1(&vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 43210);

        let result = part1(&vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 54321);

        let result = part1(&vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 65210);
    }

    #[test]
    fn test_part_2() {
        let result = part2(&vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 139629729);

        let result = part2(&vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 18216);
    }
}
