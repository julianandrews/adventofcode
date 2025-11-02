use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let vm: VM = input.trim().parse()?;

    println!("Part 1: {}", part1(vm.clone())?);
    println!("Part 2: {}", part2(vm.clone())?);

    Ok(())
}

fn part1(mut vm: VM) -> Result<String> {
    let outputs = vm.outputs().collect::<Result<Vec<_>>>()?;
    Ok(outputs.iter().join(","))
}

fn part2(vm: VM) -> Result<u64> {
    solve_for_initial_r_a(vm)?.ok_or_else(|| anyhow!("Failed to find value for register A"))
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct VM {
    r_a: u64,
    r_b: u64,
    r_c: u64,
    ip: usize,
    program: Vec<u64>,
}

impl VM {
    fn outputs(&mut self) -> impl Iterator<Item = Result<u64>> + use<'_> {
        std::iter::from_fn(|| self.next_output().transpose())
    }

    fn next_output(&mut self) -> Result<Option<u64>> {
        while let Some((operator, operand)) = self.next_instruction()? {
            if let Some(output) = self.execute(operator, operand)? {
                return Ok(Some(output));
            }
        }
        Ok(None)
    }

    fn next_instruction(&self) -> Result<Option<(Instruction, u64)>> {
        let (operator, operand): (Instruction, u64) = match self.program.get(self.ip..self.ip + 2) {
            Some([a, b]) => ((*a).try_into()?, *b),
            _ => return Ok(None),
        };
        Ok(Some((operator, operand)))
    }

    fn execute(&mut self, operator: Instruction, operand: u64) -> Result<Option<u64>> {
        let mut output = None;
        match operator {
            Instruction::Adv => self.r_a >>= self.combo_operand(operand)?,
            Instruction::Bxl => self.r_b ^= operand,
            Instruction::Bst => self.r_b = self.combo_operand(operand)? % 8,
            Instruction::Jnz => {
                if self.r_a != 0 {
                    self.ip = operand as usize
                } else {
                    self.ip += 2;
                }
            }
            Instruction::Bxc => self.r_b ^= self.r_c,
            Instruction::Out => output = Some(self.combo_operand(operand)? % 8),
            Instruction::Bdv => self.r_b = self.r_a >> self.combo_operand(operand)?,
            Instruction::Cdv => self.r_c = self.r_a >> self.combo_operand(operand)?,
        }
        if operator != Instruction::Jnz {
            self.ip += 2;
        }
        Ok(output)
    }

    fn combo_operand(&self, value: u64) -> Result<u64> {
        match value {
            0..=3 => Ok(value),
            4 => Ok(self.r_a),
            5 => Ok(self.r_b),
            6 => Ok(self.r_c),
            _ => bail!("Unrecognized combo operand {value}"),
        }
    }
}

fn solve_for_initial_r_a(mut vm: VM) -> Result<Option<u64>> {
    // Assumption: the program runs in a loop where each output depends only on the least 3 bits of r_a
    let mut stack = vec![(vm.program.len(), 0u64)];
    while let Some((i, r)) = stack.pop() {
        if i == 0 {
            return Ok(Some(r));
        }
        let i = i - 1;
        // By going in reverse our DFS will explore smaller solutions before larger ones.
        for least_three_bits in (0..8).rev() {
            let a = r << 3 | least_three_bits;
            (vm.r_a, vm.ip) = (a, 0);
            if vm.next_output()? == Some(vm.program[i]) {
                stack.push((i, a));
            }
        }
    }
    Ok(None)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u64> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::Adv),
            1 => Ok(Instruction::Bxl),
            2 => Ok(Instruction::Bst),
            3 => Ok(Instruction::Jnz),
            4 => Ok(Instruction::Bxc),
            5 => Ok(Instruction::Out),
            6 => Ok(Instruction::Bdv),
            7 => Ok(Instruction::Cdv),
            _ => bail!("Failed to parse operator {}", value),
        }
    }
}

mod parsing {
    use crate::VM;
    use anyhow::{anyhow, bail, Result};

