use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};

use aoc::planar::{Direction, TileMap, Turn};
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let (map_part, instructions_part) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Failed to parse input"))?;
    let map: TileMap<Tile> = map_part.parse()?;
    let instructions = parse_instructions(instructions_part.trim())?;

    println!("Part 1: {}", part1(&map, &instructions)?);
    println!("Part 2: {}", part2(&map, &instructions)?);

    Ok(())
}

fn part1(map: &TileMap<Tile>, instructions: &[Instruction]) -> Result<usize> {
    let monkey_map = MonkeyMap { map };
    monkey_map.find_password(instructions)
}

fn part2(map: &TileMap<Tile>, instructions: &[Instruction]) -> Result<usize> {
    let monkey_cube = MonkeyCube::new(map)?;
    monkey_cube.find_password(instructions)
}

trait WrappingMap {
    fn advance_once(&self, position: Position) -> Option<Position>;

    fn map(&self) -> &TileMap<Tile>;

    fn find_password(&self, instructions: &[Instruction]) -> Result<usize> {
        let mut position = self
            .initial_position()
            .ok_or_else(|| anyhow!("Failed to find starting point"))?;
        for &instruction in instructions {
            position = self.follow_instruction(position, instruction);
        }
        let facing_value = match position.facing {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        };
        Ok(1000 * (position.row + 1) + 4 * (position.column + 1) + facing_value)
    }

    fn initial_position(&self) -> Option<Position> {
        let column = (0..self.map().width())
            .map(|x| self.map().get(x, 0))
            .position(|value| matches!(value, Some(Tile::Open)))?;
        Some(Position {
            row: 0,
            column,
            facing: Direction::East,
        })
    }

    fn follow_instruction(&self, position: Position, instruction: Instruction) -> Position {
        match instruction {
            Instruction::Advance(n) => {
                std::iter::successors(Some(position), |&p| self.advance_once(p))
                    .take(n + 1)
                    .last()
                    .expect("Initial position always exists")
            }
            Instruction::Turn(t) => position.turn(t),
        }
    }
}

#[derive(Debug, Clone)]
struct MonkeyMap<'a> {
    map: &'a TileMap<Tile>,
}

impl<'a> WrappingMap for MonkeyMap<'a> {
    fn advance_once(&self, mut position: Position) -> Option<Position> {
        loop {
            match position.facing {
                Direction::North => {
                    position.row = (position.row + self.map.height() - 1) % self.map.height()
                }
                Direction::East => position.column = (position.column + 1) % self.map.width(),
                Direction::South => position.row = (position.row + 1) % self.map.height(),
                Direction::West => {
                    position.column = (position.column + self.map.width() - 1) % self.map.width()
                }
            }
            match self.map.get(position.column, position.row) {
                Some(Tile::Open) => return Some(position),
                Some(Tile::Wall) => return None,
                Some(Tile::Void) | None => {}
            }
        }
    }

    fn map(&self) -> &TileMap<Tile> {
        &self.map
    }
}

#[derive(Debug, Clone)]
struct MonkeyCube<'a> {
    map: &'a TileMap<Tile>,
    edge_connections: HashMap<Position, Position>,
}

impl<'a> WrappingMap for MonkeyCube<'a> {
    fn advance_once(&self, position: Position) -> Option<Position> {
        let position = match self.edge_connections.get(&position) {
            Some(position) => *position,
            None => position.advance()?,
        };
        match self.map.get(position.column, position.row) {
            Some(Tile::Open) => Some(position),
            _ => None,
        }
    }

    fn map(&self) -> &TileMap<Tile> {
        &self.map
    }
}

impl<'a> MonkeyCube<'a> {
    fn new(map: &'a TileMap<Tile>) -> Result<Self> {
        let corners = find_convex_corners(map)?;

        // For each corner, move outward matching up pairs as long as one side or the other is a
        // flat edge.
        let mut edge_connections = HashMap::new();
        for (p1, p2) in &corners {
            let (mut p1, mut p2) = (*p1, *p2);
            loop {
                let from = p1.turn(Turn::CounterClockwise);
                let to = p2.turn(Turn::CounterClockwise);
                edge_connections.insert(from, to);
                edge_connections.insert(to.reverse(), from.reverse());

                let old_facings = (p1.facing, p2.facing);
                p1 = p1.advance_along_edge(map, Turn::Clockwise)?;
                p2 = p2.advance_along_edge(map, Turn::CounterClockwise)?;
                if p1.facing != old_facings.0 && p2.facing != old_facings.1 {
                    break;
                }
            }
        }

        Ok(Self {
            map,
            edge_connections,
        })
    }
}

