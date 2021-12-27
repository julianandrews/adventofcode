use std::collections::BTreeSet;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let map = input.trim().parse()?;

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
    Ok(())
}

fn part1(map: &ElevationMap) -> u32 {
    let mut total = 0;
    for (x, y) in map.low_points() {
        total += map.at(x, y).unwrap() + 1;
    }
    total
}

fn part2(map: &ElevationMap) -> usize {
    let mut basins = vec![];
    for low_point in map.low_points() {
        let mut to_process = vec![low_point];
        let mut basin = BTreeSet::new();
        while let Some(p) = to_process.pop() {
            if !basin.contains(&p) && map.at(p.0, p.1) != Some(9) {
                basin.insert(p);
                to_process.extend(map.neighbors(p.0, p.1));
            }
        }
        basins.push(basin);
    }
    basins.sort_by_key(|basin| basin.len());
    basins
        .iter()
        .rev()
        .take(3)
        .fold(1, |total, basin| total * basin.len())
}

struct ElevationMap {
    heights: Vec<Vec<u32>>,
}

impl ElevationMap {
    fn height(&self) -> usize {
        self.heights.len()
    }

    fn width(&self) -> usize {
        self.heights.get(0).map(|row| row.len()).unwrap_or(0)
    }

    fn at(&self, x: usize, y: usize) -> Option<u32> {
        self.heights.get(y)?.get(x).copied()
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let mut neighbors = vec![];
        if x < self.width() - 1 && y < self.height() {
            neighbors.push((x + 1, y))
        }
        if x > 0 && y < self.height() {
            neighbors.push((x - 1, y))
        }
        if y < self.height() - 1 && x < self.width() {
            neighbors.push((x, y + 1))
        }
        if y > 0 && x < self.width() {
            neighbors.push((x, y - 1))
        }
        neighbors.into_iter()
    }

    fn low_points(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut low_points = vec![];
        for (y, row) in self.heights.iter().enumerate() {
            for (x, height) in row.iter().enumerate() {
                if self
                    .neighbors(x, y)
                    .all(|(x, y)| self.at(x, y).unwrap() > *height)
                {
                    low_points.push((x, y));
                }
            }
        }
        low_points.into_iter()
    }
}

impl std::str::FromStr for ElevationMap {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let heights: Vec<Vec<u32>> = s
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10)).collect())
            .collect::<Option<_>>()
            .ok_or(AOCError::new("Unrecognized character"))?;
        let width = heights.get(0).map(|row| row.len()).unwrap_or(0);
        if !heights.iter().all(|row| row.len() == width) {
            return Err(Box::new(AOCError::new("Non-rectangular map")));
        }
        Ok(ElevationMap { heights })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = "2199943210\
                            \n3987894921\
                            \n9856789892\
                            \n8767896789\
                            \n9899965678";

    #[test]
    fn test_part_1() {
        let map = TEST_DATA.parse().unwrap();
        let result = part1(&map);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part_2() {
        let map = TEST_DATA.parse().unwrap();
        let result = part2(&map);
        assert_eq!(result, 1134);
    }
}
