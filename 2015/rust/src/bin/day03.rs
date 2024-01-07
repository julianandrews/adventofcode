use anyhow::Result;
use rustc_hash::FxHashSet;

use aoc::planar::Direction;
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let directions: Vec<Direction> = parsing::parse_directions(input.trim())?;

    println!("Part 1: {}", part1(&directions));
    println!("Part 2: {}", part2(&directions));

    Ok(())
}

fn part1(directions: &[Direction]) -> usize {
    houses_visited(directions, 1)
}

fn part2(directions: &[Direction]) -> usize {
    houses_visited(directions, 2)
}

fn houses_visited(directions: &[Direction], santa_count: usize) -> usize {
    if santa_count == 0 {
        return 0;
    }
    let mut visited = FxHashSet::default();
    let mut santas = vec![Santa::default(); santa_count];
    visited.insert(Santa::default());

    for (i, direction) in directions.iter().enumerate() {
        let santa = &mut santas[i % santa_count];
        santa.step(direction);
        visited.insert(*santa);
    }

    visited.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Santa {
    pub x: i64,
    pub y: i64,
}

impl Santa {
    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }
}

mod parsing {
    use super::Direction;

    use anyhow::{anyhow, Result};

    pub fn parse_directions(s: &str) -> Result<Vec<Direction>> {
        s.chars()
            .map(|c| match c {
                '^' => Ok(Direction::North),
                '>' => Ok(Direction::East),
                'v' => Ok(Direction::South),
                '<' => Ok(Direction::West),
                c => Err(anyhow!("Unrecognized direction {}", c)),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{houses_visited, parsing::parse_directions};

    #[test]
    fn houses_visited_1() {
        let directions: Vec<_> = parse_directions(">").unwrap();

        assert_eq!(houses_visited(&directions, 1), 2);
    }

    #[test]
    fn houses_visited_2() {
        let directions: Vec<_> = parse_directions("^>v<").unwrap();

        assert_eq!(houses_visited(&directions, 1), 4);
    }

    #[test]
    fn houses_visited_3() {
        let directions: Vec<_> = parse_directions("^v^v^v^v^v").unwrap();

        assert_eq!(houses_visited(&directions, 1), 2);
    }

    #[test]
    fn robo_santa_1() {
        let directions: Vec<_> = parse_directions("^v").unwrap();

        assert_eq!(houses_visited(&directions, 2), 3);
    }

    #[test]
    fn robo_santa_2() {
        let directions: Vec<_> = parse_directions("^>v<").unwrap();

        assert_eq!(houses_visited(&directions, 2), 3);
    }

    #[test]
    fn robo_santa_3() {
        let directions: Vec<_> = parse_directions("^v^v^v^v^v").unwrap();

        assert_eq!(houses_visited(&directions, 2), 11);
    }
}
