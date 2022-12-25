use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};

use aoc::utils::{get_input, parse_fields};

type Cache = HashMap<State, u8>;

fn main() -> Result<()> {
    let input = get_input()?;
    let blueprints: Vec<Blueprint> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&blueprints));
    println!("Part 2: {}", part2(&blueprints));

    Ok(())
}

fn part1(blueprints: &[Blueprint]) -> u32 {
    blueprints.iter().map(|b| b.quality_level(24)).sum()
}

fn part2(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|b| u32::from(b.geodes(32)))
        .product()
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: u8,
    costs: [[u8; 4]; 4],
    max_robots: [u8; 4],
}

impl Blueprint {
    fn quality_level(&self, time: u8) -> u32 {
        u32::from(self.geodes(time)) * u32::from(self.id)
    }

    fn geodes(&self, time: u8) -> u8 {
        let mut cache: Cache = HashMap::new();
        let state = State::new(time);
        let mut best = 0;
        self.geodes_helper(&state, &mut cache, &mut best)
    }

    fn geodes_helper(&self, state: &State, cache: &mut Cache, best: &mut u8) -> u8 {
        if state.time == 0 {
            return state.resources[Resource::Geodes.index()];
        } else if let Some(&value) = cache.get(state) {
            return value;
        } else if *best > state.best_possible() {
            return 0;
        }

        let mut new_best = 0;
        for neighbor in self.neighbors(state) {
            new_best = new_best.max(self.geodes_helper(&neighbor, cache, best));
            if new_best > *best {
                *best = new_best;
            }
        }
        cache.insert(state.clone(), new_best);
        new_best
    }

    fn neighbors<'a: 'iter, 'b: 'iter, 'iter>(
        &'a self,
        state: &'b State,
    ) -> impl Iterator<Item = State> + 'iter {
        Resource::iter()
            .map(Some)
            .chain(std::iter::once(None))
            .filter_map(|resource| self.buy_robot(state, resource))
    }

    /// Returns the new state after waiting for and buying the given robot type, or None if we
    /// can never afford it.
    fn buy_robot(&self, state: &State, resource: Option<Resource>) -> Option<State> {
        let time = if let Some(resource) = resource {
            if state.robots[resource.index()] >= self.max_robots[resource.index()] {
                return None;
            }
            let mut time = 0;
            let costs = self.costs[resource.index()];
            for (i, (resource_count, robot_count)) in
                state.resources.iter().zip(state.robots).enumerate()
            {
                let cost = u32::from(costs[i]);
                let resource_count = u32::from(*resource_count);
                let robot_count = u32::from(robot_count);
                let max_resource = resource_count + robot_count * u32::from(state.time - 1);
                if max_resource < cost {
                    return None;
                }
                if robot_count == 0 {
                    continue;
                }
                // Ceiling division
                let resource_time =
                    ((cost + robot_count).saturating_sub(resource_count + 1) / robot_count) as u8;
                time = time.max(resource_time);
            }
            time + 1
        } else {
            state.time
        };

        let mut state = state.clone();
        state.time -= time;
        for (resources, robot_count) in state.resources.iter_mut().zip(state.robots) {
            *resources = resources.saturating_add(time.saturating_mul(robot_count));
        }
        if let Some(resource) = resource {
            for (resources, cost) in state.resources.iter_mut().zip(self.costs[resource.index()]) {
                *resources -= cost;
            }
            state.robots[resource.index()] += 1;
        }
        Some(state)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct State {
    time: u8,
    robots: [u8; 4],
    resources: [u8; 4],
}

impl State {
    fn new(time: u8) -> Self {
        Self {
            time,
            robots: [1, 0, 0, 0],
            resources: [0, 0, 0, 0],
        }
    }

    fn best_possible(&self) -> u8 {
        let time = u32::from(self.time);
        let value = u32::from(self.resources[3])
            + time * (time + 1) * u32::from(self.robots[3]) / 2
            + time * (time - 1) / 2;
        value.min(u8::MAX as u32) as u8
    }
}

#[derive(Debug, Clone, Copy)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geodes,
}

impl Resource {
    fn index(&self) -> usize {
        *self as usize
    }

    fn iter() -> impl Iterator<Item = Self> {
        static RESOURCES: [Resource; 4] = [
            Resource::Ore,
            Resource::Clay,
            Resource::Obsidian,
            Resource::Geodes,
        ];
        RESOURCES.iter().copied()
    }
}

impl std::str::FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn split_parts(s: &str) -> Option<(&str, [&str; 4])> {
            let (id_part, rem) = s.split_once(": ")?;
            let (ore_part, rem) = rem.split_once(". ")?;
            let (clay_part, rem) = rem.split_once(". ")?;
            let (obsidian_part, rem) = rem.split_once(". ")?;
            let geode_part = rem.strip_suffix('.')?;

            let id_part = id_part.strip_prefix("Blueprint ")?;
            let ore_part = ore_part.strip_prefix("Each ore robot costs ")?;
            let clay_part = clay_part.strip_prefix("Each clay robot costs ")?;
            let obsidian_part = obsidian_part.strip_prefix("Each obsidian robot costs ")?;
            let geode_part = geode_part.strip_prefix("Each geode robot costs ")?;

            Some((id_part, [ore_part, clay_part, obsidian_part, geode_part]))
        }

        let (id_part, resource_parts) =
            split_parts(s).ok_or_else(|| anyhow!("Invalid blueprint: {}", s))?;
        let mut costs = [[0; 4]; 4];
        for (i, resource_string) in resource_parts.iter().enumerate() {
            for part in resource_string.split(" and ") {
                let (amount, kind) = part
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("Invalid costs {}", s))?;
                match kind {
                    "ore" => costs[i][0] = amount.parse()?,
                    "clay" => costs[i][1] = amount.parse()?,
                    "obsidian" => costs[i][2] = amount.parse()?,
                    _ => bail!("Invalid costs {}", s),
                }
            }
        }
        let mut max_robots = [0; 4];
        for i in 0..3 {
            max_robots[i] = costs.iter().map(|c| c[i]).max().unwrap_or(0);
        }
        max_robots[3] = u8::MAX;

        Ok(Self {
            id: id_part.parse()?,
            costs,
            max_robots,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\n\
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn quality_levels() {
        let blueprints: Vec<Blueprint> = parse_fields(TEST_DATA, '\n').unwrap();
        let expected = vec![9, 24];
        let quality_levels: Vec<_> = blueprints.iter().map(|b| b.quality_level(24)).collect();
        assert_eq!(quality_levels, expected);
    }

    #[test]
    fn longer_geode_output() {
        let blueprints: Vec<Blueprint> = parse_fields(TEST_DATA, '\n').unwrap();
        let expected = vec![56, 62];
        let counts: Vec<_> = blueprints.iter().map(|b| b.geodes(32)).collect();
        assert_eq!(counts, expected);
    }
}
