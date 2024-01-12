#![feature(return_position_impl_trait_in_trait)]

use anyhow::{anyhow, Result};
use rustc_hash::FxHashMap;

use aoc::graphs::{Graph, GraphExt};
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let graph: InstructionGraph = parsing::parse_graph(input.trim())?;

    println!("Part 1: {}", part1(&graph)?);
    println!("Part 2: {}", part2(&graph)?);

    Ok(())
}

fn part1(graph: &InstructionGraph) -> Result<u16> {
    graph
        .get_value("a", FxHashMap::default())
        .ok_or(anyhow!("Failed to evaluate wire"))
}

fn part2(graph: &InstructionGraph) -> Result<u16> {
    let initial = graph
        .get_value("a", FxHashMap::default())
        .ok_or(anyhow!("Failed to evaluate initial wire"))?;
    graph
        .get_value("a", [("b", initial)].into_iter().collect())
        .ok_or(anyhow!("Failed to evaluate final wire"))
}

#[derive(Debug, Clone)]
pub struct InstructionGraph<'a> {
    ops: FxHashMap<&'a str, Op<'a>>,
}

impl<'a> InstructionGraph<'a> {
    fn get_value(&'a self, wire: &'a str, mut values: FxHashMap<&'a str, u16>) -> Option<u16> {
        self.evaluate_instructions(&mut values)?;
        values.get(wire).copied()
    }

    fn evaluate_instructions<'b>(&'a self, values: &'b mut FxHashMap<&'a str, u16>) -> Option<()> {
        for &wire in self.toposort()?.iter().rev() {
            if !values.contains_key(wire) {
                values.insert(wire, self.evaluate(wire, &values).unwrap());
            }
        }
        Some(())
    }

    fn evaluate(&self, wire: &str, values: &FxHashMap<&str, u16>) -> Option<u16> {
        let lookup = |value: &Value| -> Option<u16> {
            match value {
                Value::Int(n) => Some(*n),
                Value::Wire(w) => values.get(w).copied(),
            }
        };
        match self.ops.get(wire)? {
            Op::Value(value) => lookup(value),
            Op::And(a, b) => Some(lookup(a)? & lookup(b)?),
            Op::LShift(a, b) => Some(lookup(a)? << lookup(b)?),
            Op::Not(value) => Some(!lookup(value)?),
            Op::Or(a, b) => Some(lookup(a)? | lookup(b)?),
            Op::RShift(a, b) => Some(lookup(a)? >> lookup(b)?),
        }
    }
}

impl<'a> Graph<'a> for InstructionGraph<'a> {
    type Item = &'a str;

    fn nodes(&'a self) -> impl Iterator<Item = <InstructionGraph<'a> as Graph<'a>>::Item> + 'a {
        self.ops.keys().copied()
    }

    fn neighbors(
        &'a self,
        value: &Self::Item,
    ) -> impl Iterator<Item = <InstructionGraph<'a> as Graph<'a>>::Item> + 'a {
        let values = match self.ops.get(value) {
            Some(Op::Value(value)) => vec![value],
            Some(Op::And(a, b)) => vec![a, b],
            Some(Op::LShift(a, b)) => vec![a, b],
            Some(Op::Not(value)) => vec![value],
            Some(Op::Or(a, b)) => vec![a, b],
            Some(Op::RShift(a, b)) => vec![a, b],
            None => vec![],
        };
        values.into_iter().filter_map(|v| match v {
            Value::Wire(w) => Some(*w),
            _ => None,
        })
    }
}

#[derive(Debug, Clone)]
enum Op<'a> {
    Value(Value<'a>),
    And(Value<'a>, Value<'a>),
    LShift(Value<'a>, Value<'a>),
    Not(Value<'a>),
    Or(Value<'a>, Value<'a>),
    RShift(Value<'a>, Value<'a>),
}

#[derive(Debug, Clone)]
enum Value<'a> {
    Int(u16),
    Wire(&'a str),
}

mod parsing {
    use super::{InstructionGraph, Op, Value};

    use anyhow::{anyhow, Result};
    use rustc_hash::FxHashMap;

    pub fn parse_graph<'a>(s: &'a str) -> Result<InstructionGraph<'a>> {
        let mut ops = FxHashMap::default();
        for line in s.lines() {
            let (op_part, wire) = line
                .split_once(" -> ")
                .ok_or_else(|| anyhow!("Invalid instruction {}.", line))?;
            let op = parse_op(op_part)?;
            ops.insert(wire, op);
        }
        Ok(InstructionGraph { ops })
    }

    fn parse_op<'a>(s: &'a str) -> Result<Op<'a>> {
        let parts: Vec<_> = s.split(' ').collect();
        match parts.as_slice() {
            &[value] => Ok(Op::Value(parse_value(value)?)),
            &[op_type, value] if op_type == "NOT" => Ok(Op::Not(parse_value(value)?)),
            &[a, op_type, b] if op_type == "AND" => Ok(Op::And(parse_value(a)?, parse_value(b)?)),
            &[a, op_type, b] if op_type == "OR" => Ok(Op::Or(parse_value(a)?, parse_value(b)?)),
            &[a, op_type, b] if op_type == "LSHIFT" => {
                Ok(Op::LShift(parse_value(a)?, parse_value(b)?))
            }
            &[a, op_type, b] if op_type == "RSHIFT" => {
                Ok(Op::RShift(parse_value(a)?, parse_value(b)?))
            }
            _ => Err(anyhow!("Invalid operation {}.", s)),
        }
    }

    fn parse_value<'a>(s: &'a str) -> Result<Value<'a>> {
        match s.parse::<u16>() {
            Ok(n) => Ok(Value::Int(n)),
            Err(_) => Ok(Value::Wire(s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parsing, InstructionGraph};

    use rustc_hash::FxHashMap;

    #[test]
    fn evaluate_instructions() {
        let graph: InstructionGraph = parsing::parse_graph(
            "\
            123 -> x\n\
            456 -> y\n\
            x AND y -> d\n\
            x OR y -> e\n\
            x LSHIFT 2 -> f\n\
            y RSHIFT 2 -> g\n\
            NOT x -> h\n\
            NOT y -> i",
        )
        .unwrap();

        let mut values = FxHashMap::default();
        graph.evaluate_instructions(&mut values);
        let expected = [
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];

        assert_eq!(values, expected.into_iter().collect());
    }
}
