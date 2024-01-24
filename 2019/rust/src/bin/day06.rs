use aoc::aoc_error::AOCError;
use aoc::graphs::{bfs, Graph};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct OrbitGraph {
    orbits: HashMap<String, HashSet<String>>,
}

impl<'a> Graph<'a> for OrbitGraph {
    type Item = &'a String;

    fn nodes(&'a self) -> impl Iterator<Item = Self::Item> + 'a {
        self.orbits.keys()
    }

    fn neighbors(&'a self, value: &Self::Item) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
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

        Ok(OrbitGraph { orbits })
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
    let input = aoc::utils::get_input()?;
    let orbit_graph = input.parse()?;

    println!("Part 1: {}", part1(&orbit_graph)?);
    println!("Part 2: {}", part2(&orbit_graph)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let orbit_graph = "COM)B
                           B)C
                           C)D
                           D)E
                           E)F
                           B)G
                           G)H
                           D)I
                           E)J
                           J)K
                           K)L"
        .parse();
        assert!(orbit_graph.is_ok());
        let result = part1(&orbit_graph.unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_part2() {
        let orbit_graph = "COM)B
                           B)C
                           C)D
                           D)E
                           E)F
                           B)G
                           G)H
                           D)I
                           E)J
                           J)K
                           K)L
                           K)YOU
                           I)SAN"
            .parse();
        assert!(orbit_graph.is_ok());
        let result = part2(&orbit_graph.unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }
}
