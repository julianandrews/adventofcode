#![feature(iterator_try_collect)]

type Id = u64;

fn main() -> anyhow::Result<()> {
    let input = aoc_2025::utils::get_input()?;
    let (ranges, ids) = parsing::parse_input(input.trim())?;

    println!("{}", part1(&ranges, &ids));
    println!("{}", part2(&ranges));

    Ok(())
}

fn part1(ranges: &[IdRange], ids: &[Id]) -> usize {
    ids.iter().filter(|&id| ranges.contains_id(*id)).count()
}

fn part2(ranges: &[IdRange]) -> usize {
    ranges.merge().into_iter().map(|range| range.len()).sum()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct IdRange {
    start: Id,
    end: Id,
}

impl IdRange {
    fn len(&self) -> usize {
        (self.end + 1 - self.start) as usize
    }
}

trait IdRangeSliceExt {
    fn contains_id(&self, id: Id) -> bool;

    fn merge(&self) -> Vec<IdRange>;
}

impl IdRangeSliceExt for [IdRange] {
    fn contains_id(&self, id: Id) -> bool {
        self.iter()
            .any(|range| id >= range.start && id <= range.end)
    }

    fn merge(&self) -> Vec<IdRange> {
        let mut ranges = self.to_vec();
        ranges.sort_unstable();
        let mut ranges = ranges.into_iter();

        let mut merged = match ranges.next() {
            Some(range) => vec![range],
            None => vec![],
        };

        for range in ranges {
            let last = merged.last_mut().unwrap();

            if range.start <= last.end + 1 {
                last.end = range.end.max(last.end);
            } else {
                merged.push(range);
            }
        }

        merged
    }
}

mod parsing {
    use crate::{Id, IdRange};

    use anyhow::{anyhow, Result};

    pub fn parse_input(s: &str) -> Result<(Vec<IdRange>, Vec<Id>)> {
        let (range_part, id_part) = s
            .trim()
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("Failed to split input"))?;
        let ranges = range_part.lines().map(|line| line.parse()).try_collect()?;
        let ids = id_part.lines().map(|id| id.parse::<Id>()).try_collect()?;

        Ok((ranges, ids))
    }

    impl std::str::FromStr for IdRange {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            let (a, b) = s
                .split_once('-')
                .ok_or_else(|| anyhow!("Failed to parse range: {}", s))?;
            Ok(IdRange {
                start: a.parse()?,
                end: b.parse()?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parsing, Id, IdRange, IdRangeSliceExt};

    static TEST_DATA: &str = "\
        3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17\n\
        32";

    fn make_range(start: Id, end: Id) -> IdRange {
        IdRange { start, end }
    }

    #[test]
    fn parsing() {
        let (ranges, ids) = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(
            ranges,
            vec![
                make_range(3, 5),
                make_range(10, 14),
                make_range(16, 20),
                make_range(12, 18),
            ]
        );
        assert_eq!(ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn contains_id() {
        let (ranges, _) = parsing::parse_input(TEST_DATA).unwrap();
        assert!(!ranges.contains_id(1));
        assert!(ranges.contains_id(5));
        assert!(!ranges.contains_id(8));
        assert!(ranges.contains_id(11));
        assert!(ranges.contains_id(17));
        assert!(!ranges.contains_id(32));
    }

    #[test]
    fn merge() {
        let (ranges, _) = parsing::parse_input(TEST_DATA).unwrap();
        let merged = ranges.merge();
        assert_eq!(merged, vec![make_range(3, 5), make_range(10, 20)]);
    }
}
