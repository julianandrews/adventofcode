use std::str::FromStr;

use aoc::aoc_error::AOCError;
use aoc::planar::{Direction, Point};
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let instructions = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
    Ok(())
}

fn part1(instructions: &[Instruction]) -> u64 {
    let mut ship = SimpleShip::new();
    ship.follow_instructions(instructions);

    ship.location.manhattan_norm()
}

fn part2(instructions: &[Instruction]) -> u64 {
    let mut ship = WaypointShip::new();
    ship.follow_instructions(instructions);

    ship.location.manhattan_norm()
}

struct SimpleShip {
    location: Point,
    direction: Direction,
}

impl SimpleShip {
    fn new() -> Self {
        Self {
            location: Point { x: 0, y: 0 },
            direction: Direction::East,
        }
    }

    fn follow_instructions(&mut self, instructions: &[Instruction]) {
        use Instruction::*;
        for &instruction in instructions {
            match instruction {
                Move(direction, distance) => self.location.advance(direction, distance),
                Advance(distance) => self.location.advance(self.direction, distance),
                RightTurn(times) => {
                    (0..times).for_each(|_| self.direction = self.direction.right_turn())
                }
            }
        }
    }
}

struct WaypointShip {
    location: Point,
    waypoint: Point,
}

impl WaypointShip {
    fn new() -> Self {
        Self {
            location: Point { x: 0, y: 0 },
            waypoint: Point { x: 10, y: 1 },
        }
    }

    fn follow_instructions(&mut self, instructions: &[Instruction]) {
        use Instruction::*;
        for &instruction in instructions {
            match instruction {
                Move(direction, distance) => self.waypoint.advance(direction, distance),
                Advance(times) => (0..times).for_each(|_| self.location += self.waypoint),
                RightTurn(times) => (0..times).for_each(|_| {
                    let old_x = self.waypoint.x;
                    self.waypoint.x = self.waypoint.y;
                    self.waypoint.y = -old_x;
                }),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Move(Direction, u64),
    Advance(u64),
    RightTurn(u64),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let parse_angle = |angle: u64| -> Result<u64> {
            if angle % 90 != 0 {
                return Err(AOCError::new("Invalid angle").into());
            }
            Ok((angle / 90) % 4)
        };

        if s.len() < 2 || !s.is_ascii() {
            return Err(AOCError::new("Invalid instruction").into());
        }
        let value = s[1..].parse().map_err(|_| AOCError::new("Invalid value"))?;
        match s.chars().next().unwrap() {
            'N' => Ok(Self::Move(Direction::North, value)),
            'S' => Ok(Self::Move(Direction::South, value)),
            'E' => Ok(Self::Move(Direction::East, value)),
            'W' => Ok(Self::Move(Direction::West, value)),
            'F' => Ok(Self::Advance(value)),
            'R' => Ok(Self::RightTurn(parse_angle(value)?)),
            'L' => Ok(Self::RightTurn(4 - parse_angle(value)?)),
            _ => Err(AOCError::new("Unrecognized instruction").into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "F10\
                                     \nN3\
                                     \nF7\
                                     \nR90\
                                     \nF11";

    #[test]
    fn follow_simple_instructions() {
        let instructions = parse_fields(TEST_INPUT, '\n').unwrap();

        let mut ship = SimpleShip::new();
        ship.follow_instructions(&instructions);
        assert_eq!(ship.location.manhattan_norm(), 25);
    }

    #[test]
    fn follow_waypoint_instructions() {
        let instructions = parse_fields(TEST_INPUT, '\n').unwrap();

        let mut ship = WaypointShip::new();
        ship.follow_instructions(&instructions);
        assert_eq!(ship.location.manhattan_norm(), 286);
    }
}
