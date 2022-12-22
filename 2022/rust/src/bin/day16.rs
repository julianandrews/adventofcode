use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};

use aoc::utils::get_input;

type Pressure = u32;
type Time = u32;
type Distance = u32;
type Cache = HashMap<Node, (Pressure, Valves)>;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: ValveMap = input.trim().parse()?;

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn part1(map: &ValveMap) -> Pressure {
    let mut cache: Cache = HashMap::new();
    let starting_node = Node {
        location: map.starting_index,
        time: 30,
        valves: Valves(0),
    };
    max_pressure(map, starting_node, &mut cache).0
}

fn part2(map: &ValveMap) -> Pressure {
    let mut cache: Cache = HashMap::new();
    for valves in 0..1 << map.flow_rates.len() {
        let starting_node = Node {
            location: map.starting_index,
            time: 26,
            valves: Valves(valves),
        };
        max_pressure(map, starting_node, &mut cache);
    }

    let mut pressures: Vec<_> = cache
        .iter()
        .filter(|(n, _)| n.time == 26)
        .map(|(_, v)| v)
        .collect();
    pressures.sort_unstable_by_key(|(p, _)| std::cmp::Reverse(*p));

    let mut best = 0;
    for (i, (my_pressure, my_valves)) in pressures.iter().enumerate() {
        for (elephant_pressure, elephant_valves) in &pressures[i + 1..] {
            if my_valves.overlaps(elephant_valves) {
                continue;
            }
            let new_pressure = my_pressure + elephant_pressure;
            if new_pressure > best {
                best = new_pressure;
            } else {
                break;
            }
        }
    }

    best
}

fn max_pressure(map: &ValveMap, node: Node, cache: &mut Cache) -> (Pressure, Valves) {
    if let Some(&(pressure, valves)) = cache.get(&node) {
        return (pressure, valves);
    }
    let mut pressure = 0;
    let mut valves = Valves(0);
    for neighbor in map.neighbors(&node) {
        let (new_pressure, new_valves) = max_pressure(map, neighbor, cache);
        if new_pressure > pressure {
            pressure = new_pressure;
            valves = new_valves.insert(neighbor.location);
        }
    }
    pressure += node.time * map.flow_rates[node.location];
    cache.insert(node, (pressure, valves));
    (pressure, valves)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    location: usize,
    time: Time,
    valves: Valves,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Valves(u64);

impl Valves {
    fn contains(&self, i: usize) -> bool {
        self.0 & (1 << i) != 0
    }

    fn insert(&self, i: usize) -> Self {
        Self(self.0 | (1 << i))
    }

    fn overlaps(&self, other: &Valves) -> bool {
        self.0 & other.0 != 0
    }
}

#[derive(Debug, Clone)]
struct ValveMap {
    starting_index: usize,
    flow_rates: Vec<Pressure>,
    distances: Vec<Vec<Distance>>,
}

impl ValveMap {
    fn neighbors<'a: 'c, 'b: 'c, 'c>(&'a self, node: &'b Node) -> impl Iterator<Item = Node> + 'c {
        (0..self.flow_rates.len())
            .filter(|&i| !node.valves.contains(i))
            .filter_map(|i| {
                let distance = self.distances[node.location][i];
                if distance + 1 > node.time {
                    return None;
                }
                Some(Node {
                    location: i,
                    time: node.time - distance - 1,
                    valves: node.valves.insert(i),
                })
            })
    }
}

impl std::str::FromStr for ValveMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rooms: Vec<Room> = s.lines().map(str::parse).collect::<Result<_>>()?;
        let room_indices: HashMap<&str, usize> = rooms
            .iter()
            .enumerate()
            .map(|(i, r)| (r.name.as_str(), i))
            .collect();

        // Floyd-Warshall algorithm
        let v = rooms.len();
        let mut distances = vec![vec![v as Distance; v]; v];
        let edges = rooms.iter().flat_map(|room| {
            room.tunnels
                .iter()
                .map(|t| (room.name.as_str(), t.as_str()))
        });
        for (from, to) in edges {
            distances[room_indices[from]][room_indices[to]] = 1;
        }
        // Clippy's suggested fix is just god-awful ugly here.
        #[allow(clippy::needless_range_loop)]
        for i in 0..rooms.len() {
            distances[i][i] = 0;
        }
        for k in 0..v {
            for i in 0..v {
                for j in 0..v {
                    if distances[i][j] > distances[i][k] + distances[k][j] {
                        distances[i][j] = distances[i][k] + distances[k][j];
                    }
                }
            }
        }

        let starting_index = rooms
            .iter()
            .filter(|r| r.name == "AA" || r.flow_rate > 0)
            .position(|r| r.name == "AA")
            .ok_or_else(|| anyhow!("Failed to find starting room"))?;
        let flow_rates: Vec<Pressure> = rooms
            .iter()
            .filter(|r| r.name == "AA" || r.flow_rate > 0)
            .map(|r| r.flow_rate)
            .collect();
        let distances: Vec<Vec<Distance>> = rooms
            .iter()
            .zip(distances.into_iter())
            .filter(|(r, _)| r.name == "AA" || r.flow_rate > 0)
            .map(|(_, v)| {
                rooms
                    .iter()
                    .zip(v.into_iter())
                    .filter(|(r, _)| r.name == "AA" || r.flow_rate > 0)
                    .map(|(_, v)| v)
                    .collect()
            })
            .collect();
        if flow_rates.len() > 64 {
            bail!("Too many functioning valves! Only 64 rooms supported");
        }

        Ok(Self {
            starting_index,
            flow_rates,
            distances,
        })
    }
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    flow_rate: Pressure,
    tunnels: Vec<String>,
}

impl std::str::FromStr for Room {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (valve_part, tunnels_part) = s
            .split_once("; ")
            .ok_or_else(|| anyhow!("Invalid valve {}", s))?;
        let (name_part, rate_part) = valve_part
            .split_once(" has flow rate=")
            .ok_or_else(|| anyhow!("Invalid valve part {}", valve_part))?;

        let name = name_part
            .strip_prefix("Valve ")
            .ok_or_else(|| anyhow!("Invalid name part {}", name_part))?
            .to_string();
        let flow_rate: Pressure = rate_part.parse()?;
        let tunnels: Vec<_> = tunnels_part
            .strip_prefix("tunnels lead to valves ")
            .or_else(|| tunnels_part.strip_prefix("tunnel leads to valve "))
            .ok_or_else(|| anyhow!("Invalid tunnel part {}", tunnels_part))?
            .split(", ")
            .map(str::to_owned)
            .collect();
        Ok(Self {
            name,
            flow_rate,
            tunnels,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
        Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
        Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
        Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
        Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
        Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
        Valve HH has flow rate=22; tunnel leads to valve GG\n\
        Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
        Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn max_pressure_release() {
        let map: ValveMap = TEST_DATA.parse().unwrap();
        assert_eq!(part1(&map), 1651);
    }

    #[test]
    fn elephant_help() {
        let map: ValveMap = TEST_DATA.parse().unwrap();
        assert_eq!(part2(&map), 1707);
    }
}
