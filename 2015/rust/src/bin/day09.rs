use itertools::Itertools;

use aoc::utils::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let distances: DistanceGraph = input.trim().parse()?;
    let trip_lengths = distances.all_trip_lengths();

    println!("Part 1: {}", part1(&trip_lengths));
    println!("Part 2: {}", part2(&trip_lengths));

    Ok(())
}

fn part1(trip_lengths: &[u64]) -> u64 {
    *trip_lengths.iter().min().unwrap_or(&0)
}

fn part2(trip_lengths: &[u64]) -> u64 {
    *trip_lengths.iter().max().unwrap_or(&0)
}

#[derive(Debug, Clone)]
pub struct DistanceGraph {
    places: Vec<String>,
    distances: Vec<Vec<u64>>,
}

impl DistanceGraph {
    fn all_trip_lengths(&self) -> Vec<u64> {
        let mut trip_lengths = vec![];
        for trip in (0..self.places.len()).permutations(self.places.len()) {
            trip_lengths.push(
                trip.windows(2)
                    .map(|pair| self.distances.get(pair[0]).unwrap().get(pair[1]).unwrap())
                    .sum(),
            )
        }
        trip_lengths
    }
}

mod parsing {
    use super::DistanceGraph;

    use anyhow::{anyhow, Result};
    use rustc_hash::FxHashMap;

    impl std::str::FromStr for DistanceGraph {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut distance_map = FxHashMap::default();
            for line in s.lines() {
                let (place_part, distance) = line
                    .split_once(" = ")
                    .ok_or_else(|| anyhow!("Invalid entry {}", line))?;
                let (from, to) = place_part
                    .split_once(" to ")
                    .ok_or_else(|| anyhow!("Invalid entry {}", line))?;
                let distance: u64 = distance.parse()?;
                distance_map
                    .entry(from.to_string())
                    .or_insert_with(FxHashMap::default)
                    .insert(to.to_string(), distance);
                distance_map
                    .entry(to.to_string())
                    .or_insert_with(FxHashMap::default)
                    .insert(from.to_string(), distance);
            }
            let (places, distances) = aoc::utils::build_index_map(distance_map, || u64::MAX);
            Ok(DistanceGraph { places, distances })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DistanceGraph;

    static TEST_DATA: &str = "\
        London to Dublin = 464\n\
        London to Belfast = 518\n\
        Dublin to Belfast = 141";

    #[test]
    fn shortest_trip() {
        let distances: DistanceGraph = TEST_DATA.parse().unwrap();
        let shortest = *distances.all_trip_lengths().iter().min().unwrap();

        assert_eq!(shortest, 605);
    }

    #[test]
    fn longest_trip() {
        let distances: DistanceGraph = TEST_DATA.parse().unwrap();
        let longest = *distances.all_trip_lengths().iter().max().unwrap();

        assert_eq!(longest, 982);
    }
}