/// Finds all the convex corners we can use to zip up the cube.
///
/// For convenience, corners are represented as pairs of points (p1, p2) with p1 one space
/// clockwise from the corner, and p2 one space counterclockwise.
fn find_convex_corners(map: &TileMap<Tile>) -> Result<Vec<(Position, Position)>> {
    let mut corners = vec![];
    let mut starting_position: Position = Position {
        row: 0,
        column: 0,
        facing: Direction::East,
    };
    // First find the edge of the map
    while !starting_position.is_on_map(map) {
        starting_position = starting_position
            .advance()
            .ok_or_else(|| anyhow!("Failed to find map edge"))?;
    }
    // Then walk around the map identifying convex corners
    let mut position = starting_position;
    loop {
        if let Some(p) = position.advance() {
            if p.can_turn_on_map(map, Turn::CounterClockwise) {
                let p1 = position.advance_along_edge(map, Turn::Clockwise)?;
                let p2 = position.reverse();
                corners.push((p1, p2));
            }
        }
        position = position.advance_along_edge(map, Turn::Clockwise)?;
        if position.row == starting_position.row && position.column == starting_position.column {
            break;
        }
    }
    Ok(corners)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    column: usize,
    facing: Direction,
}

impl Position {
    fn advance(&self) -> Option<Self> {
        let mut position = *self;
        match self.facing {
            Direction::East => position.column = position.column.checked_add(1)?,
            Direction::South => position.row = position.row.checked_add(1)?,
            Direction::West => position.column = position.column.checked_sub(1)?,
            Direction::North => position.row = position.row.checked_sub(1)?,
        }
        Some(position)
    }

    fn turn(&self, turn: Turn) -> Self {
        Self {
            row: self.row,
            column: self.column,
            facing: self.facing.turn(turn),
        }
    }

    fn reverse(&self) -> Self {
        Self {
            row: self.row,
            column: self.column,
            facing: self.facing.reverse(),
        }
    }

    fn is_on_map(&self, map: &TileMap<Tile>) -> bool {
        match map.get(self.column, self.row) {
            Some(Tile::Open | Tile::Wall) => true,
            _ => false,
        }
    }

    fn can_turn_on_map(&self, map: &TileMap<Tile>, turn: Turn) -> bool {
        match self.turn(turn).advance() {
            Some(p) => p.is_on_map(map),
            None => false,
        }
    }

    /// Advance one space around the map, cutting inner corners
    fn advance_along_edge(&self, map: &TileMap<Tile>, turn: Turn) -> Result<Position> {
        // If we can advance, do so.
        if let Some(mut new_position) = self.advance() {
            if new_position.is_on_map(map) {
                // If we've moved into a convex corner, advance again to cut the corner
                if new_position.can_turn_on_map(map, turn.reverse()) {
                    new_position = new_position
                        .turn(turn.reverse())
                        .advance()
                        .expect("Turn + advance must work");
                }
                return Ok(new_position);
            }
        }
        // Otherwise try turning inwards
        if self.can_turn_on_map(map, turn) {
            Ok(self.turn(turn))
        } else {
            bail!("Failed to walk around map edge");
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Advance(usize),
    Turn(Turn),
}

#[derive(Debug, Clone)]
enum Tile {
    Open,
    Wall,
    Void,
}

fn parse_instructions(s: &str) -> Result<Vec<Instruction>> {
    let mut instructions = vec![];
    let mut bytes = s.as_bytes();
    while let Some(b) = bytes.first() {
        match b {
            b'R' => {
                bytes = &bytes[1..];
                instructions.push(Instruction::Turn(Turn::Clockwise));
            }
            b'L' => {
                bytes = &bytes[1..];
                instructions.push(Instruction::Turn(Turn::CounterClockwise));
            }
            b'0'..=b'9' => {
                let num_part;
                let i = bytes
                    .iter()
                    .position(|b| !b.is_ascii_digit())
                    .unwrap_or(bytes.len());
                (num_part, bytes) = bytes.split_at(i);
                let num = std::str::from_utf8(num_part)?.parse()?;
                instructions.push(Instruction::Advance(num));
            }
            _ => bail!("Unexpected character '{}' in instructions", *b as char),
        }
    }
    Ok(instructions)
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Wall),
            ' ' => Ok(Tile::Void),
            _ => bail!("Unexpected character {}", c),
        }
    }
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> Self {
        match tile {
            Tile::Open => '.',
            Tile::Wall => '#',
            Tile::Void => ' ',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_MAP: &str = "\
        \x20       ...#    \n\
        \x20       .#..    \n\
        \x20       #...    \n\
        \x20       ....    \n\
           ...#.......#    \n\
           ........#...    \n\
           ..#....#....    \n\
           ..........#.    \n\
        \x20       ...#....\n\
        \x20       .....#..\n\
        \x20       .#......\n\
        \x20       ......#.";

    static TEST_INSTRUCTIONS: &str = "10R5L5R10L4R5L5";

    #[test]
    fn wrapping() {
        let map = &TEST_MAP.parse().unwrap();
        let monkey_map = MonkeyMap { map };
        let instructions = parse_instructions(TEST_INSTRUCTIONS).unwrap();
        let result = monkey_map.find_password(&instructions).unwrap();

        assert_eq!(result, 6032);
    }

    #[test]
    fn cube() {
        let map = &TEST_MAP.parse().unwrap();
        let cube = MonkeyCube::new(map).unwrap();
        let instructions = parse_instructions(TEST_INSTRUCTIONS).unwrap();
        let result = cube.find_password(&instructions).unwrap();

        assert_eq!(result, 5031);
    }
}
