use std::collections::VecDeque;

use anyhow::{anyhow, Result};
use rustc_hash::FxHashMap;

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let network = parsing::parse_network(input.trim())?;

    println!("Part 1: {}", part1(network.clone()));
    println!("Part 2: {}", part2(network)?);

    Ok(())
}

fn part1(mut network: Network) -> u64 {
    let counts = network.count_pulses(1000);
    counts.low * counts.high
}

fn part2(mut network: Network) -> Result<u64> {
    network
        .find_period("rx")
        .ok_or(anyhow!("Couldn't find simple period for 'rx'"))
}

#[derive(Debug, Clone)]
pub struct Network {
    modules: Vec<Module>,
    label_map: FxHashMap<String, usize>,
}

impl Network {
    /// Press the button `presses` times and return the total pulse counts.
    fn count_pulses(&mut self, presses: u64) -> PulseCounts {
        let mut counts = PulseCounts::default();
        for _ in 0..presses {
            for (_, _, pulse) in self.press_button() {
                counts += PulseCounts::from(pulse);
            }
        }
        counts
    }

    /// Press the button and return an iterator over the pulses seen.
    fn press_button(&mut self) -> PulseIterator {
        let mut pulses = VecDeque::new();
        let broadcast_index = *self.label_map.get("broadcaster").unwrap();
        pulses.push_back((broadcast_index, broadcast_index, Pulse::Low));

        PulseIterator {
            network: self,
            pulses,
        }
    }

    /// Find the period of a given module.
    ///
    /// This code assumes that each input to the module repeats with a regular period starting from
    /// the first button press.
    fn find_period(&mut self, label: &str) -> Option<u64> {
        let inputs = {
            let mut index = *self.label_map.get(label)?;
            let mut module = self.modules.get(index)?;
            while module.inputs.len() == 1 {
                index = module.inputs[0];
                module = self.modules.get(index)?;
            }
            module.inputs.clone()
        };
        let periods = self.find_first_triggers(&inputs);

        Some(
            periods
                .into_iter()
                .fold(1, |lcm, period| num_integer::lcm(lcm, period)),
        )
    }

    /// Find the first time each index triggers a high pulse.
    fn find_first_triggers(&mut self, indices: &[usize]) -> Vec<u64> {
        let mut seen: FxHashMap<_, _> = indices.iter().map(|i| (i, 0)).collect();
        let mut count = 0;

        for t in 1.. {
            for (source, _, pulse) in self.press_button() {
                if pulse == Pulse::High {
                    if let Some(first_seen) = seen.get_mut(&source) {
                        if *first_seen == 0 {
                            *first_seen = t;
                            count += 1;
                            if count == indices.len() {
                                return indices.iter().map(|i| *seen.get(i).unwrap()).collect();
                            }
                        }
                    }
                }
            }
        }
        unreachable!();
    }
}

#[derive(Debug)]
struct PulseIterator<'a> {
    network: &'a mut Network,
    pulses: VecDeque<(usize, usize, Pulse)>,
}

impl<'a> Iterator for PulseIterator<'a> {
    type Item = (usize, usize, Pulse);

    fn next(&mut self) -> Option<Self::Item> {
        let (source, index, pulse) = self.pulses.pop_front()?;
        if let Some(module) = self.network.modules.get_mut(index) {
            if let Some(module_output) = module.state.receive(pulse, source) {
                for destination in &module.destinations {
                    self.pulses.push_back((index, *destination, module_output));
                }
            }
        }
        Some((source, index, pulse))
    }
}

#[derive(Debug, Clone)]
struct Module {
    label: String,
    state: ModuleState,
    destinations: Vec<usize>,
    inputs: Vec<usize>,
}

#[derive(Debug, Clone)]
enum ModuleState {
    Broadcast,
    FlipFlop(bool),
    Conjunction(u64, u32),
    Untyped,
}

