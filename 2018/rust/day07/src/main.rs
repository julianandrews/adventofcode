extern crate aoc;

use std::collections::HashMap;
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

fn part1(pairs: Vec<(char, char)>) -> Result<()> {
    let mut edges = HashMap::new();
    for (a, b) in pairs {
        (*edges.entry(ReverseChar { data: a }).or_insert(Vec::new())).push(ReverseChar { data: b });
        edges.entry(ReverseChar { data: b }).or_insert(Vec::new());
    }
    let values: Vec<ReverseChar> = edges.keys().cloned().collect();
    let result: String = aoc::graphs::toposort::toposort(values, &move |v| edges[v].clone())
        .unwrap()
        .iter()
        .map(|x| x.data)
        .collect();

    writeln!(io::stdout(), "{}", result)?;

    Ok(())
}

fn part2(_pairs: Vec<(char, char)>) -> Result<()> {
    writeln!(io::stdout(), "{}", 0)?;

    Ok(())
}

fn parse_line(line: &str) -> (char, char) {
    let line: Vec<char> = line.chars().collect();

    (line[5], line[36])
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let pairs: Vec<(char, char)> = input.lines().map(parse_line).collect();

    part1(pairs.clone())?;
    part2(pairs.clone())?;
    Ok(())
}
