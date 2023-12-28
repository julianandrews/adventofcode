use std::collections::VecDeque;

use anyhow::{bail, Result};

use aoc::planar::{Direction, TileMap};
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: GardenMap = input.trim().parse()?;

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map)?);

    Ok(())
}

fn part1(map: &GardenMap) -> usize {
    map.reachable::<VecMap>(64)
}

fn part2(map: &GardenMap) -> Result<usize> {
    map.reachable_quadratic_extrapolation(26501365)
}

#[derive(Debug, Clone)]
struct GardenMap(TileMap<GardenTile>);

impl GardenMap {
    fn reachable<V: VisitedMap>(&self, steps: usize) -> usize {
        let mut queue = VecDeque::new();
        queue.push_front((self.start(), 0));
        let mut visited = V::new();
        while let Some((position, time)) = queue.pop_back() {
            if visited.insert(position, time) && time < steps {
                for neighbor in self.neighbors(position) {
                    queue.push_front((neighbor, time + 1));
                }
            }
        }
        visited.count(steps % 2 == 0)
    }

    fn reachable_quadratic_extrapolation(&self, steps: usize) -> Result<usize> {
        self.check_map_valid_for_extrapolation()?;
        let size = self.0.width();
        if steps % size != size / 2 {
            bail!("Step count must end on map boundary.");
        }

        // Calculate quadratic coefficients from values at x = 0, x = 1, and x = 2.
        let y_0: i64 = self.reachable::<VecMap>(size / 2).try_into()?;
        let y_1: i64 = self.reachable::<VecMap>(size + size / 2).try_into()?;
        let y_2: i64 = self.reachable::<VecMap>(2 * size + size / 2).try_into()?;
        let c = y_0;
        let b = (4 * y_1 - y_2 - 3 * y_0) / 2;
        let a = y_1 - b - c;

        let tiles = (steps / size) as i64;
        Ok((a * tiles * tiles + b * tiles + c).try_into()?)
    }

    fn check_map_valid_for_extrapolation(&self) -> Result<()> {
        let size = self.0.width();
        if self.0.height() != size {
            bail!("Map must be square.");
        }
        if size > 170 {
            // VecMap can only handle values up to 512, and we're going up to 3 tiles out.
            bail!("Map too large.");
        }
        for a in 0..size {
            for b in [0, size / 2, size - 1] {
                if self.0.get(a, b) == Some(&GardenTile::Rock) {
                    bail!("Map must have clear left, middle, and right columns.");
                }
                if self.0.get(b, a) == Some(&GardenTile::Rock) {
                    bail!("Map must have clear top, middle, and bottom rows.");
                }
            }
        }
        for x in 0..self.0.width() {
            let y1 = (size / 2).abs_diff(x);
            let y2 = size - x.abs_diff(size / 2);
            for y in [y1, y2] {
                for dy in [-2, -1, 0, 1, 2] {
                    if self.0.get(x, (y as i32 + dy) as usize) == Some(&GardenTile::Rock) {
                        bail!("Map must have clear diamond.");
                    }
                }
            }
        }
        Ok(())
    }

