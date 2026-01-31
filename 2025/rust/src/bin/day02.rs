#![feature(iterator_try_collect)]

use rustc_hash::FxHashSet;

fn main() -> anyhow::Result<()> {
    let input = aoc_2025::utils::get_input()?;
    let ranges = parsing::parse_input(input.trim())?;

    println!("{}", part1(&ranges));
    println!("{}", part2(&ranges));

    Ok(())
}

fn part1(ranges: &[IdRange]) -> u64 {
    ranges.iter().flat_map(|r| r.simple_invalid_ids()).sum()
}

fn part2(ranges: &[IdRange]) -> u64 {
    ranges
        .iter()
        .flat_map(|r| r.invalid_ids())
        .collect::<FxHashSet<_>>()
        .into_iter()
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    fn simple_invalid_ids(&self) -> Vec<u64> {
        self.ids_for_num_repeats(2)
    }

    fn invalid_ids(&self) -> FxHashSet<u64> {
        let mut ids = FxHashSet::default();

        for num_repeats in 2..=count_digits(self.start) {
            ids.extend(self.ids_for_num_repeats(num_repeats));
        }

        ids
    }

    fn ids_for_num_repeats(&self, num_repeats: u64) -> Vec<u64> {
        let mut ids = Vec::new();
        let digits = count_digits(self.start);
        let prefix_digits = digits / num_repeats;
        let divisor = 10u64.pow((digits - prefix_digits) as u32);
        for prefix in self.start / divisor..=self.end / divisor {
            // Build an id by repeating `prefix`
            let multiplier = 10u64.pow(prefix_digits as u32);
            let id = (0..num_repeats).fold(0, |acc, _| acc * multiplier + prefix);
            if id >= self.start && id <= self.end {
                ids.push(id);
            }
        }
        ids
    }
}

fn count_digits(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => n.ilog10() as u64 + 1,
    }
}

mod parsing {
    use super::{count_digits, IdRange};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct RawIdRange {
        start: u64,
        end: u64,
    }

    impl RawIdRange {
        pub fn split_by_digit_count(&self) -> Vec<IdRange> {
            let mut result = Vec::new();
            let mut current_start = self.start;

            while current_start <= self.end {
                let start_digits = count_digits(current_start) as u32;
                let subrange_end = (10u64.pow(start_digits) - 1).min(self.end);

                result.push(IdRange {
                    start: current_start,
                    end: subrange_end,
                });

                current_start = subrange_end + 1;
            }
            result
        }
    }

    impl std::str::FromStr for RawIdRange {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (start, end) = s
                .split_once('-')
                .ok_or_else(|| anyhow::anyhow!("Failed to parse range: '{}'", s))?;
            let (start, end) = (start.parse()?, end.parse()?);
            if start > end {
                anyhow::bail!("Invalid range: {} larger than {}", start, end);
            }
            Ok(RawIdRange { start, end })
        }
    }

    // Parse the input splitting ranges as needed so each range has a fixed number of digits.
    pub fn parse_input(s: &str) -> anyhow::Result<Vec<IdRange>> {
        let mut result: Vec<RawIdRange> = s.split(',').map(|r| r.parse()).try_collect()?;
        result.sort_unstable();
        Ok(result
            .into_iter()
            .flat_map(|r| r.split_by_digit_count())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::{parsing::parse_input, IdRange};

    static TEST_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                             1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                             824824821-824824827,2121212118-2121212124";

    fn make_range(start: u64, end: u64) -> IdRange {
        IdRange { start, end }
    }

    #[test]
    fn parsing() {
        let ranges = parse_input(TEST_DATA).unwrap();
        let expected = vec![
            make_range(11, 22),
            make_range(95, 99),
            make_range(100, 115),
            make_range(998, 999),
            make_range(1000, 1012),
            make_range(222220, 222224),
            make_range(446443, 446449),
            make_range(565653, 565659),
            make_range(1698522, 1698528),
            make_range(38593856, 38593862),
            make_range(824824821, 824824827),
            make_range(1188511880, 1188511890),
            make_range(2121212118, 2121212124),
        ];

        assert_eq!(ranges, expected);
    }

    #[test]
    fn simple_invalid_ids() {
        let ranges = parse_input(TEST_DATA).unwrap();
        let result = ranges
            .into_iter()
            .flat_map(|r| r.simple_invalid_ids())
            .collect::<Vec<_>>();

        assert_eq!(
            result,
            vec![11, 22, 99, 1010, 222222, 446446, 38593859, 1188511885]
        );
    }

    #[test]
    fn invalid_ids() {
        let ranges = parse_input(TEST_DATA).unwrap();
        let result = ranges
            .into_iter()
            .flat_map(|r| r.invalid_ids())
            .collect::<Vec<_>>();

        assert_eq!(
            result,
            vec![
                11, 22, 99, 111, 999, 1010, 222222, 446446, 565656, 38593859, 824824824,
                1188511885, 2121212121
            ]
        );
    }
}
