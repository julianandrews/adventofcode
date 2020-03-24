use crate::aoc_error::AOCError;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

pub type RegisterValue = i64;

type Address = usize;
type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum ValueMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum OpType {
    Add = 1,
    Multiply = 2,
    Store = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    AdjustRelOffset = 9,
    Halt = 99,
}

impl OpType {
    fn num_params(&self) -> Address {
        match self {
            OpType::Add => 3,
            OpType::Multiply => 3,
            OpType::Store => 1,
            OpType::Output => 1,
            OpType::JumpIfTrue => 2,
            OpType::JumpIfFalse => 2,
            OpType::LessThan => 3,
            OpType::Equals => 3,
            OpType::AdjustRelOffset => 1,
            OpType::Halt => 0,
        }
    }
}

pub struct VM<I: Iterator<Item = RegisterValue>> {
    memory: VMMemory,
    inputs: I,
    ip: Address,
    relative_base: Address,
    output: Option<RegisterValue>,
}

impl<I: Iterator<Item = RegisterValue>> VM<I> {
    pub fn new(program: Vec<RegisterValue>, inputs: I) -> VM<I> {
        VM {
            memory: VMMemory { memory: program },
            inputs: inputs,
            ip: 0,
            relative_base: 0,
            output: None,
        }
    }

    pub fn step(&mut self) -> Result<(OpType, Vec<RegisterValue>)> {
        let op_type = self.get_op_type()?;
        let params = self.get_params(op_type.num_params());
        let modes = self.get_modes(op_type.num_params())?;
        let mut ip_offset = 1 + op_type.num_params();

        if params.len() != op_type.num_params() {
            return Err(AOCError::new("Incorrect number of parameters"))?;
        } else if modes.len() != op_type.num_params() {
            return Err(AOCError::new("Incorrect number of modes"))?;
        }

        log::trace!("Executing {:?} with {:?} and {:?}", op_type, params, modes);

        match op_type {
            OpType::Add | OpType::Multiply | OpType::LessThan | OpType::Equals => {
                if modes[2] == ValueMode::Immediate {
                    return Err(AOCError::new(&format!(
                        "Unexpected {:?} in {:?} at 0",
                        modes[2], op_type
                    )))?;
                }
                let a = self.get_value(params[0], modes[0])?;
                let b = self.get_value(params[1], modes[1])?;
                let address = self.get_address(params[2], modes[2])?;
                match op_type {
                    OpType::Add => {
                        log::trace!("Storing {} + {} at {}", a, b, address);
                        self.memory[address] = a + b;
                    }
                    OpType::Multiply => {
                        log::trace!("Storing {} * {} at {}", a, b, address);
                        self.memory[address] = a * b;
                    }
                    OpType::LessThan => {
                        log::trace!("Storing {} < {} at {}", a, b, address);
                        self.memory[address] = if a < b { 1 } else { 0 }
                    }
                    OpType::Equals => {
                        log::trace!("Storing {} == {} at {}", a, b, address);
                        self.memory[address] = if a == b { 1 } else { 0 }
                    }
                    _ => return Err(AOCError::new(&format!("Unexpected Op {:?}", op_type)))?,
                }
            }
            OpType::Store => {
                if modes[0] == ValueMode::Immediate {
                    return Err(AOCError::new(&format!(
                        "Unexpected {:?} in {:?} at 0",
                        modes[0], op_type
                    )))?;
                }
                let address = self.get_address(params[0], modes[0])?;
                let value = self
                    .inputs
                    .next()
                    .ok_or(AOCError::new("Failed to get input"))?;
                log::trace!("Storing {} at {}", value, address);
                self.memory[address] = value;
            }
            OpType::Output => {
                let value = self.get_value(params[0], modes[0])?;
                log::trace!("Outputting {:?}", value);
                self.output = Some(value);
            }
            OpType::JumpIfTrue | OpType::JumpIfFalse => {
                let value = self.get_value(params[0], modes[0])?;
                let address = Address::try_from(self.get_value(params[1], modes[1])?)?;
                log::trace!("value: {:?} address: {:?}", value, address);
                let should_jump = (op_type == OpType::JumpIfTrue && value != 0)
                    || (op_type == OpType::JumpIfFalse && value == 0);
                if should_jump {
                    log::trace!("Jumping to {:?}", address);
                    self.ip = address;
                    ip_offset = 0;
                }
            }
            OpType::AdjustRelOffset => {
                let value = self.get_value(params[0], modes[0])?;
                log::trace!("Adjusting relative base by {:?}", value);
                self.relative_base =
                    Address::try_from(RegisterValue::try_from(self.relative_base)? + value)?;
            }
            OpType::Halt => log::trace!("Halting"),
        }

        self.ip += ip_offset;

        Ok((op_type, params))
    }

