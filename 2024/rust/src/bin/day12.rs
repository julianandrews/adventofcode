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
    regions.iter().map(Region::price).sum()
}

fn part2(regions: &[Region]) -> usize {
    regions.iter().map(Region::bulk_price).sum()
}

#[derive(Debug, Clone)]
struct GardenMap(TileMap<Plot>);

impl GardenMap {
    fn regions(&self) -> Vec<Region> {
        let mut regions = vec![];
        let mut visited = vec![vec![false; self.0.width()]; self.0.height()];
        let mut roots = vec![(0, 0)];
        let mut neighbors = vec![];
        let mut side_tracker = SideTracker::new(self.0.height(), self.0.width());
        let mut region = Region::default();
        loop {
            let (x, y) = match neighbors.pop() {
                Some(value) => value,
                None => {
                    if region.area != 0 {
                        region.sides = side_tracker.count();
                        side_tracker.clear();
                        regions.push(region);
                        region = Region::default();
                    }
                    match roots.pop() {
                        Some(root) => root,
                        None => break,
                    }
                }
            };
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            let plot = self.0.get(x, y);
            region.area += 1;
            region.perimeter += 4;
            for (neighbor, direction) in self.neighbors(x, y) {
                let (nx, ny) = match neighbor {
                    Some(neighbor) => neighbor,
                    None => {
                        side_tracker.add(direction, x, y);
                        continue;
                    }
                };
                if self.0.get(nx, ny) == plot {
                    region.perimeter -= 1;
                    if !visited[ny][nx] {
                        neighbors.push((nx, ny));
                    }
                } else {
                    side_tracker.add(direction, x, y);
                    if !visited[ny][nx] {
                        roots.push((nx, ny));
                    }
                }
            }
        }
        regions
    }

    pub fn neighbors(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (Option<(usize, usize)>, CardinalDirection)> + '_ {
        CardinalDirection::iter().map(move |d| (self.0.step(x, y, d), d))
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
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    fn price(&self) -> usize {
        self.area * self.perimeter
    }

    fn bulk_price(&self) -> usize {
        self.area * self.sides
    }
}

#[derive(Debug, Clone, Default)]
struct SideTracker([Vec<Vec<bool>>; 4]);

impl SideTracker {
    fn new(height: usize, width: usize) -> Self {
        SideTracker(std::array::from_fn(|i| {
            if i % 2 == 0 {
                vec![vec![false; width]; height]
            } else {
                vec![vec![false; height]; width]
            }
        }))
    }

    fn add(&mut self, direction: CardinalDirection, x: usize, y: usize) {
        let (position, value) = match direction {
            CardinalDirection::North | CardinalDirection::South => (y, x),
            CardinalDirection::East | CardinalDirection::West => (x, y),
        };

        self.0[u8::from(direction) as usize][position][value] = true;
    }

    fn clear(&mut self) {
        for v1 in &mut self.0 {
            for v2 in v1 {
                for value in v2 {
                    *value = false;
                }
            }
        }
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for v1 in &self.0 {
            for v2 in v1 {
                for pair in v2.windows(2) {
                    if pair[1] && !pair[0] {
                        count += 1;
                    }
                }
                if v2.first() == Some(&true) {
                    count += 1;
                }
            }
        }
        count
    }
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
            Region { area: 12, perimeter: 18, sides: 10 },
            Region { area: 4, perimeter: 8, sides: 4 },
            Region { area: 14, perimeter: 28, sides: 22 },
            Region { area: 10, perimeter: 18, sides: 12 },
            Region { area: 13, perimeter: 20, sides: 10 },
            Region { area: 11, perimeter: 20, sides: 12 },
            Region { area: 1, perimeter: 4, sides: 4 },
            Region { area: 13, perimeter: 18, sides: 8 },
            Region { area: 14, perimeter: 22, sides: 16 },
            Region { area: 5, perimeter: 12, sides: 6 },
            Region { area: 3, perimeter: 8, sides: 6 },
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
