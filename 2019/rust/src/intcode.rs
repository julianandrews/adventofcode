use anyhow::anyhow;
use num_enum::TryFromPrimitive;

use std::convert::TryFrom;

use anyhow::Result;

pub type RegisterValue = i64;

type Address = usize;

pub fn parse_program(input: &str) -> Result<Vec<RegisterValue>> {
    Ok(input
        .trim()
        .split(',')
        .map(|s| s.parse::<RegisterValue>())
        .collect::<std::result::Result<Vec<_>, _>>()?)
}

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

pub struct VM<'a> {
    memory: VMMemory,
    inputs: Option<Box<dyn Iterator<Item = RegisterValue> + 'a>>,
    ip: Address,
    relative_base: Address,
    output: Option<RegisterValue>,
}

// https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999
pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

impl<'a> VM<'a> {
    pub fn new(
        program: Vec<RegisterValue>,
        inputs: Option<Box<dyn Iterator<Item = RegisterValue> + 'a>>,
    ) -> VM {
        VM {
            memory: VMMemory { memory: program },
            inputs,
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
            OpType::Add => self.add(&params, &modes)?,
            OpType::Multiply => self.multiply(&params, &modes)?,
            OpType::LessThan => self.less_than(&params, &modes)?,
            OpType::Equals => self.equals(&params, &modes)?,
            OpType::Store => self.store(&params, &modes)?,
            OpType::Output => self.output(&params, &modes)?,
            OpType::JumpIfTrue => self.jump_if_true(&params, &modes)?,
            OpType::JumpIfFalse => self.jump_if_false(&params, &modes)?,
            OpType::AdjustRelOffset => self.adjust_rel_offset(&params, &modes)?,
            OpType::Halt => self.halt(&params, &modes)?,
        }

        Ok((op_type, params))
    }

    pub fn diagnostic_code(&self) -> RegisterValue {
        self.memory[0]
    }