    pub fn diagnostic_code(&self) -> RegisterValue {
        self.memory[0]
    }

    pub fn outputs<'a>(&'a mut self) -> impl Iterator<Item = RegisterValue> + 'a {
        OutputIterator { vm: self }
    }

    pub fn set_memory(&mut self, index: Address, value: RegisterValue) {
        log::trace!("Setting memory at {} to {}", index, value);
        self.memory[index] = value;
    }

    pub fn memory(&self) -> &[RegisterValue] {
        &self.memory[..]
    }

    pub fn last_output(&self) -> Option<RegisterValue> {
        self.output
    }

    fn get_op_type(&self) -> Result<OpType> {
        Ok(OpType::try_from(u8::try_from(self.memory[self.ip] % 100)?)?)
    }

    fn get_params(&self, n: usize) -> Vec<RegisterValue> {
        (self.ip + 1..self.ip + n + 1)
            .map(|i| self.memory[i])
            .collect()
    }

    fn get_modes(&self, n: usize) -> Result<Vec<ValueMode>> {
        std::iter::successors(Some(self.memory[self.ip] / 100), |x| Some(x / 10))
            .map(|x| Ok(ValueMode::try_from(u8::try_from(x % 10)?)?))
            .take(n)
            .collect()
    }

    fn get_value(&self, value: RegisterValue, mode: ValueMode) -> Result<RegisterValue> {
        match mode {
            ValueMode::Position => Ok(self.memory[Address::try_from(value)?]),
            ValueMode::Immediate => Ok(value),
            ValueMode::Relative => Ok(self.memory
                [Address::try_from(RegisterValue::try_from(self.relative_base)? + value)?]),
        }
    }

    fn get_address(&self, base_address: RegisterValue, mode: ValueMode) -> Result<Address> {
        match mode {
            ValueMode::Position => Ok(Address::try_from(base_address)?),
            _ => Ok(Address::try_from(
                RegisterValue::try_from(self.relative_base)? + base_address,
            )?),
        }
    }
}

struct OutputIterator<'a, I: Iterator<Item = RegisterValue>> {
    vm: &'a mut VM<I>,
}

impl<'a, I: Iterator<Item = RegisterValue>> Iterator for OutputIterator<'a, I> {
    type Item = RegisterValue;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (op_type, _) = self.vm.step().unwrap();
            match op_type {
                OpType::Output => return self.vm.output,
                OpType::Halt => return None,
                _ => {}
            }
        }
    }
}

struct VMMemory {
    memory: Vec<RegisterValue>,
}

impl std::ops::Index<Address> for VMMemory {
    type Output = RegisterValue;

    fn index(&self, index: Address) -> &Self::Output {
        if index < self.memory.len() {
            &self.memory[index]
        } else {
            &0
        }
    }
}

impl std::ops::Index<std::ops::RangeFull> for VMMemory {
    type Output = [RegisterValue];

    fn index(&self, index: std::ops::RangeFull) -> &[RegisterValue] {
        &self.memory[index]
    }
}

impl std::ops::IndexMut<Address> for VMMemory {
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }
        &mut self.memory[index]
    }
}
