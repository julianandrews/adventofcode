extern crate aoc;

use aoc::aoc_error::AOCError;
use aoc::graphs::{bfs, Graph};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct OrbitGraph {
    orbits: HashMap<String, HashSet<String>>,
}

impl<'a> Graph<'a, &'a String> for OrbitGraph {
    fn nodes(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        Box::new(self.orbits.keys())
    }

    fn neighbors(&'a self, value: &&String) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        match self.orbits.get(*value) {
            Some(neighbors) => Box::new(neighbors.iter()),
            None => Box::new(std::iter::empty()),
        }
    }
}

impl FromStr for OrbitGraph {
    type Err = AOCError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let mut orbits = HashMap::new();
        for line in s.lines() {
            let mut split = line.trim().splitn(2, ')');
            let a = match split.next() {
                Some(value) => value,
                None => return Err(AOCError::new(&format!("Failed to parse line: {}", line))),
            };
            let b = match split.next() {
                Some(value) => value,
                None => return Err(AOCError::new(&format!("Failed to parse line: {}", line))),
            };
            orbits
                .entry(a.to_string())
                .or_insert_with(HashSet::new)
                .insert(b.to_string());
            orbits
                .entry(b.to_string())
                .or_insert_with(HashSet::new)
                .insert(a.to_string());
        }

        Ok(OrbitGraph { orbits: orbits })
    }
}

fn part1(orbit_graph: &OrbitGraph) -> Result<u64> {
    Ok(bfs(orbit_graph, &"COM".to_string())
        .map(|node| node.depth)
        .sum())
}

fn part2(orbit_graph: &OrbitGraph) -> Result<u64> {
    for node in bfs(orbit_graph, &"YOU".to_string()) {
        if node.value == "SAN" {
            return Ok(node.depth - 2);
        }
    }
    Err(AOCError::new("Failed to find Santa!"))?
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let orbit_graph = input.parse()?;

    println!("Part 1: {}", part1(&orbit_graph)?);
    println!("Part 2: {}", part2(&orbit_graph)?);
    Ok(())
}
