#![feature(str_split_once)]

use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;

    println!("Part 1: {}", part1(input.parse()?));
    println!("Part 2: {}", part2(input.parse()?)?);
    Ok(())
}

fn part1(mut vm: VirtualMachine) -> i64 {
    vm.run_until_repeat_or_done();
    vm.accumulator
}

fn part2(mut vm: VirtualMachine) -> Result<i64> {
    vm.find_swap()
        .ok_or(AOCError::new("Terminating program not found").into())
}

#[derive(Debug)]
struct VirtualMachine {
    instructions: Vec<Instruction>,
    accumulator: i64,
    ip: usize,
}

impl VirtualMachine {
    fn step(&mut self) -> ProgramState {
        let op = match self.instructions.get(self.ip) {
            Some(op) => op,
            None => return ProgramState::Complete,
        };
        match op {
            Instruction::Nop(_) => self.ip += 1,
            Instruction::Acc(value) => {
                self.accumulator += value;
                self.ip += 1;
            }
            Instruction::Jmp(value) => self.ip = usize::try_from(self.ip as i64 + value).unwrap(),
        }
        ProgramState::Running
    }

    fn run_until_repeat_or_done(&mut self) -> ProgramState {
        let mut seen = HashSet::new();
        let mut state = ProgramState::Running;
        while !seen.contains(&self.ip) {
            seen.insert(self.ip);
            state = self.step();
        }
        state
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.accumulator = 0;
    }

    fn swap(&mut self, index: usize) -> bool {
        match self.instructions.get(index) {
            Some(Instruction::Nop(value)) => self.instructions[index] = Instruction::Jmp(*value),
            Some(Instruction::Jmp(value)) => self.instructions[index] = Instruction::Nop(*value),
            _ => return false,
        }
        true
    }

    fn find_swap(&mut self) -> Option<i64> {
        for index in 0..self.instructions.len() {
            self.reset();
            if self.swap(index) {
                match self.run_until_repeat_or_done() {
                    ProgramState::Complete => return Some(self.accumulator),
                    _ => {}
                }
            }
            self.swap(index);
        }
        None
    }
}

impl FromStr for VirtualMachine {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let instructions: Vec<Instruction> = s.lines().map(&str::parse).collect::<Result<_>>()?;

        Ok(VirtualMachine {
            instructions,
            accumulator: 0,
            ip: 0,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ProgramState {
    Running,
    Complete,
}

#[derive(Debug)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (opp_part, value_part) = s
            .split_once(' ')
            .ok_or(AOCError::new("Invalid instruction"))?;
        let value = value_part.parse()?;
        match opp_part {
            "nop" => Ok(Instruction::Nop(value)),
            "acc" => Ok(Instruction::Acc(value)),
            "jmp" => Ok(Instruction::Jmp(value)),
            _ => Err(AOCError::new("Unrecognized instruction").into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "nop +0\
                                     \nacc +1\
                                     \njmp +4\
                                     \nacc +3\
                                     \njmp -3\
                                     \nacc -99\
                                     \nacc +1\
                                     \njmp -4\
                                     \nacc +6";

    #[test]
    fn step() {
        let mut vm: VirtualMachine = TEST_INPUT.parse().unwrap();
        vm.step();
        assert_eq!((vm.ip, vm.accumulator), (1, 0));
        vm.step();
        assert_eq!((vm.ip, vm.accumulator), (2, 1));
        vm.step();
        assert_eq!((vm.ip, vm.accumulator), (6, 1));
        vm.step();
        assert_eq!((vm.ip, vm.accumulator), (7, 2));
        vm.step();
        assert_eq!((vm.ip, vm.accumulator), (3, 2));
        vm.step();
        assert_eq!((vm.ip, vm.accumulator), (4, 5));
        vm.step();
        assert_eq!((vm.ip, vm.accumulator), (1, 5));
    }

    #[test]
    fn run_until_repeat() {
        let mut vm: VirtualMachine = TEST_INPUT.parse().unwrap();
        let result = vm.run_until_repeat_or_done();
        assert_eq!(result, ProgramState::Running);
        assert_eq!((vm.ip, vm.accumulator), (1, 5));
    }

    #[test]
    fn find_swap() {
        let mut vm: VirtualMachine = TEST_INPUT.parse().unwrap();
        assert_eq!(vm.find_swap(), Some(8));
    }
}
