use anyhow::{bail, Result};
use rustc_hash::FxHashMap;

use aoc::planar::Direction;
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: RockMap = input.parse()?;

    println!("Part 1: {}", test_load(&map));
    println!("Part 2: {}", final_load(map));

    Ok(())
}

#[derive(Debug, Clone)]
struct RockMap {
    width: u32,
    cubes: Vec<u128>,
    spheres: Vec<u128>,
}

fn test_load(map: &RockMap) -> u32 {
    let mut map = map.clone();
    map.tilt(Direction::North);
    map.total_load()
}

fn final_load(mut map: RockMap) -> u32 {
    map.run_cycles(1_000_000_000);
    map.total_load()
}

impl RockMap {
    fn run_cycles(&mut self, cycles: usize) {
        // TODO: Try using a bloom filter to speed this up.
        let mut seen = FxHashMap::default();
        for t in 0..cycles {
            let last_seen = *seen.entry(self.spheres.clone()).or_insert(t);
            if last_seen != t {
                let cycle_length = t - last_seen;
                let offset = (cycles - t) % cycle_length;
                for _ in 0..offset {
                    self.cycle();
                }
                break;
            }
            self.cycle();
        }
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                let mut changed = true;
                while changed {
                    changed = false;
                    for i in 0..self.spheres.len().saturating_sub(1) {
                        let moved = self.spheres[i + 1] & !(self.spheres[i] | self.cubes[i]);
                        if moved != 0 {
                            self.spheres[i] |= moved;
                            self.spheres[i + 1] &= !moved;
                            changed = true;
                        }
                    }
                }
            }
            Direction::West => {
                for (spheres, cubes) in self.spheres.iter_mut().zip(&self.cubes) {
                    let mut right = 0;
                    while right < self.width {
                        let left = ((cubes >> right).trailing_zeros() + right).min(self.width);
                        let mask: u128 = ((1 << (left - right)) - 1) << right;
                        let sphere_count = (*spheres & mask).count_ones();
                        *spheres &= !mask;
                        *spheres |= ((1 << sphere_count) - 1) << right;
                        right = left + (cubes >> left).trailing_ones();
                    }
                }
            }
            Direction::South => {
                let mut changed = true;
                while changed {
                    changed = false;
                    for i in (1..=self.spheres.len().saturating_sub(1)).rev() {
                        let moved = self.spheres[i - 1] & !(self.spheres[i] | self.cubes[i]);
                        if moved != 0 {
                            self.spheres[i] |= moved;
                            self.spheres[i - 1] &= !moved;
                            changed = true;
                        }
                    }
                }
            }
            Direction::East => {
                for (spheres, cubes) in self.spheres.iter_mut().zip(&self.cubes) {
                    let mut right = 0;
                    while right < self.width {
                        let left = ((cubes >> right).trailing_zeros() + right).min(self.width);
                        let mask: u128 = ((1 << (left - right)) - 1) << right;
                        let sphere_count = (*spheres & mask).count_ones();
                        *spheres &= !mask;
                        *spheres |= ((1 << sphere_count) - 1) << left.saturating_sub(sphere_count);
                        right = left + (cubes >> left).trailing_ones();
                    }
                }
            }
        }
    }

    fn total_load(&self) -> u32 {
        self.spheres
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| (i as u32 + 1) * row.count_ones())
            .sum()
    }
}

impl std::str::FromStr for RockMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(|line| line.len()).unwrap_or(0);
        if width > 128 {
            bail!("Map too large");
        }
        let cubes = bytes_as_bitvecs(s, b'#');
        let spheres = bytes_as_bitvecs(s, b'O');
        Ok(RockMap {
            width: width as u32,
            cubes,
            spheres,
        })
    }
}

fn bytes_as_bitvecs(s: &str, value: u8) -> Vec<u128> {
    s.lines()
        .map(|line| {
            line.bytes()
                .rev()
                .fold(0, |acc, b| (acc << 1) | (b == value) as u128)
        })
        .collect()
}

impl std::fmt::Display for RockMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (cubes, spheres) in self.cubes.iter().zip(&self.spheres) {
            for x in 0..self.width {
                if (cubes >> x) & 1 != 0 {
                    write!(f, "#")?;
                } else if (spheres >> x) & 1 != 0 {
                    write!(f, "O")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static MAPS: [&str; 4] = [
        "O....#....\n\
        O.OO#....#\n\
        .....##...\n\
        OO.#O....O\n\
        .O.....O#.\n\
        O.#..O.#.#\n\
        ..O..#O..O\n\
        .......O..\n\
        #....###..\n\
        #OO..#....",
        ".....#....\n\
        ....#...O#\n\
        ...OO##...\n\
        .OO#......\n\
        .....OOO#.\n\
        .O#...O#.#\n\
        ....O#....\n\
        ......OOOO\n\
        #...O###..\n\
        #..OO#....",
        ".....#....\n\
        ....#...O#\n\
        .....##...\n\
        ..O#......\n\
        .....OOO#.\n\
        .O#...O#.#\n\
        ....O#...O\n\
        .......OOO\n\
        #..OO###..\n\
        #.OOO#...O",
        ".....#....\n\
        ....#...O#\n\
        .....##...\n\
        ..O#......\n\
        .....OOO#.\n\
        .O#...O#.#\n\
        ....O#...O\n\
        .......OOO\n\
        #...O###.O\n\
        #.OOO#...O",
    ];

    #[test]
    fn load_test() {
        let map: RockMap = MAPS[0].parse().unwrap();
        assert_eq!(test_load(&map), 136);
    }

    #[test]
    fn cycles_1() {
        let mut map: RockMap = MAPS[0].parse().unwrap();
        map.cycle();
        let expected: RockMap = MAPS[1].parse().unwrap();
        assert_eq!(map.spheres, expected.spheres);
    }

    #[test]
    fn cycles_2() {
        let mut map: RockMap = MAPS[1].parse().unwrap();
        map.cycle();
        let expected: RockMap = MAPS[2].parse().unwrap();
        assert_eq!(map.spheres, expected.spheres);
    }

    #[test]
    fn cycles_3() {
        let mut map: RockMap = MAPS[2].parse().unwrap();
        map.cycle();
        let expected: RockMap = MAPS[3].parse().unwrap();
        assert_eq!(map.spheres, expected.spheres);
    }

    #[test]
    fn long_load_test() {
        let map: RockMap = MAPS[0].parse().unwrap();
        assert_eq!(final_load(map), 64);
    }
}