impl ModuleState {
    fn receive(&mut self, pulse: Pulse, source: usize) -> Option<Pulse> {
        match self {
            ModuleState::Broadcast => Some(pulse),
            ModuleState::Untyped => None,
            ModuleState::FlipFlop(on) => match pulse {
                Pulse::Low => {
                    *on = !*on;
                    if *on {
                        Some(Pulse::High)
                    } else {
                        Some(Pulse::Low)
                    }
                }
                Pulse::High => None,
            },
            ModuleState::Conjunction(state, input_count) => {
                match pulse {
                    Pulse::Low => *state &= !(1 << source),
                    Pulse::High => *state |= 1 << source,
                }
                let pulse = if state.count_ones() == *input_count {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                Some(pulse)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct PulseCounts {
    high: u64,
    low: u64,
}

impl From<Pulse> for PulseCounts {
    fn from(value: Pulse) -> Self {
        match value {
            Pulse::Low => PulseCounts { high: 0, low: 1 },
            Pulse::High => PulseCounts { high: 1, low: 0 },
        }
    }
}

impl std::ops::AddAssign for PulseCounts {
    fn add_assign(&mut self, rhs: Self) {
        self.high += rhs.high;
        self.low += rhs.low;
    }
}

mod parsing {
    use super::{Module, ModuleState, Network};
    use rustc_hash::FxHashMap;

    use anyhow::{anyhow, bail, Result};

    pub fn parse_network<'a>(s: &'a str) -> Result<Network> {
        let mut modules = vec![];
        let mut destination_list = vec![];
        let mut label_map = FxHashMap::default();
        let mut broadcast_index = None;
        for (index, line) in s.lines().enumerate() {
            let (label, labels, state) = parse_module(line)?;
            if label == "broadcaster" {
                if let Some(j) = broadcast_index.replace(index) {
                    bail!("Broadcasters found at both {} and {}", index, j);
                }
            }
            if label_map.insert(label.to_string(), index).is_some() {
                bail!("Repeated module id {}", label);
            }
            modules.push(Module {
                label: label.to_string(),
                state,
                destinations: vec![],
                inputs: vec![],
            });
            destination_list.push(labels);
        }

        // Add any untyped modules
        for label in destination_list.iter().flatten() {
            let label = label.to_string();
            if !label_map.contains_key(&label) {
                label_map.insert(label.clone(), modules.len());
                modules.push(Module {
                    label,
                    state: ModuleState::Untyped,
                    destinations: vec![],
                    inputs: vec![],
                });
            }
        }
        for _ in destination_list.len()..modules.len() {
            destination_list.push(vec![]);
        }

        let mut inputs = vec![vec![]; modules.len()];
        for (module, labels) in modules.iter_mut().zip(destination_list) {
            for label in labels {
                let index = *label_map.get(&label.to_string()).unwrap();
                module.destinations.push(index);
                let module_index = *label_map.get(module.label.as_str()).unwrap();
                inputs[index].push(module_index);
                if label == "output" {
                    println!("Added {} to output", module_index);
                }
            }
        }
        for (module, i) in modules.iter_mut().zip(inputs) {
            if let ModuleState::Conjunction(_, count) = &mut module.state {
                *count = i.len() as u32;
            }
            module.inputs = i;
        }
        Ok(Network { modules, label_map })
    }

    fn parse_module(s: &str) -> Result<(&str, Vec<&str>, ModuleState)> {
        let (start, destinations) = s
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("Invalid module {}", s))?;
        let destinations = destinations.split(", ").collect();
        if start == "broadcaster" {
            Ok(("broadcaster", destinations, ModuleState::Broadcast))
        } else {
            match start.chars().next() {
                Some('%') => Ok((&start[1..], destinations, ModuleState::FlipFlop(false))),
                Some('&') => Ok((&start[1..], destinations, ModuleState::Conjunction(0, 0))),
                _ => bail!("Invalid module {}", s),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_1: &str = "\
        broadcaster -> a, b, c\n\
        %a -> b\n\
        %b -> c\n\
        %c -> inv\n\
        &inv -> a";

    static TEST_DATA_2: &str = "\
        broadcaster -> a\n\
        %a -> inv, con\n\
        &inv -> b\n\
        %b -> con\n\
        &con -> output";

    #[test]
    fn press_button_1() {
        let mut network: Network = parsing::parse_network(TEST_DATA_1).unwrap();
        let result: Vec<_> = network.press_button().collect();
        let expected = vec![
            (0, 0, Pulse::Low),
            (0, 1, Pulse::Low),
            (0, 2, Pulse::Low),
            (0, 3, Pulse::Low),
            (1, 2, Pulse::High),
            (2, 3, Pulse::High),
            (3, 4, Pulse::High),
            (4, 1, Pulse::Low),
            (1, 2, Pulse::Low),
            (2, 3, Pulse::Low),
            (3, 4, Pulse::Low),
            (4, 1, Pulse::High),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn press_button_2() {
        let mut network: Network = parsing::parse_network(TEST_DATA_2).unwrap();
        let result: Vec<_> = network.press_button().collect();
        let expected = vec![
            (0, 0, Pulse::Low),
            (0, 1, Pulse::Low),
            (1, 2, Pulse::High),
            (1, 4, Pulse::High),
            (2, 3, Pulse::Low),
            (4, 5, Pulse::High),
            (3, 4, Pulse::High),
            (4, 5, Pulse::Low),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn count_pulses_1() {
        let mut network: Network = parsing::parse_network(TEST_DATA_1).unwrap();
        let result = network.count_pulses(1000);
        let expected = PulseCounts {
            low: 8000,
            high: 4000,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn count_pulses_2() {
        let mut network: Network = parsing::parse_network(TEST_DATA_2).unwrap();
        let result = network.count_pulses(1000);
        let expected = PulseCounts {
            low: 4250,
            high: 2750,
        };

        assert_eq!(result, expected);
    }
}
