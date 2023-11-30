use z3::ast::{Ast, BV};

use aoc::utils::{get_input, parse_fields};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = get_input()?;
    let program: Vec<Instruction> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&program)?);
    println!("Part 2: {}", part2(&program)?);

    Ok(())
}

fn part1(program: &[Instruction]) -> Result<i64> {
    solve_constraints(program, Register::Z, 0, Goal::Maximize)
        .ok_or(anyhow!("Failed to find valid model number"))
}

fn part2(program: &[Instruction]) -> Result<i64> {
    solve_constraints(program, Register::Z, 0, Goal::Minimize)
        .ok_or(anyhow!("Failed to find valid model number"))
}

fn solve_constraints(
    program: &[Instruction],
    register: Register,
    expected: i64,
    goal: Goal,
) -> Option<i64> {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Optimize::new(&ctx);
    let zero = BV::from_i64(&ctx, 0, 64);
    let one = BV::from_i64(&ctx, 1, 64);
    let inputs: Vec<_> = (0..14)
        .map(|i| BV::new_const(&ctx, format!("input_{}", i), 64))
        .collect();
    for digit in &inputs {
        solver.assert(&digit.bvsgt(&zero));
        solver.assert(&digit.bvsle(&BV::from_i64(&ctx, 9, 64)));
    }

    // We will step through, updating the registers with each instruction to build our constraints.
    let mut registers: [BV; 4] = [zero.clone(), zero.clone(), zero.clone(), zero.clone()];
    let mut input_iter = inputs.iter();
    for (i, instruction) in program.iter().enumerate() {
        let output = BV::new_const(&ctx, format!("output_{}", i), 64);
        match instruction {
            Instruction::Inp(a) => registers[*a as usize] = input_iter.next()?.clone(),
            Instruction::Add(a, b) => {
                let value = match b {
                    Value::Register(b) => registers[*b as usize].clone(),
                    Value::Number(b) => BV::from_i64(&ctx, *b, 64),
                };
                solver.assert(&output._eq(&(registers[*a as usize].bvadd(&value))));
                registers[*a as usize] = output;
            }
            Instruction::Mul(a, b) => {
                let value = match b {
                    Value::Register(b) => registers[*b as usize].clone(),
                    Value::Number(b) => BV::from_i64(&ctx, *b, 64),
                };
                solver.assert(&output._eq(&(registers[*a as usize].bvmul(&value))));
                registers[*a as usize] = output;
            }
            Instruction::Div(a, b) => {
                let value = match b {
                    Value::Register(b) => registers[*b as usize].clone(),
                    Value::Number(b) => BV::from_i64(&ctx, *b, 64),
                };
                solver.assert(&output._eq(&registers[*a as usize].bvsdiv(&value)));
                registers[*a as usize] = output;
            }
            Instruction::Mod(a, b) => {
                let value = match b {
                    Value::Register(b) => registers[*b as usize].clone(),
                    Value::Number(b) => BV::from_i64(&ctx, *b, 64),
                };
                solver.assert(&output._eq(&registers[*a as usize].bvsmod(&value)));
                registers[*a as usize] = output;
            }
            Instruction::Eql(a, b) => {
                let value = match b {
                    Value::Register(b) => registers[*b as usize].clone(),
                    Value::Number(b) => BV::from_i64(&ctx, *b, 64),
                };
                solver.assert(&output._eq(&(registers[*a as usize]._eq(&value).ite(&one, &zero))));
                registers[*a as usize] = output;
            }
        };
    }
    solver.assert(&registers[register as usize]._eq(&BV::from_i64(&ctx, expected, 64)));
    let model_number = BV::new_const(&ctx, "model_number", 64);
    solver.assert(
        &model_number._eq(
            &(0..14)
                .map(|i| inputs[i].bvmul(&BV::from_u64(&ctx, 10_i64.pow(13 - i as u32) as u64, 64)))
                .fold(zero, |result, n| result.bvadd(&n)),
        ),
    );
    match goal {
        Goal::Minimize => solver.minimize(&model_number),
        Goal::Maximize => solver.maximize(&model_number),
    }
    match solver.check(&[]) {
        z3::SatResult::Sat => solver.get_model()?.eval(&model_number, true)?.as_i64(),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
enum Goal {
    Minimize,
    Maximize,
}

#[derive(Debug, Clone, Copy)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl std::str::FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Register::W),
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            _ => Err(anyhow!("Failed to parse register {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Register(Register),
    Number(i64),
}

impl std::str::FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Register>() {
            Ok(register) => Ok(Value::Register(register)),
            Err(_) => Ok(Value::Number(s.parse()?)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(Register),
    Add(Register, Value),
    Mul(Register, Value),
    Div(Register, Value),
    Mod(Register, Value),
    Eql(Register, Value),
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, args) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Failed to parse instruction {}", s))?;

        match instruction {
            "inp" => Ok(Instruction::Inp(args.parse()?)),
            _ => {
                let (a, b) = args
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("Failed to parse arguments {}", args))?;
                match instruction {
                    "add" => Ok(Instruction::Add(a.parse()?, b.parse()?)),
                    "mul" => Ok(Instruction::Mul(a.parse()?, b.parse()?)),
                    "div" => Ok(Instruction::Div(a.parse()?, b.parse()?)),
                    "mod" => Ok(Instruction::Mod(a.parse()?, b.parse()?)),
                    "eql" => Ok(Instruction::Eql(a.parse()?, b.parse()?)),
                    _ => Err(anyhow!("Unrecognized instruction {}", instruction)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver() {
        let input = "\
            inp z\n\
            eql z 5\n\
            eql z 0";
        let program: Vec<Instruction> = parse_fields(input, '\n').unwrap();
        assert_eq!(
            solve_constraints(&program, Register::Z, 0, Goal::Maximize),
            Some(59999999999999)
        );
    }

    #[test]
    fn negate() {
        let input = "\
            inp x\n\
            mul x -1";
        let program: Vec<Instruction> = parse_fields(input, '\n').unwrap();
        for value in 1..=9 {
            let result = solve_constraints(&program, Register::X, -value, Goal::Maximize);
            assert_eq!(result, Some(9999999999999 + value * 10_i64.pow(13)));
        }
    }

    #[test]
    fn is_three_times_larger() {
        let input = "\
            inp z\n\
            inp x\n\
            mul z 3\n\
            eql z x";
        let program: Vec<Instruction> = parse_fields(input, '\n').unwrap();
        let result = solve_constraints(&program, Register::Z, 1, Goal::Maximize);
        assert_eq!(result, Some(39999999999999));
        let result = solve_constraints(&program, Register::Z, 1, Goal::Minimize);
        assert_eq!(result, Some(13111111111111));
    }
}