    fn neighbors(&self, position: Position) -> impl Iterator<Item = Position> + '_ {
        Direction::iterator().filter_map(move |d| {
            let new = position.step(d);
            let x = new.x.rem_euclid(self.0.width() as i64) as usize;
            let y = new.y.rem_euclid(self.0.height() as i64) as usize;
            match self.0.get(x, y) {
                Some(GardenTile::Garden) => Some(new),
                _ => None,
            }
        })
    }

    fn start(&self) -> Position {
        Position {
            x: self.0.width() as i64 / 2,
            y: self.0.height() as i64 / 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn step(&self, direction: Direction) -> Position {
        let (x, y) = match direction {
            Direction::North => (self.x, self.y - 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y + 1),
            Direction::West => (self.x - 1, self.y),
        };
        Position { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GardenTile {
    Garden,
    Rock,
}

trait VisitedMap {
    fn new() -> Self;

    fn insert(&mut self, position: Position, time: usize) -> bool;

    fn count(&self, evens: bool) -> usize;
}

/// Sparse array for tracking visited positions.
/// May panic or fail silently if used with coordinates with absolute values over 511.
/// A `HashMap` works fine for a much wider range of inputs, but more than doubles the runtime.
#[derive(Debug, Clone)]
struct VecMap(Vec<u8>);

impl VisitedMap for VecMap {
    fn new() -> Self {
        VecMap(vec![0; 1 << 20])
    }

    fn insert(&mut self, position: Position, time: usize) -> bool {
        // Hash value should be unique for x, y in (-512, 512)
        let hash = (position.x + 512) as usize | ((position.y + 512) as usize) << 10;
        let seen = &mut self.0[hash];
        let is_new = *seen == 0;
        *seen = (time % 2) as u8 + 1;
        is_new
    }

    fn count(&self, evens: bool) -> usize {
        let x = if evens { 1 } else { 2 };
        self.0.iter().filter(|&p| *p == x).count()
    }
}

mod parsing {
    use super::{GardenMap, GardenTile};

    use anyhow::bail;

    impl std::str::FromStr for GardenMap {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let map = s.parse()?;
            if s.bytes().filter(|&b| b == b'S').count() > 1 {
                bail!("Multiple start tiles found");
            }
            if s.as_bytes()[s.len() / 2] != b'S' {
                bail!("Start tile not found in center of map.");
            }

            Ok(GardenMap(map))
        }
    }

    impl TryFrom<char> for GardenTile {
        type Error = anyhow::Error;

        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                '#' => Ok(GardenTile::Rock),
                '.' => Ok(GardenTile::Garden),
                'S' => Ok(GardenTile::Garden),
                _ => bail!("Unrecognized character {} in map", c),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hashmap for tracking visited positions.
    ///
    /// This map is much slower than the sparse array used in the real problems, but necessary
    /// for brute-force testing the larger traversal values.
    type SlowVisitedMap = rustc_hash::FxHashMap<Position, usize>;

    impl VisitedMap for SlowVisitedMap {
        fn new() -> Self {
            rustc_hash::FxHashMap::default()
        }

        fn insert(&mut self, position: Position, time: usize) -> bool {
            self.insert(position, time).is_none()
        }

        fn count(&self, evens: bool) -> usize {
            let x = if evens { 0 } else { 1 };
            self.values().filter(|&t| t % 2 == x).count()
        }
    }

    static TEST_MAP: &str = "\
        ...........\n\
        .....###.#.\n\
        .###.##..#.\n\
        ..#.#...#..\n\
        ....#.#....\n\
        .##..S####.\n\
        .##..#...#.\n\
        .......##..\n\
        .##.#.####.\n\
        .##..##.##.\n\
        ...........";

    #[test]
    fn reachable() {
        let map: GardenMap = TEST_MAP.parse().unwrap();

        assert_eq!(map.reachable::<VecMap>(0), 1);
        assert_eq!(map.reachable::<VecMap>(1), 2);
        assert_eq!(map.reachable::<VecMap>(2), 4);
        assert_eq!(map.reachable::<VecMap>(3), 6);
        assert_eq!(map.reachable::<VecMap>(6), 16);
        assert_eq!(map.reachable::<VecMap>(10), 50);
        assert_eq!(map.reachable::<VecMap>(50), 1594);
        assert_eq!(map.reachable::<VecMap>(100), 6536);
        assert_eq!(map.reachable::<VecMap>(500), 167004);
    }

    #[test]
    #[ignore]
    fn slow_reachable() {
        let map: GardenMap = TEST_MAP.parse().unwrap();

        assert_eq!(map.reachable::<SlowVisitedMap>(1000), 668697);
        assert_eq!(map.reachable::<SlowVisitedMap>(5000), 16733044);
    }
}
