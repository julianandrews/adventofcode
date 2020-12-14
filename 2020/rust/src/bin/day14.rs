#![feature(str_split_once)]

use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let program = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&program));
    println!("Part 2: {}", part2(&program));
    Ok(())
}

fn part1(program: &[Instruction]) -> u64 {
    let mut vm = Vm::new();
    vm.run_program(program);
    vm.mem.values().sum()
}

fn part2(program: &[Instruction]) -> u64 {
    let mut vm = Vm::new();
    vm.really_run_program(program);
    vm.mem.values().sum()
}

struct Vm {
    mask: Mask,
    mem: HashMap<u64, u64>,
}

impl Vm {
    fn new() -> Self {
        Vm {
            mask: Mask::new(),
            mem: HashMap::new(),
        }
    }

    fn run_program(&mut self, program: &[Instruction]) {
        for instruction in program {
            match instruction {
                Instruction::SetMask(mask) => self.mask = *mask,
                Instruction::SetMem(address, value) => {
                    self.mem.insert(*address, self.mask.apply(*value));
                }
            }
        }
    }

    fn really_run_program(&mut self, program: &[Instruction]) {
        for instruction in program {
            match instruction {
                Instruction::SetMask(mask) => self.mask = *mask,
                Instruction::SetMem(address, value) => {
                    for masked_address in self.mask.addresses(*address) {
                        self.mem.insert(masked_address, *value);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    SetMask(Mask),
    SetMem(u64, u64),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (type_part, value_part) = s
            .split_once(" = ")
            .ok_or(AOCError::new("Invalid instruction"))?;
        if type_part == "mask" {
            let mask = value_part.parse()?;
            Ok(Instruction::SetMask(mask))
        } else {
            if !(type_part.starts_with("mem[") && type_part.ends_with("]")) {
                return Err(AOCError::new("Invalid instruction").into());
            }
            let address = type_part[4..type_part.len() - 1].parse()?;
            let value = value_part.parse()?;
            Ok(Instruction::SetMem(address, value))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Mask {
    and_mask: u64,
    or_mask: u64,
    x_mask: u64,
}

impl Mask {
    fn new() -> Self {
        Mask {
            and_mask: 1 << 36 - 1,
            or_mask: 0,
            x_mask: 1 << 36 - 1,
        }
    }

    fn apply(&self, value: u64) -> u64 {
        (value & self.and_mask) | self.or_mask
    }

    fn addresses(&self, address: u64) -> impl Iterator<Item = u64> {
        let base_address = (address | self.or_mask) & !self.x_mask;
        let floating_bits: Vec<_> = (0..=36)
            .map(|i| 1 << i)
            .filter(|x| x & self.x_mask != 0)
            .collect();

        powerset(floating_bits).map(move |bits| {
            bits.into_iter()
                .fold(base_address, |address, bit| address | bit)
        })
    }
}

impl FromStr for Mask {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let and_mask = u64::from_str_radix(&s.replace("X", "1"), 2)?;
        let or_mask = u64::from_str_radix(&s.replace("X", "0"), 2)?;
        let x_mask = u64::from_str_radix(&s.replace("1", "0").replace("X", "1"), 2)?;
        Ok(Mask {
            and_mask,
            or_mask,
            x_mask,
        })
    }
}

fn powerset<T: Clone>(values: Vec<T>) -> impl Iterator<Item = Vec<T>> {
    (0..=values.len())
        .map(move |count| values.clone().into_iter().combinations(count))
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_program() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\
                   \nmem[8] = 11\
                   \nmem[7] = 101\
                   \nmem[8] = 0";
        let program = parse_fields(input, '\n').unwrap();
        let mut vm = Vm::new();
        vm.run_program(&program);

        assert_eq!(vm.mem.get(&7), Some(&101));
        assert_eq!(vm.mem.get(&8), Some(&64));
    }

    #[test]
    fn really_run_program() {
        let input = "mask = 000000000000000000000000000000X1001X\
                   \nmem[42] = 100\
                   \nmask = 00000000000000000000000000000000X0XX\
                   \nmem[26] = 1";
        let program = parse_fields(input, '\n').unwrap();
        let mut vm = Vm::new();
        vm.really_run_program(&program);

        println!("{:?}", vm.mem);
        assert_eq!(vm.mem.values().sum::<u64>(), 208);
    }
}
