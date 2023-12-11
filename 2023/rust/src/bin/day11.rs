use anyhow::{bail, Result};

use aoc::iterators::iter_pairs;
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: GalaxyMap = input.trim().parse()?;

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn part1(map: &GalaxyMap) -> usize {
    map.distances(1).sum()
}

fn part2(map: &GalaxyMap) -> usize {
    map.distances(999999).sum()
}

#[derive(Debug, Clone)]
struct GalaxyMap {
    /// Coordinates of galaxies.
    galaxies: Vec<(usize, usize)>,
    /// Prefix sums of number of row voids less than index.
    row_void_sums: Vec<usize>,
    /// Prefix sums of number of column voids less than index.
    column_void_sums: Vec<usize>,
}

impl GalaxyMap {
    pub fn distances(&self, expansion: usize) -> impl Iterator<Item = usize> + '_ {
        iter_pairs(&self.galaxies).map(move |(&a, &b)| self.distance(expansion, a, b))
    }

    fn distance(&self, expansion: usize, a: (usize, usize), b: (usize, usize)) -> usize {
        let x_range = a.0.min(b.0)..a.0.max(b.0);
        let y_range = a.1.min(b.1)..a.1.max(b.1);
        let void_rows = self.row_void_sums[y_range.end] - self.row_void_sums[y_range.start];
        let void_columns =
            self.column_void_sums[x_range.end] - self.column_void_sums[x_range.start];
        x_range.len() + y_range.len() + (void_rows + void_columns) * expansion
    }
}

impl std::str::FromStr for GalaxyMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(|l| l.len()).unwrap_or(0);
        let mut galaxies = vec![];
        for (y, line) in s.lines().enumerate() {
            if line.len() != width {
                bail!("Non rectangular map");
            }
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => galaxies.push((x, y)),
                    '.' => {}
                    _ => bail!("Unrecognized character in input"),
                }
            }
        }

        let row_void_sums = prefix_sums(s.lines().map(|line| line.chars().all(|c| c == '.')));

        let mut column_voids = vec![true; width];
        for galaxy in &galaxies {
            column_voids[galaxy.0] = false;
        }
        let column_void_sums = prefix_sums(column_voids.into_iter());

        Ok(GalaxyMap {
            galaxies,
            row_void_sums,
            column_void_sums,
        })
    }
}

fn prefix_sums(items: impl Iterator<Item = bool>) -> Vec<usize> {
    items
        .scan(0, |sum, b| {
            *sum += b as usize;
            Some(*sum)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....";

    #[test]
    fn distances() {
        let map: GalaxyMap = TEST_DATA.parse().unwrap();
        let result: Vec<usize> = map.distances(1).collect();
        let expected = [
            6, 6, 9, 9, 15, 15, 15, 12, 10, 5, 13, 9, 9, 19, 14, 11, 5, 17, 17, 9, 14, 8, 6, 6, 14,
            9, 12, 12, 6, 9, 6, 16, 11, 10, 5, 5,
        ];
        assert_eq!(result, expected);
    }
}
