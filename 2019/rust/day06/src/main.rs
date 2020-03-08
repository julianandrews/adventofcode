extern crate aoc;

use aoc::graphs::{bfs, Graph};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

struct OrbitGraph {
    orbits: HashMap<String, HashSet<String>>,
}

impl Graph<String> for OrbitGraph {
    fn nodes<'a>(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        Box::new(self.orbits.keys())
    }

    fn neighbors<'a>(&'a self, value: &String) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        match self.orbits.get(value) {
            Some(neighbors) => Box::new(neighbors.iter()),
            None => Box::new(std::iter::empty()),
        }
    }
}

fn parse_orbits(data: String) -> OrbitGraph {
    let mut orbits = HashMap::new();
    for line in data.lines() {
        let mut split = line.trim().splitn(2, ')');
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        orbits
            .entry(a.to_string())
            .or_insert_with(HashSet::new)
            .insert(b.to_string());
        orbits
            .entry(b.to_string())
            .or_insert_with(HashSet::new)
            .insert(a.to_string());
    }

    OrbitGraph { orbits: orbits }
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
    Ok(0)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let orbit_graph = parse_orbits(input);

    println!("Part 1: {}", part1(&orbit_graph)?);
    println!("Part 2: {}", part2(&orbit_graph)?);
    Ok(())
}
