extern crate aoc;

use aoc::Result;
use std::collections::HashSet;
use std::io;
use std::io::{Read, Write};

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_char(c: char) -> Result<Direction> {
        match c {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err("Unrecognized direction".into()),
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let directions: Vec<Direction> = input
        .trim()
        .chars()
        .map(Direction::from_char)
        .collect::<std::result::Result<_, _>>()?;

    writeln!(io::stdout(), "{}", part1(&directions)?);
    writeln!(io::stdout(), "{}", part2(&directions)?);

    Ok(())
}

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

    fn new() -> Santa {
        Santa { x: 0, y: 0 }
    }
}

fn part1(directions: &[Direction]) -> Result<usize> {
    let mut seen_houses = HashSet::new();
    seen_houses.insert((0, 0));
    let mut santa = Santa::new();

    for direction in directions {
        santa.step(direction);
        seen_houses.insert((santa.x, santa.y));
    }

    Ok(seen_houses.len())
}

fn part2(directions: &[Direction]) -> Result<usize> {
    let mut seen_houses = HashSet::new();
    seen_houses.insert((0, 0));
    let mut santas = [Santa::new(), Santa::new()];

    for (i, direction) in directions.iter().enumerate() {
        let santa = &mut santas[i % santas.len()];
        santa.step(direction);
        seen_houses.insert((santa.x, santa.y));
    }

    Ok(seen_houses.len())
}
