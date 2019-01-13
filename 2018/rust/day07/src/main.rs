extern crate aoc;

use aoc::graphs::{toposort, Graph};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(PartialEq, Eq, Hash, Clone)]
struct ReverseChar {
    data: char,
}

impl PartialOrd for ReverseChar {
    fn partial_cmp(&self, other: &ReverseChar) -> Option<std::cmp::Ordering> {
        self.data
            .partial_cmp(&other.data)
            .map(std::cmp::Ordering::reverse)
    }
}

impl Ord for ReverseChar {
    fn cmp(&self, other: &ReverseChar) -> std::cmp::Ordering {
        self.data.cmp(&other.data).reverse()
    }
}

struct DependencyGraph<T> {
    values: HashSet<T>,
    edges: HashMap<T, HashSet<T>>,
}

impl DependencyGraph<ReverseChar> {
    fn new(pairs: Vec<[char; 2]>) -> DependencyGraph<ReverseChar> {
        let mut values = HashSet::new();
        let mut edges = HashMap::new();
        for value in pairs.iter().flatten() {
            values.insert(ReverseChar { data: *value });
            edges.insert(ReverseChar { data: *value }, HashSet::new());
        }
        for [a, b] in pairs {
            edges
                .get_mut(&ReverseChar { data: a })
                .unwrap()
                .insert(ReverseChar { data: b });
        }

        DependencyGraph {
            values: values,
            edges: edges,
        }
    }
}

impl<T: Clone + Eq + Hash> Graph<T> for DependencyGraph<T> {
    fn values(&self) -> Vec<T> {
        self.values.iter().cloned().collect()
    }

    fn neighbors(&self, value: &T) -> Vec<T> {
        self.edges[value].iter().cloned().collect()
    }
}

fn part1(pairs: Vec<[char; 2]>) -> Result<()> {
    let graph = DependencyGraph::new(pairs);
    let result: String = toposort(graph).unwrap().iter().map(|x| x.data).collect();

    writeln!(io::stdout(), "{}", result)?;

    Ok(())
}

fn part2(_pairs: Vec<[char; 2]>) -> Result<()> {
    writeln!(io::stdout(), "{}", 0)?;

    Ok(())
}

fn parse_line(line: &str) -> [char; 2] {
    let line: Vec<char> = line.chars().collect();

    [line[5], line[36]]
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let pairs: Vec<[char; 2]> = input.lines().map(parse_line).collect();

    part1(pairs.clone())?;
    part2(pairs.clone())?;
    Ok(())
}
