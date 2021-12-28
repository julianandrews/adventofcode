use std::collections::BTreeMap;
use std::convert::TryInto;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let (template, rules_part) = input
        .trim()
        .split_once("\n\n")
        .ok_or(AOCError::new("Failed to split input"))?;
    let rules: Rules = rules_part.parse()?;

    println!("Part 1: {}", part1(&template, &rules));
    println!("Part 2: {}", part2(&template, &rules));
    Ok(())
}

fn part1(template: &str, rules: &Rules) -> u64 {
    most_common_minus_least_common(template, rules, 10)
}

fn part2(template: &str, rules: &Rules) -> u64 {
    most_common_minus_least_common(template, rules, 40)
}

fn most_common_minus_least_common(template: &str, rules: &Rules, steps: usize) -> u64 {
    let letter_counts = get_letter_counts(template, rules, steps);
    let mut letter_counts: Vec<_> = letter_counts.values().collect();
    letter_counts.sort_unstable();
    if letter_counts.len() == 0 {
        0
    } else {
        *letter_counts.last().unwrap() - *letter_counts.first().unwrap()
    }
}

/// Find the counts of letters after `steps` steps.
fn get_letter_counts(template: &str, rules: &Rules, steps: usize) -> BTreeMap<u8, u64> {
    let mut pair_counts: BTreeMap<[u8; 2], u64> = BTreeMap::new();
    for pair in template.as_bytes().windows(2) {
        *pair_counts.entry(pair.try_into().unwrap()).or_insert(0) += 1;
    }
    for _ in 0..steps {
        pair_counts = step_pair_counts(rules, &pair_counts);
    }

    let mut letter_counts = BTreeMap::new();
    for ([a, b], count) in pair_counts {
        *letter_counts.entry(a).or_insert(0) += count;
        *letter_counts.entry(b).or_insert(0) += count;
    }
    // All letters except the first and last in `template` appear in two pairs. The first and last
    // letters will only have been counted once. Ceiling division does the right thing in all
    // cases.
    for (_, count) in letter_counts.iter_mut() {
        *count = (*count + 1) / 2;
    }

    letter_counts
}

/// Take the existing counts of letter pairs, and find the counts of letter pairs after a step.
fn step_pair_counts(rules: &Rules, counts: &BTreeMap<[u8; 2], u64>) -> BTreeMap<[u8; 2], u64> {
    let mut new_counts = BTreeMap::new();
    for (pair, count) in counts {
        match rules.rules.get(pair) {
            Some(&b) => {
                *new_counts.entry([pair[0], b]).or_insert(0) += count;
                *new_counts.entry([b, pair[1]]).or_insert(0) += count;
            }
            None => *new_counts.entry(*pair).or_insert(0) += count,
        }
    }
    new_counts
}

#[derive(Debug)]
struct Rules {
    rules: BTreeMap<[u8; 2], u8>,
}

impl std::str::FromStr for Rules {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let rules = s
            .lines()
            .map(|line| {
                let (from, to) = line
                    .split_once(" -> ")
                    .ok_or(AOCError::new("Failed to parse rule"))?;
                let from: [u8; 2] = from.as_bytes().try_into()?;
                let to: [u8; 1] = to.as_bytes().try_into()?;

                Ok((from, to[0]))
            })
            .collect::<Result<_>>()?;
        Ok(Rules { rules })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_RULES: &str = "CH -> B\
                             \nHH -> N\
                             \nCB -> H\
                             \nNH -> C\
                             \nHB -> C\
                             \nHC -> B\
                             \nHN -> C\
                             \nNN -> C\
                             \nBH -> H\
                             \nNC -> B\
                             \nNB -> B\
                             \nBN -> B\
                             \nBB -> N\
                             \nBC -> B\
                             \nCC -> N\
                             \nCN -> C";

    #[test]
    fn full_logic() {
        let template = "NNCB";
        let rules: Rules = TEST_RULES.parse().unwrap();

        let result = most_common_minus_least_common(template, &rules, 10);
        assert_eq!(result, 1588);
    }
}