    impl std::str::FromStr for VM {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            let (register_part, program_part) = s
                .split_once("\n\n")
                .ok_or_else(|| anyhow!("Failed to parse input"))?;
            let lines: Vec<_> = register_part.lines().collect();
            if lines.len() != 3 {
                bail!("Failed to parse VM. Expected 3 lines in {}", s);
            }
            let r_a = parse_register(lines[0], "Register A: ")?;
            let r_b = parse_register(lines[1], "Register B: ")?;
            let r_c = parse_register(lines[2], "Register C: ")?;

            let program_part = program_part
                .strip_prefix("Program: ")
                .ok_or_else(|| anyhow!("Invalid program."))?;
            let program: Vec<_> = aoc::utils::parse_fields(program_part, ',')?;

            Ok(VM {
                r_a,
                r_b,
                r_c,
                ip: 0,
                program,
            })
        }
    }

    fn parse_register(line: &str, prefix: &str) -> Result<u64> {
        Ok(line
            .strip_prefix(prefix)
            .ok_or_else(|| anyhow!("Failed to parse VM"))?
            .parse()?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_for_initial_r_a, VM};
    use anyhow::Result;

    static EXAMPLE_1: &str = "\
        Register A: 729\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,1,5,4,3,0";

    static EXAMPLE_2: &str = "\
        Register A: 2024\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,3,5,4,3,0";

    #[test]
    fn parsing() {
        let vm: VM = EXAMPLE_1.parse().unwrap();
        let expected = VM {
            r_a: 729,
            r_b: 0,
            r_c: 0,
            ip: 0,
            program: vec![0, 1, 5, 4, 3, 0],
        };
        assert_eq!(vm, expected);
    }

    #[test]
    fn simple_program_1() {
        let mut vm = VM {
            r_a: 0,
            r_b: 0,
            r_c: 9,
            ip: 0,
            program: vec![2, 6],
        };
        let _outputs = vm.outputs().collect::<Result<Vec<_>>>().unwrap();
        assert_eq!(vm.r_b, 1);
    }

    #[test]
    fn simple_program_2() {
        let mut vm = VM {
            r_a: 10,
            r_b: 0,
            r_c: 0,
            ip: 0,
            program: vec![5, 0, 5, 1, 5, 4],
        };
        let outputs = vm.outputs().collect::<Result<Vec<_>>>().unwrap();
        assert_eq!(outputs, vec![0, 1, 2]);
    }

    #[test]
    fn simple_program_3() {
        let mut vm = VM {
            r_a: 2024,
            r_b: 0,
            r_c: 0,
            ip: 0,
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let outputs = vm.outputs().collect::<Result<Vec<_>>>().unwrap();
        assert_eq!(outputs, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(vm.r_a, 0);
    }

    #[test]
    fn simple_program_4() {
        let mut vm = VM {
            r_a: 0,
            r_b: 29,
            r_c: 0,
            ip: 0,
            program: vec![1, 7],
        };
        let _outputs = vm.outputs().collect::<Result<Vec<_>>>().unwrap();
        assert_eq!(vm.r_b, 26);
    }

    #[test]
    fn simple_program_5() {
        let mut vm = VM {
            r_a: 0,
            r_b: 2024,
            r_c: 43690,
            ip: 0,
            program: vec![4, 0],
        };
        let _outputs = vm.outputs().collect::<Result<Vec<_>>>().unwrap();
        assert_eq!(vm.r_b, 44354);
    }

    #[test]
    fn output() {
        let mut vm: VM = EXAMPLE_1.parse().unwrap();
        let outputs = vm.outputs().collect::<Result<Vec<_>>>().unwrap();
        let expected = [4, 6, 3, 5, 6, 3, 5, 2, 1, 0];
        assert_eq!(outputs, expected);
    }

    #[test]
    fn example_2_works() {
        let mut vm: VM = EXAMPLE_2.parse().unwrap();
        vm.r_a = 117440;
        let outputs = vm.outputs().collect::<Result<Vec<_>>>().unwrap();
        let expected = vm.program;
        assert_eq!(outputs, expected);
    }

    #[test]
    fn solve() {
        let vm: VM = EXAMPLE_2.parse().unwrap();
        let value = solve_for_initial_r_a(vm).unwrap();
        assert_eq!(value, Some(117440));
    }
}
