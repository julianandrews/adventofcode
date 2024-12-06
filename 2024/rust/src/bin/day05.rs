#![feature(iterator_try_collect)]

use anyhow::{anyhow, bail, Result};
use pageset::PageSet;

type Page = usize;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let (ruleset, updates) = parse_input(input.trim())?;

    println!("Part 1: {}", part1(&ruleset, &updates));
    println!("Part 2: {}", part2(&ruleset, &updates)?);

    Ok(())
}

fn parse_input(input: &str) -> Result<(RuleSet, Vec<Update>)> {
    let (rules_part, updates_part) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Failed to split sections"))?;
    let ruleset: RuleSet = rules_part.parse()?;
    let updates: Vec<Update> = aoc::utils::parse_fields(updates_part, '\n')?;

    Ok((ruleset, updates))
}

fn part1(ruleset: &RuleSet, updates: &[Update]) -> Page {
    updates
        .iter()
        .filter(|update| update.is_sorted(ruleset))
        .map(|update| update.middle_page())
        .sum()
}

fn part2(ruleset: &RuleSet, updates: &[Update]) -> Result<Page> {
    updates
        .iter()
        .filter(|update| !update.is_sorted(ruleset))
        .map(|update| update.sorted(ruleset).map(|update| update.middle_page()))
        .sum()
}

#[derive(Debug, Clone)]
struct Update(Vec<Page>);

impl Update {
    fn middle_page(&self) -> Page {
        self.0[self.0.len() / 2]
    }

    fn is_sorted(&self, ruleset: &RuleSet) -> bool {
        let mut seen = PageSet::default();
        for &page in &self.0 {
            if !(seen & ruleset.out_edges[page]).is_empty() {
                return false;
            }
            seen.insert(page);
        }
        true
    }

    fn sorted(&self, ruleset: &RuleSet) -> Result<Self> {
        let ruleset = ruleset.for_update(self);
        let mut result = vec![];
        let mut visited = PageSet::default();
        let mut to_visit: Vec<_> = {
            let is_root = |&page: &Page| ruleset.in_edges[page].is_empty();
            self.0.iter().copied().filter(is_root).collect()
        };
        while let Some(page) = to_visit.pop() {
            if visited.contains(page) {
                continue;
            }
            visited.insert(page);
            let candidates = ruleset.out_edges[page] & !visited;
            let neighbors = self.0.iter().copied().filter(|&n| candidates.contains(n));
            for neighbor in neighbors {
                if (ruleset.in_edges[neighbor] & !visited).is_empty() {
                    to_visit.push(neighbor);
                }
            }
            result.push(page);
        }
        if result.len() != self.0.len() {
            bail!("Failed to sort pages.");
        }
        Ok(Update(result))
    }

    fn pageset(&self) -> PageSet {
        PageSet(self.0.iter().fold(0, |value, page| value | (1 << page)))
    }
}

#[derive(Debug, Clone)]
struct RuleSet {
    out_edges: [PageSet; 100],
    in_edges: [PageSet; 100],
}

impl RuleSet {
    /// Return a ruleset with only rules for the pages in `update`.
    fn for_update(&self, update: &Update) -> Self {
        let mut ruleset = RuleSet {
            out_edges: [PageSet(0); 100],
            in_edges: [PageSet(0); 100],
        };
        let pagesset = update.pageset();
        for &page in &update.0 {
            ruleset.out_edges[page] = self.out_edges[page] & pagesset;
            ruleset.in_edges[page] = self.in_edges[page] & pagesset;
        }
        ruleset
    }
}

mod pageset {
    use crate::Page;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
    pub struct PageSet(pub u128);

    impl PageSet {
        pub fn insert(&mut self, page: Page) {
            self.0 |= 1 << page;
        }

        pub fn contains(&self, page: Page) -> bool {
            self.0 & (1 << page) != 0
        }

        pub fn is_empty(&self) -> bool {
            self.0 == 0
        }
    }

    impl std::ops::BitAnd for PageSet {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            PageSet(self.0 & rhs.0)
        }
    }

    impl std::ops::BitAndAssign for PageSet {
        fn bitand_assign(&mut self, rhs: Self) {
            self.0 &= rhs.0
        }
    }

    impl std::ops::BitOrAssign for PageSet {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 |= rhs.0
        }
    }

    impl std::ops::Not for PageSet {
        type Output = Self;

        fn not(self) -> Self::Output {
            PageSet(!self.0)
        }
    }
}

impl std::str::FromStr for RuleSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut in_edges = [PageSet(0); 100];
        let mut out_edges = [PageSet(0); 100];
        for line in s.lines() {
            let Some((left, right)) = line.split_once('|') else {
                bail!("Failed to split '{}'", line);
            };
            let (left, right): (Page, Page) = (left.parse()?, right.parse()?);
            if left > 99 || right > 99 {
                bail!("Value too large in '{}'", line);
            }
            out_edges[left].insert(right);
            in_edges[right].insert(left);
        }
        Ok(RuleSet {
            in_edges,
            out_edges,
        })
    }
}

impl std::str::FromStr for Update {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pages: Vec<_> = aoc::utils::parse_fields(s, ',')?;
        if pages.iter().any(|&v| v > 99) {
            bail!("Value too large in '{}'", s);
        }
        if pages.len() % 2 == 0 {
            bail!("Even number of pages in '{}'", s);
        }
        Ok(Update(pages))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47";

    #[test]
    fn is_sorted() {
        let (ruleset, updates) = parse_input(TEST_DATA).unwrap();
        let result: Vec<_> = updates
            .iter()
            .map(|update| update.is_sorted(&ruleset))
            .collect();
        let expected = vec![true, true, true, false, false, false];

        assert_eq!(result, expected);
    }

    #[test]
    fn sort() {
        let (ruleset, updates) = parse_input(TEST_DATA).unwrap();

        assert_eq!(
            updates[3].sorted(&ruleset).unwrap().0,
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(updates[4].sorted(&ruleset).unwrap().0, vec![61, 29, 13]);
        assert_eq!(
            updates[5].sorted(&ruleset).unwrap().0,
            vec![97, 75, 47, 29, 13]
        );
    }
}
