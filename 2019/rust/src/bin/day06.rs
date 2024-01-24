use anyhow::{anyhow, Result};
use rustc_hash::{FxHashMap, FxHashSet};

use aoc::graphs::{bfs, Graph};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let orbit_graph = input.parse()?;

    println!("Part 1: {}", part1(&orbit_graph)?);
    println!("Part 2: {}", part2(&orbit_graph)?);
    Ok(())
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
    Err(anyhow!("Failed to find Santa!"))
}

#[derive(Debug)]
struct OrbitGraph {
    orbits: FxHashMap<String, FxHashSet<String>>,
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

mod parsing {
    use super::OrbitGraph;

    use anyhow::bail;

    use rustc_hash::{FxHashMap, FxHashSet};

    impl std::str::FromStr for OrbitGraph {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            let mut orbits: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
            for line in s.lines() {
                let mut split = line.trim().splitn(2, ')');
                let a = match split.next() {
                    Some(value) => value,
                    None => bail!("Failed to parse line: {}", line),
                };
                let b = match split.next() {
                    Some(value) => value,
                    None => bail!("Failed to parse line: {}", line),
                };
                orbits
                    .entry(a.to_string())
                    .or_default()
                    .insert(b.to_string());
                orbits
                    .entry(b.to_string())
                    .or_default()
                    .insert(a.to_string());
            }

            Ok(OrbitGraph { orbits })
        }
    }
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
