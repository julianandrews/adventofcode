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
    _inputs: I, // TODO: Change this once used!
    ip: Address,
    relative_base: Address,
    output: Option<RegisterValue>,
}

impl<I: Iterator<Item = RegisterValue>> VM<I> {
    pub fn new(program: Vec<RegisterValue>, inputs: I) -> VM<I> {
        VM {
            memory: VMMemory { memory: program },
            _inputs: inputs, // TODO: Change this once used!
            ip: 0,
            relative_base: 0,
            output: None,
        }
    }

    pub fn step(&mut self) -> Result<(OpType, Vec<RegisterValue>)> {
        let op_type = self.get_op_type()?;
        let params = self.get_params(op_type.num_params());
        let modes = self.get_modes(op_type.num_params())?;

        log::trace!("Executing {:?} with {:?} and {:?}", op_type, params, modes);

        match op_type {
            OpType::Add | OpType::Multiply => {
                if params.len() != 3 {
                    return Err(AOCError::new("Incorrect number of parameters"))?;
                } else if modes.len() != 3 {
                    return Err(AOCError::new("Incorrect number of modes"))?;
                }
                let a = self.get_value(params[0], modes[0]);
                let b = self.get_value(params[1], modes[1]);
                let address = self.get_address(params[2], modes[2]);
                match op_type {
                    OpType::Add => {
                        log::trace!("Storing {} + {} at {}", a, b, address);
                        self.memory[address] = a + b;
                    }
                    OpType::Multiply => {
                        log::trace!("Storing {} * {} at {}", a, b, address);
                        self.memory[address] = a * b;
                    }
                    _ => return Err(AOCError::new(&format!("Unexpected Op {:?}", op_type)))?,
                }
            }
            OpType::Halt => log::trace!("Halting"),
            // TODO: Implement other operations!
            _ => unimplemented!(),
        }

        match op_type {
            OpType::JumpIfTrue | OpType::JumpIfFalse => {}
            _ => self.ip += 1 + op_type.num_params(),
        }

        Ok((op_type, params))
    }

    pub fn diagnostic_code(&self) -> RegisterValue {
        self.memory[0]
    }

    pub fn last_output(&self) -> Option<RegisterValue> {
        self.output
    }

    pub fn set_memory(&mut self, index: Address, value: RegisterValue) {
        log::trace!("Setting memory at {} to {}", index, value);
        self.memory[index] = value;
    }

    pub fn memory(&self) -> &[RegisterValue] {
        &self.memory[..]
    }

    fn get_op_type(&self) -> Result<OpType> {
        Ok(OpType::try_from(u8::try_from(self.memory[self.ip])?)?)
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

    fn get_value(&self, value: RegisterValue, mode: ValueMode) -> RegisterValue {
        // TODO: Make `as Address` casts safe
        match mode {
            ValueMode::Position => self.memory[value as Address],
            ValueMode::Immediate => value,
            ValueMode::Relative => self.memory[self.relative_base + (value as Address)],
        }
    }

    fn get_address(&self, base_address: RegisterValue, mode: ValueMode) -> Address {
        // TODO: Make `as Address` cast safe
        match mode {
            ValueMode::Position => base_address as Address,
            _ => base_address as Address + self.relative_base,
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
