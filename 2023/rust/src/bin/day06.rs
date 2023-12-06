use anyhow::Result;
use std::ops::Range;

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let races = parsing::parse_races(input.trim())?;
    let race = parsing::parse_miskerned_race(input.trim())?;

    println!("Part 1: {}", part1(&races));
    println!("Part 2: {}", part2(race));

    Ok(())
}

fn part1(races: &[Race]) -> u64 {
    races
        .iter()
        .map(|race| {
            let range = race.win_range();
            range.end - range.start
        })
        .product()
}

fn part2(race: Race) -> u64 {
    let range = race.win_range();
    range.end - range.start
}

#[derive(Debug, Clone, Copy)]
pub struct Race {
    time: u64,
    record: u64,
}

impl Race {
    pub fn win_range(&self) -> Range<u64> {
        let t = self.time as f64;
        let sqrt = (t * t - 4.0 * self.record as f64).sqrt();
        let start = ((t - sqrt) / 2.0).floor() as u64 + 1;
        let end = ((t + sqrt) / 2.0).ceil() as u64;
        start..end
    }
}

mod parsing {
    use anyhow::{anyhow, Result};
    use std::num::ParseIntError;

    use super::Race;

    pub fn parse_races(s: &str) -> Result<Vec<Race>> {
        let (time_part, record_part) =
            get_race_parts(s).ok_or_else(|| anyhow!("Invalid races: {}", s))?;
        time_part
            .split_whitespace()
            .zip(record_part.split_whitespace())
            .map(|(time, record)| {
                Ok(Race {
                    time: time.parse()?,
                    record: record.parse()?,
                })
            })
            .collect()
    }

    pub fn parse_miskerned_race(s: &str) -> Result<Race> {
        let (time_part, record_part) =
            get_race_parts(s).ok_or_else(|| anyhow!("Invalid race: {}", s))?;
        let time = parse_miskerned_number(time_part)?;
        let record = parse_miskerned_number(record_part)?;
        Ok(Race { time, record })
    }

    fn get_race_parts(s: &str) -> Option<(&str, &str)> {
        let (time_part, record_part) = s.split_once('\n')?;
        Some((
            time_part.strip_prefix("Time:")?,
            record_part.strip_prefix("Distance:")?,
        ))
    }

    fn parse_miskerned_number(s: &str) -> Result<u64, ParseIntError> {
        s.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        Time:      7  15   30\n\
        Distance:  9  40  200";

    #[test]
    fn multiple_races() {
        let races = parsing::parse_races(TEST_DATA).unwrap();
        let result: Vec<_> = races.iter().map(|r| r.win_range()).collect();
        let expected = vec![2..6, 4..12, 11..20];
        assert_eq!(result, expected);
        assert_eq!(part1(&races), 288)
    }

    #[test]
    fn single_race() {
        let race = parsing::parse_miskerned_race(TEST_DATA).unwrap();
        let range = race.win_range();
        assert_eq!(range, 14..71517);
        assert_eq!(part2(race), 71503);
    }
}
