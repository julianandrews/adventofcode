use anyhow::{anyhow, bail, Result};
use rustc_hash::FxHashMap;
use std::{convert::TryFrom, str::FromStr};

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let (instructions, network) = parse_input(input.trim())?;

    println!("Part 1: {}", part1(&network, &instructions)?);
    println!("Part 2: {}", part2(&network, &instructions)?);

    Ok(())
}

fn part1(network: &Network, instructions: &[Instruction]) -> Result<u64> {
    network
        .steps(instructions, "AAA", "ZZZ")
        .ok_or(anyhow!("Failed to find path"))
}

fn part2(network: &Network, instructions: &[Instruction]) -> Result<u64> {
    network.ghost_steps(instructions)
}

fn parse_input(s: &str) -> Result<(Vec<Instruction>, Network)> {
    let (instructions, network) = s.split_once("\n\n").ok_or(anyhow!("Invalid input"))?;
    let instructions = instructions
        .chars()
        .map(|c| c.try_into())
        .collect::<Result<_>>()?;
    let network = network.parse()?;
    Ok((instructions, network))
}

struct Network {
    nodes: FxHashMap<String, (String, String)>,
}

impl Network {
    fn steps(&self, instructions: &[Instruction], start: &str, end: &str) -> Option<u64> {
        let mut node = start;
        for (count, instruction) in instructions.iter().cycle().enumerate() {
            node = match instruction {
                Instruction::Left => &self.nodes.get(node)?.0,
                Instruction::Right => &self.nodes.get(node)?.1,
            };
            if node == end {
                return Some(count as u64 + 1);
            }
        }
        unreachable!();
    }

    fn ghost_steps(&self, instructions: &[Instruction]) -> Result<u64> {
        let starting_nodes = self
            .nodes
            .keys()
            .filter(|n| matches!(n.as_bytes(), [_, _, b'A']));
        let cycles: Vec<Cycle> = starting_nodes
            .map(|node| self.find_cycle(instructions, node))
            .collect::<Result<_>>()?;
        if cycles.iter().any(|cycle| cycle.length != cycle.start) {
            bail!("Cycle start and cycle length different. Smarter solution required.");
        }

        Ok(cycles
            .into_iter()
            .fold(1, |lcm, cycle| num_integer::lcm(lcm, cycle.length)) as u64)
    }

    fn find_cycle(&self, instructions: &[Instruction], starting_node: &str) -> Result<Cycle> {
        let mut seen = FxHashMap::default();
        seen.insert(starting_node, 0);
        let mut endpoint = None;
        for (count, node) in self.iter(instructions, starting_node).enumerate() {
            if matches!(node.as_bytes(), [_, _, b'Z']) {
                if let Some(end) = endpoint {
                    if end != node {
                        bail!("Multiple end points discovered. Smarter solution required.");
                    }
                } else {
                    endpoint = Some(node);
                }
                if let Some(&start) = seen.get(node) {
                    return Ok(Cycle {
                        start,
                        length: count as u64 + 1 - start,
                    });
                }
            }
            seen.insert(node, count as u64 + 1);
        }
        unreachable!()
    }

    fn iter<'a>(
        &'a self,
        instructions: &'a [Instruction],
        mut node: &'a str,
    ) -> impl Iterator<Item = &str> + 'a {
        instructions
            .iter()
            .cycle()
            .map(move |instruction| {
                node = match instruction {
                    Instruction::Left => &self.nodes.get(node)?.0,
                    Instruction::Right => &self.nodes.get(node)?.1,
                };
                Some(node)
            })
            .flatten()
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Cycle {
    start: u64,
    length: u64,
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = FxHashMap::default();
        for line in s.lines() {
            let (from, to) = line
                .split_once(" = ")
                .ok_or_else(|| anyhow!("Invalid node: {}", s))?;
            let (left, right) = to
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.split_once(", "))
                .ok_or_else(|| anyhow!("Invalid node targets: {}", s))?;
            nodes.insert(from.to_string(), (left.to_string(), right.to_string()));
        }

        Ok(Network { nodes })
    }
}

impl TryFrom<char> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(anyhow!("Unrecognized instruction: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_1: &str = "\
        RL\n\
        \n\
        AAA = (BBB, CCC)\n\
        BBB = (DDD, EEE)\n\
        CCC = (ZZZ, GGG)\n\
        DDD = (DDD, DDD)\n\
        EEE = (EEE, EEE)\n\
        GGG = (GGG, GGG)\n\
        ZZZ = (ZZZ, ZZZ)";

    static TEST_DATA_2: &str = "\
        LLR\n\
        \n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)";

    static TEST_DATA_3: &str = "\
        LR\n\
        \n\
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)";

    #[test]
    fn steps_1() {
        let (instructions, network) = parse_input(TEST_DATA_1).unwrap();
        assert_eq!(network.steps(&instructions, "AAA", "ZZZ"), Some(2));
    }

    #[test]
    fn steps_2() {
        let (instructions, network) = parse_input(TEST_DATA_2).unwrap();
        assert_eq!(network.steps(&instructions, "AAA", "ZZZ"), Some(6));
    }

    #[test]
    fn ghost_steps() {
        let (instructions, network) = parse_input(TEST_DATA_3).unwrap();
        assert_eq!(network.ghost_steps(&instructions).unwrap(), 6);
    }
}
