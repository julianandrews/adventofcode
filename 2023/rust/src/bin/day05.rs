use std::ops::Range;

use anyhow::{anyhow, bail, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let almanac: Almanac = input.trim().parse()?;

    println!("Part 1: {}", part1(&almanac)?);
    println!("Part 2: {}", part2(&almanac)?);

    Ok(())
}

fn part1(almanac: &Almanac) -> Result<u64> {
    almanac
        .seed_images()
        .min()
        .ok_or(anyhow!("No seeds in input"))
}

fn part2(almanac: &Almanac) -> Result<u64> {
    almanac
        .seed_range_images()
        .map(|r| r.start)
        .min()
        .ok_or(anyhow!("No seeds in input"))
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<RangeMapCollection>,
}

impl Almanac {
    /// Return the final value of all seeds after mapping through all maps.
    pub fn seed_images<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
        self.seeds
            .iter()
            .map(|&seed| self.maps.iter().fold(seed, |number, map| map.get(number)))
    }

    /// Return the image of all seeds interpreted as ranges as a collection of ranges after mapping through all maps.
    pub fn seed_range_images<'a>(&'a self) -> impl Iterator<Item = Range<u64>> + 'a {
        let seed_ranges = self.seeds.chunks(2).map(|p| p[0]..p[0] + p[1]);
        seed_ranges.flat_map(|range| self.range_image(range))
    }

    fn range_image<'a>(&'a self, range: Range<u64>) -> impl Iterator<Item = Range<u64>> + 'a {
        let mut ranges = vec![range];
        for map in self.maps.iter() {
            ranges = ranges
                .into_iter()
                .flat_map(|r| map.image(r.clone()))
                .collect();
        }
        ranges.into_iter()
    }
}

#[derive(Debug, Clone)]
struct RangeMapCollection {
    ranges: Vec<RangeMap>,
}

impl RangeMapCollection {
    /// Return the value mapped to by number.
    pub fn get(&self, number: u64) -> u64 {
        for range in &self.ranges {
            if range.source.contains(&number) {
                return range.destination.start + number - range.source.start;
            }
        }
        number
    }

    /// Return the image of a range as a collection of ranges.
    pub fn image(&self, mut range: Range<u64>) -> Vec<Range<u64>> {
        let mut ranges = vec![];
        for r in self.overlapping(range.clone()) {
            if r.source.start > range.start {
                ranges.push(range.start..r.source.start);
            }
            ranges.push(r.map_overlap(&range));
            range = r.source.end..range.end;
        }
        if !range.is_empty() {
            ranges.push(range);
        }

        ranges
    }

    fn overlapping<'a>(&'a self, range: Range<u64>) -> impl Iterator<Item = &RangeMap> + 'a {
        self.ranges
            .iter()
            .skip_while(move |r| r.source.end < range.start)
            .take_while(move |r| r.source.start < range.end)
    }
}

#[derive(Debug, Clone)]
struct RangeMap {
    source: Range<u64>,
    destination: Range<u64>,
}

impl RangeMap {
    /// Return the image of any part of range that overlaps source.
    pub fn map_overlap(&self, range: &Range<u64>) -> Range<u64> {
        let start = self.destination.start + range.start.saturating_sub(self.source.start);
        let end = self
            .destination
            .end
            .saturating_sub(self.source.end.saturating_sub(range.end));
        start..end
    }
}

impl std::str::FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let seeds = parts
            .next()
            .and_then(|s| s.strip_prefix("seeds: "))
            .ok_or_else(|| anyhow!("Invalid Almanac: {}", s))?
            .split(' ')
            .map(|n| Ok(n.parse()?))
            .collect::<Result<_>>()?;
        let mut maps = vec![];
        let mut expected_source = "seed";
        for map_part in parts {
            let (kind_part, range_part) = map_part
                .split_once('\n')
                .ok_or_else(|| anyhow!("Invalid RangeMapCollection: {}", map_part))?;
            let (source, destination) = kind_part
                .strip_suffix(" map:")
                .and_then(|s| s.split_once("-to-"))
                .ok_or_else(|| anyhow!("Invalid RangeMapCollection: {}", map_part))?;
            if expected_source != source {
                bail!(
                    "Expected mapping from {}, but found {}",
                    expected_source,
                    source
                );
            }
            expected_source = destination;
            maps.push(range_part.parse()?);
        }
        Ok(Almanac { seeds, maps })
    }
}

impl std::str::FromStr for RangeMapCollection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges: Vec<RangeMap> = parse_fields(s, '\n')?;
        ranges.sort_unstable_by_key(|r| r.source.start);
        Ok(RangeMapCollection { ranges })
    }
}

impl std::str::FromStr for RangeMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u64> = s
            .split(' ')
            .map(|n| Ok(n.parse()?))
            .collect::<Result<_>>()?;
        if numbers.len() != 3 {
            bail!("Invalid RangeMap: {}", s);
        }
        Ok(RangeMap {
            source: numbers[1]..numbers[1] + numbers[2],
            destination: numbers[0]..numbers[0] + numbers[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4";

    #[test]
    fn seed_images() {
        let almanac: Almanac = TEST_DATA.parse().unwrap();
        let result: Vec<_> = almanac.seed_images().collect();
        let expected = vec![82, 43, 86, 35];
        assert_eq!(result, expected);
    }

    #[test]
    fn map_overlap() {
        fn new_map(source: Range<u64>, destination: Range<u64>) -> RangeMap {
            RangeMap {
                source,
                destination,
            }
        }

        assert!(new_map(10..20, 5..15).map_overlap(&(1..5)).is_empty());
        assert!(new_map(10..20, 5..15).map_overlap(&(20..25)).is_empty());
        assert_eq!(new_map(10..20, 5..15).map_overlap(&(1..12)), 5..7);
        assert_eq!(new_map(10..20, 5..15).map_overlap(&(15..25)), 10..15);
        assert_eq!(new_map(10..20, 5..15).map_overlap(&(12..17)), 7..12);
        assert_eq!(new_map(10..20, 5..15).map_overlap(&(5..25)), 5..15);
    }

    #[test]
    fn min_location() {
        let almanac: Almanac = TEST_DATA.parse().unwrap();
        assert_eq!(part2(&almanac).unwrap(), 46);
    }
}
