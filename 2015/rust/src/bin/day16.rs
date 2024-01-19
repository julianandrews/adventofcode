use anyhow::{anyhow, Result};
use strum::IntoEnumIterator;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let sues: Vec<Sue> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&sues)?);
    println!("Part 2: {}", part2(&sues)?);

    Ok(())
}

fn part1(sues: &[Sue]) -> Result<u64> {
    find_sue(sues, MatchKind::Exact)
        .map(|sue| sue.number)
        .ok_or(anyhow!("Unique matching Sue not found"))
}

fn part2(sues: &[Sue]) -> Result<u64> {
    find_sue(sues, MatchKind::Correct)
        .map(|sue| sue.number)
        .ok_or(anyhow!("Unique matching Sue not found"))
}

fn find_sue(sues: &[Sue], match_kind: MatchKind) -> Option<Sue> {
    let mut found = None;
    for sue in sues {
        if sue.matches(match_kind) {
            match found {
                Some(_) => return None,
                None => found = Some(*sue),
            }
        }
    }
    found
}

#[derive(Debug, Clone, Copy, Default)]
struct Sue {
    number: u64,
    counts: [Option<u64>; 10],
}

impl Sue {
    fn matches(&self, match_kind: MatchKind) -> bool {
        static MFCSAMFACTS: [u64; 10] = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];

        FactKind::iter()
            .zip(self.counts)
            .zip(MFCSAMFACTS)
            .all(|((kind, count), expected)| match (count, match_kind) {
                (Some(count), MatchKind::Exact) => count == expected,
                (Some(count), MatchKind::Correct) => kind.matches(count, expected),
                (None, _) => true,
            })
    }
}

#[derive(Debug, Clone, Copy, strum_macros::EnumIter)]
enum FactKind {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizlas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl FactKind {
    fn matches(&self, count: u64, expected: u64) -> bool {
        match self {
            FactKind::Cats | FactKind::Trees => count > expected,
            FactKind::Pomeranians | FactKind::Goldfish => count < expected,
            _ => count == expected,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum MatchKind {
    Exact,
    Correct,
}

mod parsing {
    use super::{FactKind, Sue};

    use anyhow::{anyhow, bail};

    impl std::str::FromStr for Sue {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut sue = Sue::default();
            let (num_part, rest) = s
                .split_once(": ")
                .ok_or_else(|| anyhow!("Invalid Sue {}.", s))?;
            sue.number = num_part
                .strip_prefix("Sue ")
                .ok_or_else(|| anyhow!("Invalid Sue number in {}.", s))?
                .parse()?;
            for entry in rest.split(", ") {
                let (kind_part, count_part) = entry
                    .split_once(": ")
                    .ok_or_else(|| anyhow!("Invalid entry {} in {}.", entry, s))?;
                let kind: FactKind = kind_part.parse()?;
                let fact = &mut sue.counts[kind as usize];
                match *fact {
                    Some(_) => bail!("Repeated fact {} in {}.", kind_part, s),
                    _ => *fact = Some(count_part.parse()?),
                }
            }
            Ok(sue)
        }
    }

    impl std::str::FromStr for FactKind {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "children" => Ok(FactKind::Children),
                "cats" => Ok(FactKind::Cats),
                "samoyeds" => Ok(FactKind::Samoyeds),
                "pomeranians" => Ok(FactKind::Pomeranians),
                "akitas" => Ok(FactKind::Akitas),
                "vizslas" => Ok(FactKind::Vizlas),
                "goldfish" => Ok(FactKind::Goldfish),
                "trees" => Ok(FactKind::Trees),
                "cars" => Ok(FactKind::Cars),
                "perfumes" => Ok(FactKind::Perfumes),
                _ => bail!("Unrecognized kind {}.", s),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{find_sue, Sue};

    use aoc::utils::parse_fields;

    static TEST_DATA: &str = "\
        Sue 1: cats: 1, pomeranians: 3, samoyeds: 5\n\
        Sue 2: children: 3, trees: 4, pomeranians: 2\n\
        Sue 3: samoyeds: 2, trees: 3, perfumes: 1\n\
        Sue 4: cars: 10";

    #[test]
    fn find_sue_exact() {
        let sues: Vec<Sue> = parse_fields(TEST_DATA, '\n').unwrap();
        let sue = find_sue(&sues, crate::MatchKind::Exact).unwrap();

        assert_eq!(sue.number, 3);
    }

    #[test]
    fn find_sue_correct() {
        let sues: Vec<Sue> = parse_fields(TEST_DATA, '\n').unwrap();
        let sue = find_sue(&sues, crate::MatchKind::Correct).unwrap();

        assert_eq!(sue.number, 2);
    }
}
