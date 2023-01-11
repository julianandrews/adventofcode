use rustc_hash::{FxHashMap, FxHashSet};

use anyhow::{bail, Result};

use aoc::planar::Point;
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: SeedlingMap = input.trim().parse()?;

    println!("Part 1: {}", part1(map.clone()));
    println!("Part 2: {}", part2(map));

    Ok(())
}

fn part1(mut map: SeedlingMap) -> usize {
    for i in 0..10 {
        map.step(i);
    }
    map.empty_ground()
}

fn part2(mut map: SeedlingMap) -> usize {
    map.run()
}

#[derive(Debug, Clone)]
struct SeedlingMap {
    elves: FxHashSet<Point>,
}

impl SeedlingMap {
    fn run(&mut self) -> usize {
        for i in 0.. {
            if !self.step(i) {
                return i + 1;
            }
        }
        unreachable!();
    }

    fn step(&mut self, i: usize) -> bool {
        // HashMap entries are (to, from) pairs
        let mut proposals: FxHashMap<Point, Point> = FxHashMap::default();
        for &elf in &self.elves {
            let destination = self.proposal(elf, i);
            if let Some(other_elf) = proposals.get(&destination).cloned() {
                proposals.remove(&destination);
                proposals.insert(elf, elf);
                proposals.insert(other_elf, other_elf);
            } else {
                proposals.insert(destination, elf);
            }
        }

        self.elves = proposals.keys().cloned().collect();
        proposals.iter().any(|(to, from)| to != from)
    }

    fn proposal(&self, point: Point, i: usize) -> Point {
        static OFFSETS: [Point; 8] = [
            Point::new(-1, -1),
            Point::new(0, -1),
            Point::new(1, -1),
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(-1, 1),
            Point::new(-1, 0),
        ];
        static DIRECTIONS: [(Point, u8); 4] = [
            (Point::new(0, -1), 0b00000111),
            (Point::new(0, 1), 0b01110000),
            (Point::new(-1, 0), 0b11000001),
            (Point::new(1, 0), 0b00011100),
        ];
        let mut neighbor_mask: u8 = 0;
        for (j, &offset) in OFFSETS.iter().enumerate() {
            if self.elves.contains(&(point + offset)) {
                neighbor_mask |= 1 << j;
            }
        }
        if neighbor_mask == 0b00000000 {
            return point;
        }
        for j in 0..4 {
            let (dir, mask) = DIRECTIONS[(i + j) % 4];
            if neighbor_mask & mask == 0 {
                return point + dir;
            }
        }
        point
    }

    fn empty_ground(&self) -> usize {
        let min_x = self.elves.iter().map(|p| p.x).min().unwrap_or(0);
        let max_x = self.elves.iter().map(|p| p.x).max().unwrap_or(0);
        let min_y = self.elves.iter().map(|p| p.y).min().unwrap_or(0);
        let max_y = self.elves.iter().map(|p| p.y).max().unwrap_or(0);
        ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - self.elves.len()
    }
}

impl std::str::FromStr for SeedlingMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elves = FxHashSet::default();

        for (y, line) in s.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                match b {
                    b'#' => {
                        elves.insert(Point::new(x as i64, y as i64));
                    }
                    b'.' => {}
                    _ => bail!("Unexpected character {}", b as char),
                }
            }
        }

        Ok(Self { elves })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SMALL_EXAMPLE: &str = "\
        .....\n\
        ..##.\n\
        ..#..\n\
        .....\n\
        ..##.\n\
        .....";

    static LARGER_EXAMPLE: &str = "\
        ..............\n\
        ..............\n\
        .......#......\n\
        .....###.#....\n\
        ...#...#.#....\n\
        ....#...##....\n\
        ...#.###......\n\
        ...##.#.##....\n\
        ....#..#......\n\
        ..............\n\
        ..............\n\
        ..............";

    #[test]
    fn simple_test() {
        let mut map: SeedlingMap = SMALL_EXAMPLE.parse().unwrap();
        map.run();
        let expected: FxHashSet<_> = [(2, 0), (4, 1), (0, 2), (4, 3), (2, 5)]
            .iter()
            .map(|(x, y)| Point::new(*x, *y))
            .collect();

        assert_eq!(map.elves, expected);
    }

    #[test]
    fn larger_test() {
        let mut map: SeedlingMap = LARGER_EXAMPLE.parse().unwrap();
        for i in 0..10 {
            map.step(i);
        }
        let result = map.empty_ground();

        assert_eq!(result, 110);
    }

    #[test]
    fn full_run() {
        let mut map: SeedlingMap = LARGER_EXAMPLE.parse().unwrap();
        let steps = map.run();

        assert_eq!(steps, 20);
    }
}
