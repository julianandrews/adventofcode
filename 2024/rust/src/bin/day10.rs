use anyhow::{anyhow, Result};

use aoc::planar::{CardinalDirection, TileMap};
use rustc_hash::FxHashSet;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let map: TrailMap = TrailMap(input.trim().parse().unwrap());

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn part1(map: &TrailMap) -> usize {
    map.trailheads().map(|(x, y)| map.score(x, y)).sum()
}

fn part2(map: &TrailMap) -> usize {
    map.trailheads().map(|(x, y)| map.rating(x, y)).sum()
}

#[derive(Debug, Clone)]
struct TrailMap(TileMap<Height>);

impl TrailMap {
    fn trailheads(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.0
            .iter_coords()
            .filter(|&(x, y)| self.0.get(x, y) == Some(&Height(0)))
    }

    fn score(&self, x: usize, y: usize) -> usize {
        let peaks: FxHashSet<_> = self.peaks(x, y).into_iter().collect();
        peaks.len()
    }

    fn rating(&self, x: usize, y: usize) -> usize {
        self.peaks(x, y).len()
    }

    fn peaks(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut peaks = vec![];
        let mut to_visit = vec![(x, y, 0)];
        while let Some((x, y, height)) = to_visit.pop() {
            if height == 9 {
                peaks.push((x, y));
            } else {
                to_visit.extend(self.neighbors(x, y, height));
            }
        }
        peaks
    }

    fn neighbors(&self, x: usize, y: usize, height: u32) -> Vec<(usize, usize, u32)> {
        let mut result = vec![];
        for direction in CardinalDirection::iter() {
            if let Some((nx, ny)) = self.0.step(x, y, direction) {
                if let Some(&Height(nh)) = self.0.get(nx, ny) {
                    if nh == height + 1 {
                        result.push((nx, ny, nh));
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Height(u32);

impl TryFrom<char> for Height {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(Height(value.to_digit(10).ok_or_else(|| {
            anyhow!("Unrecognized character {}", value)
        })?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732";

    #[test]
    fn score() {
        let map: TrailMap = TrailMap(TEST_DATA.parse().unwrap());

        assert_eq!(map.score(2, 0), 5);
        assert_eq!(map.score(4, 0), 6);
        assert_eq!(map.score(4, 2), 5);
        assert_eq!(map.score(6, 4), 3);
        assert_eq!(map.score(2, 5), 1);
        assert_eq!(map.score(5, 5), 3);
        assert_eq!(map.score(0, 6), 5);
        assert_eq!(map.score(6, 6), 3);
        assert_eq!(map.score(1, 7), 5);
        assert_eq!(part1(&map), 36);
    }

    #[test]
    fn rating() {
        let map: TrailMap = TrailMap(TEST_DATA.parse().unwrap());

        assert_eq!(map.rating(2, 0), 20);
        assert_eq!(map.rating(4, 0), 24);
        assert_eq!(map.rating(4, 2), 10);
        assert_eq!(map.rating(6, 4), 4);
        assert_eq!(map.rating(2, 5), 1);
        assert_eq!(map.rating(5, 5), 4);
        assert_eq!(map.rating(0, 6), 5);
        assert_eq!(map.rating(6, 6), 8);
        assert_eq!(map.rating(1, 7), 5);
        assert_eq!(part2(&map), 81);
    }
}
