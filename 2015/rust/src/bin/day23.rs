fn main() -> anyhow::Result<()> {
    let input = aoc::utils::get_input()?;
    let program: Vec<Instruction> = aoc::utils::parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&program));
    println!("Part 2: {}", part2(&program));

    Ok(())
}

fn part1(program: &[Instruction]) -> u64 {
    let mut vm = VM::new(program);
    vm.run();
    vm.registers[Register::B]
}

fn part2(program: &[Instruction]) -> u64 {
    let mut vm = VM::new(program);
    vm.registers[Register::A] = 1;
    vm.run();
    vm.registers[Register::B]
}

#[derive(Debug, Clone)]
struct VM<'a> {
    program: &'a [Instruction],
    instruction_ptr: i64,
    registers: Registers,
}

impl<'a> VM<'a> {
    fn new(program: &[Instruction]) -> VM {
        VM {
            program,
            instruction_ptr: 0,
            registers: Registers::default(),
        }
    }

    fn run(&mut self) {
        while let Some(instruction) = self.current_instruction() {
            self.execute(*instruction);
        }
    }

    fn current_instruction(&self) -> Option<&Instruction> {
        let ip: usize = self.instruction_ptr.try_into().ok()?;
        self.program.get(ip)
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Hlf(register) => self.registers[register] /= 2,
            Instruction::Tpl(register) => self.registers[register] *= 3,
            Instruction::Inc(register) => self.registers[register] += 1,
            Instruction::Jmp(jump) => self.instruction_ptr += jump - 1,
            Instruction::Jie(register, jump) => {
                if self.registers[register] % 2 == 0 {
                    self.instruction_ptr += jump - 1;
                }
            }
            Instruction::Jio(register, jump) => {
                if self.registers[register] == 1 {
                    self.instruction_ptr += jump - 1;
                }
            }
        }
        self.instruction_ptr += 1;
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
}

#[derive(Debug, Clone, Copy, Default)]
struct Registers([u64; 2]);

impl std::ops::Index<Register> for Registers {
    type Output = u64;

    fn index(&self, index: Register) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl std::ops::IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i64),
    Jie(Register, i64),
    Jio(Register, i64),
}

mod parsing {
    use super::{Instruction, Register};

    use anyhow::anyhow;

    impl std::str::FromStr for Instruction {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (instruction, args) = s
                .split_once(' ')
                .ok_or_else(|| anyhow!("Invalid instruction {}.", s))?;
            match instruction {
                "hlf" => Ok(Instruction::Hlf(args.parse()?)),
                "tpl" => Ok(Instruction::Tpl(args.parse()?)),
                "inc" => Ok(Instruction::Inc(args.parse()?)),
                "jmp" => Ok(Instruction::Jmp(args.parse()?)),
                "jie" | "jio" => {
                    let (register, offset) = args
                        .split_once(", ")
                        .ok_or_else(|| anyhow!("Invalid instruction {}.", s))?;
                    match instruction {
                        "jie" => Ok(Instruction::Jie(register.parse()?, offset.parse()?)),
                        "jio" => Ok(Instruction::Jio(register.parse()?, offset.parse()?)),
                        _ => unreachable!(),
                    }
                }
                _ => Err(anyhow!("Invalid instruction {}.", s)),
            }
        }
    }

    impl std::str::FromStr for Register {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "a" => Ok(Register::A),
                "b" => Ok(Register::B),
                _ => Err(anyhow!("Unrecognized register {}.", s)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction, Register, VM};

    static TEST_DATA: &str = "\
        inc a\n\
        jio a, +2\n\
        tpl a\n\
        inc a";

    #[test]
    fn run_program() {
        let program: Vec<Instruction> = aoc::utils::parse_fields(TEST_DATA, '\n').unwrap();
        let mut vm = VM::new(&program);
        vm.run();

        assert_eq!(vm.registers[Register::A], 2);
    }
}
