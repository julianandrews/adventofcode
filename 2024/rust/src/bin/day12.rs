use anyhow::{bail, Result};

use aoc::planar::{CardinalDirection, TileMap};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let map = GardenMap(input.trim().parse()?);
    let regions = map.regions();

    println!("Part 1: {}", part1(&regions));
    println!("Part 2: {}", part2(&regions));

    Ok(())
}

fn part1(regions: &[Region]) -> usize {
    regions.iter().map(|r| r.area * r.perimeter).sum()
}

fn part2(regions: &[Region]) -> usize {
    regions.iter().map(|r| r.area * r.sides).sum()
}

#[derive(Debug, Clone)]
struct GardenMap(TileMap<Plot>);

impl GardenMap {
    fn regions(&self) -> Vec<Region> {
        let mut regions = vec![];
        let mut visited = vec![vec![false; self.0.width()]; self.0.height()];
        let mut roots = vec![(0, 0)];
        let mut to_visit = vec![];
        let mut region = Region::default();
        loop {
            let (x, y) = match to_visit.pop() {
                Some(value) => value,
                None => {
                    if region.area != 0 {
                        regions.push(region);
                        region = Region::default();
                    }
                    match roots.pop() {
                        Some(root) => {
                            region.label = (self.0.get(root.0, root.1).unwrap().0 + b'A') as char;
                            root
                        }
                        None => break,
                    }
                }
            };
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            let plot = *self.0.get(x, y).unwrap();
            let mut perimeter = 4;
            for (nx, ny) in self.neighbors(x, y) {
                if self.matches(nx, ny, plot) {
                    perimeter -= 1;
                    if !visited[ny][nx] {
                        to_visit.push((nx, ny));
                    }
                } else if !visited[ny][nx] {
                    roots.push((nx, ny));
                }
            }
            region.sides += self.corners(x, y, plot);
            region.area += 1;
            region.perimeter += perimeter;
        }
        regions
    }

    fn matches(&self, x: usize, y: usize, plot: Plot) -> bool {
        self.0.get(x, y).map(|&p| p == plot).unwrap_or(false)
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        CardinalDirection::iter().filter_map(move |d| self.0.step(x, y, d))
    }

    /// Count the number of corners at (x, y)
    fn corners(&self, x: usize, y: usize, plot: Plot) -> usize {
        let mut count = 0;
        let matches: Vec<bool> = CardinalDirection::iter()
            .map(|d| {
                self.0
                    .step(x, y, d)
                    .map(|(nx, ny)| self.matches(nx, ny, plot))
                    .unwrap_or(false)
            })
            .collect();
        for i in 0..4 {
            if matches[i] == matches[(i + 1) % 4] {
                if matches[i] {
                    let d1 = CardinalDirection::try_from(i as u8).unwrap();
                    let d2 = CardinalDirection::try_from((i as u8 + 1) % 4).unwrap();
                    let p = self
                        .0
                        .step(x, y, d1)
                        .and_then(|(nx, ny)| self.0.step(nx, ny, d2));
                    if let Some((nx, ny)) = p {
                        if !self.matches(nx, ny, plot) {
                            count += 1;
                        }
                    }
                } else {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Plot(u8);

impl TryFrom<char> for Plot {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'A'..='Z' => Ok(Plot(value as u8 - b'A')),
            _ => bail!("Unexpected character: {}", value),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Region {
    label: char,
    area: usize,
    perimeter: usize,
    sides: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        RRRRIICCFF\n\
        RRRRIICCCF\n\
        VVRRRCCFFF\n\
        VVRCCCJFFF\n\
        VVVVCJJCFE\n\
        VVIVCCJJEE\n\
        VVIIICJJEE\n\
        MIIIIIJJEE\n\
        MIIISIJEEE\n\
        MMMISSJEEE";

    #[test]
    fn regions() {
        let map = GardenMap(TEST_DATA.parse().unwrap());
        let mut regions = map.regions();
        regions.sort_unstable();
        #[rustfmt::skip]
        let mut expected = vec![
            Region { label: 'R', area: 12, perimeter: 18, sides: 10 },
            Region { label: 'I', area: 4, perimeter: 8, sides: 4 },
            Region { label: 'C', area: 14, perimeter: 28, sides: 22 },
            Region { label: 'F', area: 10, perimeter: 18, sides: 12 },
            Region { label: 'V', area: 13, perimeter: 20, sides: 10 },
            Region { label: 'J', area: 11, perimeter: 20, sides: 12 },
            Region { label: 'C', area: 1, perimeter: 4, sides: 4 },
            Region { label: 'E', area: 13, perimeter: 18, sides: 8 },
            Region { label: 'I', area: 14, perimeter: 22, sides: 16 },
            Region { label: 'M', area: 5, perimeter: 12, sides: 6 },
            Region { label: 'S', area: 3, perimeter: 8, sides: 6 },
        ];
        expected.sort_unstable();
        assert_eq!(regions, expected);
    }

    #[test]
    fn price() {
        let regions = GardenMap(TEST_DATA.parse().unwrap()).regions();
        assert_eq!(part1(&regions), 1930);
    }

    #[test]
    fn bulk_price() {
        let regions = GardenMap(TEST_DATA.parse().unwrap()).regions();
        assert_eq!(part2(&regions), 1206);
    }
}
