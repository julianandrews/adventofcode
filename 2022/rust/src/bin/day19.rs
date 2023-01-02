use anyhow::{anyhow, bail, Result};
use rustc_hash::FxHashMap;

use aoc::utils::{get_input, parse_fields};

type Cache = FxHashMap<State, u8>;

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
}

impl Blueprint {
    fn quality_level(&self, time: u8) -> u32 {
        u32::from(self.geodes(time)) * u32::from(self.id)
    }

    fn geodes(&self, time: u8) -> u8 {
        let mut cache: Cache = FxHashMap::default();
        let state = State::new(time);
        self.geodes_helper(&state, &mut cache, &mut 0)
    }

    fn geodes_helper(&self, state: &State, cache: &mut Cache, best: &mut u8) -> u8 {
        if state.time == 0 {
            return state.resources[Resource::Geodes.index()];
        } else if *best > state.best_possible() {
            return 0;
        } else if let Some(&value) = cache.get(state) {
            return value;
        }

        let branch_best = self
            .neighbors(state)
            .map(|n| self.geodes_helper(&n, cache, best))
            .max()
            .unwrap_or(0);

        cache.insert(*state, branch_best);
        *best = branch_best.max(*best);

        branch_best
    }

    fn neighbors<'a: 'iter, 'b: 'iter, 'iter>(
        &'a self,
        state: &'b State,
    ) -> impl Iterator<Item = State> + 'iter {
        let resources = Resource::iter().filter(|&resource| !self.should_prune(state, resource));
        std::iter::once(None)
            .chain(resources.map(Some))
            .filter_map(|resource| self.buy_robot(state, resource))
    }

    /// Returns true if buying a given robot type isn't worth considering
    fn should_prune(&self, state: &State, resource: Resource) -> bool {
        if matches!(resource, Resource::Geodes) {
            return state.time <= 1;
        };
        // Stop buying robots once we have enough resources to buy until the end of time.
        let max_cost = self
            .costs
            .iter()
            .map(|c| c[resource.index()])
            .max()
            .unwrap_or(0);
        let max_robots = max_cost.saturating_sub(state.resources[resource.index()] / max_cost);
        state.robots[resource.index()] >= max_robots
    }

    /// Returns the new state after waiting for and buying the given robot type, or None if we
    /// can never afford it.
    fn buy_robot(&self, state: &State, resource: Option<Resource>) -> Option<State> {
        let time = if let Some(resource) = resource {
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

        let mut state = *state;
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
        let geodes = u32::from(self.resources[3]);
        let max_current_robot_geodes = time * (time + 1) * u32::from(self.robots[3]) / 2;
        let max_new_robot_geodes = time * (time - 1) / 2;
        (geodes + max_current_robot_geodes + max_new_robot_geodes).min(u8::MAX as u32) as u8
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

        Ok(Self {
            id: id_part.parse()?,
            costs,
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
