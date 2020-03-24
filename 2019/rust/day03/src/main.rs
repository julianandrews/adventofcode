extern crate aoc;

use aoc::aoc_error::AOCError;
use aoc::direction::Direction;
use aoc::point::Point2D;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Point = Point2D<i64>;

struct Wire {
    signal_distances: HashMap<Point, u64>,
}

impl FromStr for Wire {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        fn direction_from_char(c: char) -> Result<Direction> {
            match c {
                'U' => Ok(Direction::North),
                'R' => Ok(Direction::East),
                'D' => Ok(Direction::South),
                'L' => Ok(Direction::West),
                _ => Err(AOCError::new(&format!("Unexpected character: {}", c)))?,
            }
        }

        let mut signal_distances: HashMap<Point, u64> = HashMap::new();
        let mut signal_distance = 0;
        let mut pos = Point { x: 0, y: 0 };
        for instruction in s.split(",") {
            let direction = direction_from_char(
                instruction
                    .chars()
                    .nth(0)
                    .ok_or(AOCError::new("No direction found"))?,
            )?;
            let distance: u64 = instruction[1..].parse()?;
            for _ in 0..distance {
                signal_distance += 1;
                pos = direction.next_position(pos);
                signal_distances
                    .entry(pos.clone())
                    .or_insert(signal_distance);
            }
        }

        Ok(Wire {
            signal_distances: signal_distances,
        })
    }
}

impl Wire {
    fn points(&self) -> impl Iterator<Item = &Point> {
        self.signal_distances.keys()
    }

    fn signal_distance(&self, point: &Point) -> u64 {
        self.signal_distances[point]
    }
}

fn part1(wire_1: &Wire, wire_2: &Wire) -> Result<i64> {
    let wire_1_points = wire_1.points().collect::<HashSet<_>>();
    let wire_2_points = wire_2.points().collect::<HashSet<_>>();
    let intersections = wire_1_points.intersection(&wire_2_points);
    Ok(intersections
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .ok_or(AOCError::new("No intersections found"))?)
}

fn part2(wire_1: &Wire, wire_2: &Wire) -> Result<u64> {
    let wire_1_points = wire_1.points().collect::<HashSet<_>>();
    let wire_2_points = wire_2.points().collect::<HashSet<_>>();
    let intersections = wire_1_points.intersection(&wire_2_points);
    Ok(intersections
        .map(|p| wire_1.signal_distance(p) + wire_2.signal_distance(p))
        .min()
        .ok_or(AOCError::new("No intersections found"))?)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut wires = input
        .lines()
        .map(|line| line.parse::<Wire>())
        .collect::<Result<Vec<Wire>>>()?;
    if wires.len() != 2 {
        Err(AOCError::new("Invalid input"))?;
    }
    let wire_1 = wires.pop().unwrap();
    let wire_2 = wires.pop().unwrap();

    println!("Part 1: {}", part1(&wire_1, &wire_2)?);
    println!("Part 2: {}", part2(&wire_1, &wire_2)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let wire_1 = "R8,U5,L5,D3".parse().unwrap();
        let wire_2 = "U7,R6,D4,L4".parse().unwrap();
        let part1_result = part1(&wire_1, &wire_2);
        assert!(part1_result.is_ok());
        assert_eq!(part1_result.unwrap(), 6);

        let part2_result = part2(&wire_1, &wire_2);
        assert!(part2_result.is_ok());
        assert_eq!(part2_result.unwrap(), 30);
    }

    #[test]
    fn test_case_2() {
        let wire_1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".parse().unwrap();
        let wire_2 = "U62,R66,U55,R34,D71,R55,D58,R83".parse().unwrap();
        let part1_result = part1(&wire_1, &wire_2);
        assert!(part1_result.is_ok());
        assert_eq!(part1_result.unwrap(), 159);

        let part2_result = part2(&wire_1, &wire_2);
        assert!(part2_result.is_ok());
        assert_eq!(part2_result.unwrap(), 610);
    }

    #[test]
    fn test_case_3() {
        let wire_1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
            .parse()
            .unwrap();
        let wire_2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".parse().unwrap();
        let part1_result = part1(&wire_1, &wire_2);
        assert!(part1_result.is_ok());
        assert_eq!(part1_result.unwrap(), 135);

        let part2_result = part2(&wire_1, &wire_2);
        assert!(part2_result.is_ok());
        assert_eq!(part2_result.unwrap(), 410);
    }
}
