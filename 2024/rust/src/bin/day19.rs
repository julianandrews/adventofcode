use anyhow::Result;
use rustc_hash::FxHashMap;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let (mut trie, designs) = parsing::parse_input(&input)?;

    println!("Part 1: {}", part1(&mut trie, &designs));
    println!("Part 2: {}", part2(&mut trie, &designs));

    Ok(())
}

fn part1<'a, 'b: 'a>(trie: &mut Trie<'a>, designs: &'b [Vec<Stripe>]) -> usize {
    designs
        .iter()
        .filter(|design| trie.can_build(design))
        .count()
}

fn part2<'a, 'b: 'a>(trie: &mut Trie<'a>, designs: &'b [Vec<Stripe>]) -> usize {
    designs
        .iter()
        .map(|design| trie.ways_to_build(design))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Stripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug)]
struct Trie<'a> {
    root: TrieNode,
    cache: FxHashMap<&'a [Stripe], usize>,
}

impl<'a> Default for Trie<'a> {
    fn default() -> Self {
        let mut trie = Trie {
            root: TrieNode::default(),
            cache: FxHashMap::default(),
        };
        trie.cache.insert(&[], 1); // Base case
        trie
    }
}

#[derive(Debug, Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 5],
    is_end: bool,
}

impl<'a> Trie<'a> {
    fn ways_to_build<'b: 'a>(&mut self, design: &'b [Stripe]) -> usize {
        if let Some(&count) = self.cache.get(design) {
            return count;
        }
        let count = self
            .prefix_lengths(design)
            .iter()
            .map(|&l| self.ways_to_build(&design[l..]))
            .sum();
        self.cache.insert(design, count);
        count
    }

    fn can_build<'b: 'a>(&mut self, design: &'b [Stripe]) -> bool {
        // Much faster solutions are possible for part 1, but this prepopulates the cache making
        // part 2 just a series of lookups, so none of the work is wasted.
        self.ways_to_build(design) > 0
    }

    fn prefix_lengths(&self, design: &[Stripe]) -> Vec<usize> {
        let mut lengths = vec![];
        let mut node = &self.root;
        for (i, color) in design.iter().enumerate() {
            if let Some(ref next) = node.children[*color as usize] {
                node = next;
                if node.is_end {
                    lengths.push(i + 1);
                }
            } else {
                break;
            }
        }
        lengths
    }
}

mod parsing {
    use anyhow::{anyhow, Result};

    use super::{Stripe, Trie, TrieNode};

    pub fn parse_input(s: &str) -> Result<(Trie, Vec<Vec<Stripe>>)> {
        let (pattern_part, design_part) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("Failed to parse input."))?;
        let trie = pattern_part.parse()?;
        let designs = design_part
            .lines()
            .map(parse_pattern)
            .collect::<Result<_>>()?;
        Ok((trie, designs))
    }

    fn parse_pattern(s: &str) -> Result<Vec<Stripe>> {
        s.chars()
            .map(|c| match c {
                'w' => Ok(Stripe::White),
                'u' => Ok(Stripe::Blue),
                'b' => Ok(Stripe::Black),
                'r' => Ok(Stripe::Red),
                'g' => Ok(Stripe::Green),
                _ => Err(anyhow!("Unrecognized color {} in {}", c, s)),
            })
            .collect()
    }

    impl<'a> std::str::FromStr for Trie<'a> {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self> {
            let mut trie = Trie::default();
            for pattern in s.split(", ") {
                let mut node = &mut trie.root;
                let pattern = parse_pattern(pattern)?;
                for color in pattern {
                    node = node.children[color as usize]
                        .get_or_insert_with(|| Box::new(TrieNode::default()))
                        .as_mut();
                }
                node.is_end = true;
            }
            Ok(trie)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parsing::parse_input;

    static EXAMPLE: &str = "\
        r, wr, b, g, bwu, rb, gb, br\n\
        \n\
        brwrr\n\
        bggr\n\
        gbbr\n\
        rrbgbr\n\
        ubwu\n\
        bwurrg\n\
        brgr\n\
        bbrgwb";

    #[test]
    fn possible_designs() {
        let (mut trie, designs) = parse_input(EXAMPLE).unwrap();

        let results = designs
            .iter()
            .map(|design| trie.can_build(design))
            .collect::<Vec<_>>();
        let expected = vec![true, true, true, true, false, true, true, false];

        assert_eq!(results, expected);
    }

    #[test]
    fn ways_to_build() {
        let (mut trie, designs) = parse_input(EXAMPLE).unwrap();

        let results = designs
            .iter()
            .map(|design| trie.ways_to_build(design))
            .collect::<Vec<_>>();
        let expected = vec![2, 1, 4, 6, 0, 1, 2, 0];

        assert_eq!(results, expected);
    }
}
