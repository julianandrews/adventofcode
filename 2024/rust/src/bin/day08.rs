use anyhow::{bail, Result};
use rustc_hash::FxHashSet;

use aoc::planar::Point;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let map: AntennaMap = input.trim().parse()?;

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn part1(map: &AntennaMap) -> usize {
    map.simple_antinodes().len()
}

fn part2(map: &AntennaMap) -> usize {
    map.antinodes().len()
}

#[derive(Debug, Clone)]
struct AntennaMap {
    width: usize,
    height: usize,
    antennas: [Vec<Point>; 62],
}

impl AntennaMap {
    fn simple_antinodes(&self) -> FxHashSet<Point> {
        let mut antinodes: FxHashSet<Point> = FxHashSet::default();
        for points in &self.antennas {
            for (i, &a) in points.iter().enumerate() {
                for &b in &points[i + 1..] {
                    let diff = a - b;
                    for point in [a + diff, b - diff] {
                        if self.in_bounds(point) {
                            antinodes.insert(point);
                        }
                    }
                }
            }
        }

        antinodes
    }

    fn antinodes(&self) -> FxHashSet<Point> {
        let mut antinodes: FxHashSet<Point> = FxHashSet::default();
        for points in &self.antennas {
            for (i, &a) in points.iter().enumerate() {
                for &b in &points[i + 1..] {
                    for (mut point, diff) in [(a, a - b), (b, b - a)] {
                        while self.in_bounds(point) {
                            antinodes.insert(point);
                            point += diff;
                        }
                    }
                }
            }
        }

        antinodes
    }

    fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width as i64 && point.y >= 0 && point.y < self.height as i64
    }
}

impl std::str::FromStr for AntennaMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut antennas: [Vec<Point>; 62] = [const { vec![] }; 62];
        let height = s.lines().count();
        let width = s.lines().next().map(|line| line.len()).unwrap_or(0);
        for (y, line) in s.lines().rev().enumerate() {
            if line.len() != width {
                bail!("Non square map detected");
            }
            for (x, c) in line.chars().enumerate() {
                let index = match c {
                    '.' => continue,
                    '0'..='9' => c as usize - '0' as usize,
                    'a'..='z' => c as usize - 'a' as usize + 10,
                    'A'..='Z' => c as usize - 'A' as usize + 36,
                    _ => bail!("Unrecognized character {} at {}, {}", c, x, y),
                };
                antennas[index].push(Point::new(x as i64, y as i64));
            }
        }
        Ok(AntennaMap {
            width,
            height,
            antennas,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        ............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............";

    #[test]
    fn simple_antinodes() {
        let map: AntennaMap = TEST_DATA.parse().unwrap();
        let result = map.simple_antinodes();
        let expected: FxHashSet<_> = [
            Point { x: 6, y: 11 },
            Point { x: 11, y: 11 },
            Point { x: 3, y: 10 },
            Point { x: 4, y: 9 },
            Point { x: 10, y: 9 },
            Point { x: 2, y: 8 },
            Point { x: 9, y: 7 },
            Point { x: 1, y: 6 },
            Point { x: 6, y: 6 },
            Point { x: 3, y: 5 },
            Point { x: 0, y: 4 },
            Point { x: 7, y: 4 },
            Point { x: 10, y: 1 },
            Point { x: 10, y: 0 },
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn antinodes() {
        let map: AntennaMap = TEST_DATA.parse().unwrap();
        let result = map.antinodes();
        assert_eq!(result.len(), 34);
    }
}