    pub fn outputs<'b>(&'b mut self) -> impl Iterator<Item = RegisterValue> + Captures<'a> + 'b {
        OutputIterator { vm: self }
    }

    pub fn set_inputs(&mut self, inputs: Option<Box<dyn Iterator<Item = RegisterValue> + 'a>>) {
        self.inputs = inputs;
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

    fn binary_operands(
        &self,
        params: &[RegisterValue],
        modes: &[ValueMode],
    ) -> Result<(RegisterValue, RegisterValue, Address)> {
        if modes[2] == ValueMode::Immediate {
            return Err(anyhow!(
                "Unexpected mode {:?} at 0 for binary operation",
                modes[2]
            ));
        }
        Ok((
            self.get_value(params[0], modes[0])?,
            self.get_value(params[1], modes[1])?,
            self.get_address(params[2], modes[2])?,
        ))
    }

    fn jump_operands(
        &self,
        params: &[RegisterValue],
        modes: &[ValueMode],
    ) -> Result<(RegisterValue, Address)> {
        Ok((
            self.get_value(params[0], modes[0])?,
            Address::try_from(self.get_value(params[1], modes[1])?)?,
        ))
    }

    fn add(&mut self, params: &[RegisterValue], modes: &[ValueMode]) -> Result<()> {
        let (a, b, address) = self.binary_operands(params, modes)?;
        log::trace!("Storing {} + {} at {}", a, b, address);
        self.memory[address] = a + b;
        self.ip += params.len() + 1;

        Ok(())
    }

    fn multiply(&mut self, params: &[RegisterValue], modes: &[ValueMode]) -> Result<()> {
        let (a, b, address) = self.binary_operands(params, modes)?;
        log::trace!("Storing {} * {} at {}", a, b, address);
        self.memory[address] = a * b;
        self.ip += params.len() + 1;

        Ok(())
    }

    fn less_than(&mut self, params: &[RegisterValue], modes: &[ValueMode]) -> Result<()> {
        let (a, b, address) = self.binary_operands(params, modes)?;
        log::trace!("Storing {} < {} at {}", a, b, address);
        self.memory[address] = if a < b { 1 } else { 0 };
        self.ip += params.len() + 1;

        Ok(())
    }

    fn equals(&mut self, params: &[RegisterValue], modes: &[ValueMode]) -> Result<()> {
        let (a, b, address) = self.binary_operands(params, modes)?;
        log::trace!("Storing {} == {} at {}", a, b, address);
        self.memory[address] = if a == b { 1 } else { 0 };
        self.ip += params.len() + 1;

        Ok(())
    }

    fn store(&mut self, params: &[RegisterValue], modes: &[ValueMode]) -> Result<()> {
        if modes[0] == ValueMode::Immediate {
            return Err(anyhow!("Unexpected mode {:?} at 0", modes[0]));
        }
        let address = self.get_address(params[0], modes[0])?;
        let value = self
            .inputs
            .as_mut()
            .ok_or(anyhow!("Inputs not provided"))?
            .next()
            .ok_or(anyhow!("Failed to get input"))?;
        log::trace!("Storing {} at {}", value, address);
        self.memory[address] = value;
        self.ip += params.len() + 1;

        Ok(())
    }

    fn output(&mut self, params: &[RegisterValue], modes: &[ValueMode]) -> Result<()> {
        let value = self.get_value(params[0], modes[0])?;
        log::trace!("Outputting {:?}", value);
        self.output = Some(value);
        self.ip += params.len() + 1;

        Ok(())
    }

    fn jump_if_true(&mut self, params: &[RegisterValue], modes: &[ValueMode]) -> Result<()> {
        let (value, address) = self.jump_operands(params, modes)?;
        if value != 0 {
            log::trace!("Jumping to {:?}", address);
            self.ip = address;
        } else {
            self.ip += params.len() + 1;
        }

        Ok(())
    }

    fn jump_if_false(&mut self, params: &[RegisterValue], modes: &[ValueMode]) -> Result<()> {
        let (value, address) = self.jump_operands(params, modes)?;
        if value == 0 {
            log::trace!("Jumping to {:?}", address);
            self.ip = address;
        } else {
            self.ip += params.len() + 1;
        }

        Ok(())
    }

    fn adjust_rel_offset(&mut self, params: &[RegisterValue], modes: &[ValueMode]) -> Result<()> {
        let value = self.get_value(params[0], modes[0])?;
        log::trace!("Adjusting relative base by {:?}", value);
        self.relative_base =
            Address::try_from(RegisterValue::try_from(self.relative_base)? + value)?;
        self.ip += params.len() + 1;

        Ok(())
    }

    fn halt(&mut self, _params: &[RegisterValue], _modes: &[ValueMode]) -> Result<()> {
        log::trace!("Halting");

        Ok(())
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

struct OutputIterator<'r, 'v> {
    vm: &'r mut VM<'v>,
}

impl<'r, 'v> Iterator for OutputIterator<'r, 'v> {
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
        self.memory.get(index).unwrap_or(&0)
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

pub struct FakeVM<'a> {
    outputs: std::vec::IntoIter<RegisterValue>,
    inputs: Option<Box<dyn Iterator<Item = RegisterValue> + 'a>>,
}

impl<'a> FakeVM<'a> {
    pub fn new(
        _program: Vec<RegisterValue>,
        _inputs: Option<Box<dyn Iterator<Item = RegisterValue> + 'a>>,
    ) -> FakeVM {
        FakeVM {
            outputs: vec![].into_iter(),
            inputs: None,
        }
    }

    // pub fn step(&mut self) -> Result<(OpType, Vec<RegisterValue>)> { }

    // pub fn diagnostic_code(&self) -> RegisterValue { }

    pub fn outputs<'b>(&'b mut self) -> impl Iterator<Item = RegisterValue> + Captures<'a> + 'b {
        self.outputs.by_ref()
    }

    pub fn set_inputs(&mut self, inputs: Option<Box<dyn Iterator<Item = RegisterValue> + 'a>>) {
        self.inputs = inputs;
    }

    pub fn set_memory(&mut self, _index: Address, _value: RegisterValue) {}

    // pub fn memory(&self) -> &[RegisterValue] { }

    // pub fn last_output(&self) -> Option<RegisterValue>

    pub fn set_outputs(&mut self, outputs: Vec<RegisterValue>) {
        self.outputs = outputs.into_iter();
    }
}
