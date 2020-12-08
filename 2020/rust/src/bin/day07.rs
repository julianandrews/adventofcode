#![feature(str_split_once)]

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use aoc::aoc_error::AOCError;
use aoc::graphs::{bfs, Graph};
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let bag_graph: BagGraph = input.trim().parse()?;

    println!("Part 1: {}", part1(&bag_graph));
    println!("Part 2: {}", part2(&bag_graph));
    Ok(())
}

fn part1(bag_graph: &BagGraph) -> usize {
    bag_graph.get_parents("shiny gold").len()
}

fn part2(bag_graph: &BagGraph) -> usize {
    bag_graph.get_bag_count("shiny gold") - 1
}

#[derive(Debug)]
struct BagGraph {
    bags: HashMap<String, HashSet<(String, usize)>>,
}

impl BagGraph {
    fn get_parents<'a>(&'a self, goal_color: &str) -> HashSet<&'a str> {
        self.bags
            .keys()
            .filter(|&color| {
                color != goal_color && bfs(self, color).any(|node| node.value == goal_color)
            })
            .map(|color| color.as_str())
            .collect()
    }

    fn get_bag_count(&self, color: &str) -> usize {
        let mut count = 1;
        for (child, multiple) in self.bags.get(color).unwrap_or(&HashSet::new()) {
            count += multiple * self.get_bag_count(child);
        }
        count
    }
}

impl<'a> Graph<'a> for BagGraph {
    type Item = &'a String;

    fn nodes(&'a self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        Box::new(self.bags.keys())
    }

    fn neighbors(&'a self, value: &Self::Item) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        match self.bags.get(*value) {
            Some(neighbors) => Box::new(neighbors.iter().map(|(color, _num)| color)),
            None => Box::new(std::iter::empty()),
        }
    }
}

impl FromStr for BagGraph {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let bags = s.lines().map(parse_rule).collect::<Result<_>>()?;

        Ok(BagGraph { bags })
    }
}

fn parse_rule(line: &str) -> Result<(String, HashSet<(String, usize)>)> {
    let (color, contents_str) = line
        .split_once(" contain ")
        .ok_or(AOCError::new("Invalid rule"))?;
    let color = color
        .strip_suffix(" bags")
        .ok_or(AOCError::new("Invalid item"))?
        .to_string();
    let contents = contents_str
        .trim_end_matches('.')
        .split(", ")
        .filter(|&item| item != "no other bags")
        .map(|item| {
            let (num_part, color_part) =
                item.split_once(' ').ok_or(AOCError::new("Invalid item"))?;
            let color = color_part
                .trim_end_matches('s')
                .strip_suffix(" bag")
                .ok_or(AOCError::new("Invalid item"))?
                .to_string();
            Ok((color, num_part.parse()?))
        })
        .collect::<Result<_>>()?;

    Ok((color, contents))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\
                                     \ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\
                                     \nbright white bags contain 1 shiny gold bag.\
                                     \nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\
                                     \nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\
                                     \ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\
                                     \nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\
                                     \nfaded blue bags contain no other bags.\
                                     \ndotted black bags contain no other bags.";

    #[test]
    fn get_parents() {
        let bag_graph: BagGraph = TEST_INPUT.parse().unwrap();
        let parents = bag_graph.get_parents("shiny gold");
        let expected: HashSet<_> = vec!["bright white", "muted yellow", "dark orange", "light red"]
            .into_iter()
            .collect();
        assert_eq!(parents, expected);
    }

    #[test]
    fn get_child_count_1() {
        let bag_graph: BagGraph = TEST_INPUT.parse().unwrap();
        assert_eq!(bag_graph.get_bag_count("faded blue"), 1);
        assert_eq!(bag_graph.get_bag_count("dotted black"), 1);
        assert_eq!(bag_graph.get_bag_count("vibrant plum"), 12);
        assert_eq!(bag_graph.get_bag_count("dark olive"), 8);
    }

    #[test]
    fn get_child_count_2() {
        let bag_graph: BagGraph = "shiny gold bags contain 2 dark red bags.\
                                 \ndark red bags contain 2 dark orange bags.\
                                 \ndark orange bags contain 2 dark yellow bags.\
                                 \ndark yellow bags contain 2 dark green bags.\
                                 \ndark green bags contain 2 dark blue bags.\
                                 \ndark blue bags contain 2 dark violet bags.\
                                 \ndark violet bags contain no other bags."
            .parse()
            .unwrap();
        assert_eq!(bag_graph.get_bag_count("shiny gold"), 127);
    }
}
