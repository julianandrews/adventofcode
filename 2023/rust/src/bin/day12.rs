use anyhow::{anyhow, Result};
use rustc_hash::FxHashMap;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let records: Vec<Record> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&records));
    println!("Part 2: {}", part2(&records));

    Ok(())
}

fn part1(records: &[Record]) -> u64 {
    records.iter().map(|record| record.combinations()).sum()
}

fn part2(records: &[Record]) -> u64 {
    records
        .iter()
        .map(|record| record.unfold().combinations())
        .sum()
}

#[derive(Debug, Clone)]
struct Record {
    gears: Vec<Condition>,
    runs: Vec<u64>,
}

impl Record {
    pub fn combinations(&self) -> u64 {
        let mut cache = FxHashMap::default();
        self.combinations_helper(&mut cache, 0, 0)
    }

    pub fn unfold(&self) -> Record {
        let mut gears = self.gears.clone();
        gears.push(Condition::Unknown);
        gears = gears.repeat(5);
        gears.pop();
        let runs = self.runs.repeat(5);
        Record { gears, runs }
    }

    fn combinations_helper(
        &self,
        cache: &mut FxHashMap<(usize, usize), u64>,
        mut gear_offset: usize,
        run_offset: usize,
    ) -> u64 {
        let cache_key = (gear_offset, run_offset);
        if let Some(result) = cache.get(&cache_key) {
            return *result;
        }
        let mut run = match self.runs.get(run_offset) {
            Some(&run) => run,
            None => return self.rest_not_damaged(gear_offset) as u64,
        };

        // Skip any leading non-damaged gears.
        while self.gears.get(gear_offset) == Some(&Condition::Operational) {
            gear_offset += 1;
        }

        // If the first gear in the run is unknown figure out how many combinations we'd get
        // assuming it's undamaged.
        let undamaged_combos = match self.gears.get(gear_offset) {
            Some(&Condition::Unknown) => {
                self.combinations_helper(cache, gear_offset + 1, run_offset)
            }
            _ => 0,
        };

        // Try to advance to the end of the current run.
        while matches!(
            self.gears.get(gear_offset),
            Some(&Condition::Damaged | &Condition::Unknown)
        ) && run > 0
        {
            run -= 1;
            gear_offset += 1;
        }

        let damaged_combos = {
            if run > 0 {
                0
            } else {
                match self.gears.get(gear_offset) {
                    Some(Condition::Damaged) => 0,
                    Some(_) => self.combinations_helper(cache, gear_offset + 1, run_offset + 1),
                    None => self.runs[run_offset + 1..].is_empty() as u64,
                }
            }
        };
        let result = damaged_combos + undamaged_combos;
        cache.insert(cache_key, result);
        result
    }

    fn rest_not_damaged(&self, start: usize) -> bool {
        self.gears[start..]
            .iter()
            .all(|&gear| gear != Condition::Damaged)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl std::str::FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (gear_part, run_part) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid record: {}", s))?;
        let gears = gear_part
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<_>>()?;
        let runs = parse_fields(run_part, ',')?;
        Ok(Record { gears, runs })
    }
}

impl TryFrom<char> for Condition {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Condition::Operational),
            '#' => Ok(Condition::Damaged),
            '?' => Ok(Condition::Unknown),
            _ => Err(anyhow!("Unrecognized gear condition: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combinations_1() {
        let record: Record = "???.### 1,1,3".parse().unwrap();
        assert_eq!(record.combinations(), 1);
    }

    #[test]
    fn combinations_2() {
        let record: Record = ".??..??...?##. 1,1,3".parse().unwrap();
        assert_eq!(record.combinations(), 4);
    }

    #[test]
    fn combinations_3() {
        let record: Record = "?#?#?#?#?#?#?#? 1,3,1,6".parse().unwrap();
        assert_eq!(record.combinations(), 1);
    }

    #[test]
    fn combinations_4() {
        let record: Record = "????.#...#... 4,1,1".parse().unwrap();
        assert_eq!(record.combinations(), 1);
    }

    #[test]
    fn combinations_5() {
        let record: Record = "????.######..#####. 1,6,5".parse().unwrap();
        assert_eq!(record.combinations(), 4);
    }

    #[test]
    fn combinations_6() {
        let record: Record = "?###???????? 3,2,1".parse().unwrap();
        assert_eq!(record.combinations(), 10);
    }

    #[test]
    fn unfolded_1() {
        let record: Record = "???.### 1,1,3".parse().unwrap();
        assert_eq!(record.unfold().combinations(), 1);
    }

    #[test]
    fn unfolded_2() {
        let record: Record = ".??..??...?##. 1,1,3".parse().unwrap();
        assert_eq!(record.unfold().combinations(), 16384);
    }

    #[test]
    fn unfolded_3() {
        let record: Record = "?#?#?#?#?#?#?#? 1,3,1,6".parse().unwrap();
        assert_eq!(record.unfold().combinations(), 1);
    }

    #[test]
    fn unfolded_4() {
        let record: Record = "????.#...#... 4,1,1".parse().unwrap();
        assert_eq!(record.unfold().combinations(), 16);
    }

    #[test]
    fn unfolded_5() {
        let record: Record = "????.######..#####. 1,6,5".parse().unwrap();
        assert_eq!(record.unfold().combinations(), 2500);
    }

    #[test]
    fn unfolded_6() {
        let record: Record = "?###???????? 3,2,1".parse().unwrap();
        assert_eq!(record.unfold().combinations(), 506250);
    }
}
