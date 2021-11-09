use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::str::FromStr;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let program: Program = input.parse()?;

    println!("Part 1: {}", part1(program.clone()));
    println!("Part 2: {}", part2(program)?);
    Ok(())
}

fn part1(program: Program) -> i64 {
    let mut vm = VirtualMachine::new(program);
    vm.run_until_repeat_or_done();
    vm.accumulator
}

fn part2(mut program: Program) -> Result<i64> {
    program.fix()?;
    let mut vm = VirtualMachine::new(program);
    match vm.run_until_repeat_or_done() {
        ProgramState::Running => Err(AOCError::new("Fixed program did not terminate"))?,
        ProgramState::Complete => Ok(vm.accumulator),
    }
}

#[derive(Debug, Clone)]
struct Program(Vec<Instruction>);

impl Program {
    /// Get the Instruction at index `i` in the program.
    pub fn op(&self, i: usize) -> Option<&Instruction> {
        self.0.get(i)
    }

    /// Fix the program by doing a single jmp/nop swap.
    pub fn fix(&mut self) -> Result<()> {
        let i = self.fix_index()?;
        let op = self.0.get_mut(i).ok_or(AOCError::new("Fix out of range"))?;
        *op = match op {
            Instruction::Jmp(j) => Instruction::Nop(*j),
            Instruction::Nop(j) => Instruction::Jmp(*j),
            _ => return Err(Box::new(AOCError::new("Invalid operation for fix"))),
        };
        Ok(())
    }

    /// Returns the first index that can be fixed to allow the program to terminate.
    fn fix_index(&self) -> Result<usize> {
        // Find all the instructions that eventually terminate in the unfixed program.
        let terminating_instructions = self.terminating_instructions();

        // Scan from the start of the program and check each instruction to see if fixing it will lead
        // to a terminating instruction.
        let mut i = 0;
        let mut seen = BTreeSet::new();
        while !seen.contains(&i) {
            seen.insert(i);
            // Get the index of the next operation for both the fixed and unfixed programs.
            let (next_index, fixed_next_index) = match self.op(i) {
                Some(Instruction::Jmp(j)) => (self.next_index(i, *j), self.next_index(i, 1)),
                Some(Instruction::Nop(j)) => (self.next_index(i, 1), self.next_index(i, *j)),
                _ => (self.next_index(i, 1), None),
            };
            if let Some(j) = fixed_next_index {
                // If the fixed index terminates, we've found a fix!
                if terminating_instructions.contains(&j) {
                    return Ok(i);
                }
            }
            i = next_index.ok_or(AOCError::new("Unfixed program terminated"))?;
        }
        Err(Box::new(AOCError::new("No fix found")))
    }

    /// Returns the set of indices of all instructions which eventually terminate.
    fn terminating_instructions(&self) -> BTreeSet<usize> {
        let mut terminating = BTreeSet::new();
        let mut unchecked: BTreeSet<usize> = (0..self.0.len()).collect();
        while let Some(mut i) = unchecked.iter().next().copied() {
            let mut current = BTreeSet::new();
            while unchecked.contains(&i) {
                unchecked.remove(&i);
                current.insert(i);
                let next = self.op(i).and_then(|op| self.next_index(i, op.offset()));
                if next.is_none() || terminating.contains(&next.unwrap()) {
                    terminating.append(&mut current);
                    break;
                }
                i = next.unwrap();
            }
        }

        terminating
    }

    /// Returns the instruction index `offset` from `i`, or `None` if the program would terminate.
    fn next_index(&self, i: usize, offset: i64) -> Option<usize> {
        usize::try_from(i64::try_from(i).ok()? + offset)
            .ok()
            .filter(|&j| j < self.0.len())
    }
}

impl FromStr for Program {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Program(s.lines().map(&str::parse).collect::<Result<_>>()?))
    }
}

#[derive(Debug)]
struct VirtualMachine {
    program: Program,
    accumulator: i64,
    ip: usize,
}

impl VirtualMachine {
    pub fn new(program: Program) -> Self {
        VirtualMachine {
            program,
            accumulator: 0,
            ip: 0,
        }
    }

    pub fn step(&mut self) -> ProgramState {
        let op = match self.program.op(self.ip) {
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

    pub fn run_until_repeat_or_done(&mut self) -> ProgramState {
        let mut seen = BTreeSet::new();
        let mut state = ProgramState::Running;
        while !seen.contains(&self.ip) {
            seen.insert(self.ip);
            state = self.step();
        }
        state
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ProgramState {
    Running,
    Complete,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl Instruction {
    pub fn offset(&self) -> i64 {
        match self {
            Instruction::Jmp(i) => *i,
            _ => 1,
        }
    }
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
        let mut vm = VirtualMachine::new(TEST_INPUT.parse().unwrap());
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
        let mut vm = VirtualMachine::new(TEST_INPUT.parse().unwrap());
        let result = vm.run_until_repeat_or_done();
        assert_eq!(result, ProgramState::Running);
        assert_eq!((vm.ip, vm.accumulator), (1, 5));
    }

    #[test]
    fn fix_and_run() {
        let mut program: Program = TEST_INPUT.parse().unwrap();
        program.fix().unwrap();
        let mut vm = VirtualMachine::new(program);
        let result = vm.run_until_repeat_or_done();
        assert_eq!(result, ProgramState::Complete);
        assert_eq!(vm.accumulator, 8);
    }
}
