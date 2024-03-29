use aoc::point::Point2D;
use aoc::utils::parse_fields;

use anyhow::{anyhow, Result};
use rustc_hash::{FxHashMap, FxHashSet};

type Point = Point2D<i64>;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let mut wires: Vec<Wire> = parse_fields(input.trim(), '\n')?;
    if wires.len() != 2 {
        Err(anyhow!("Invalid input"))?;
    }
    let wire_1 = wires.pop().unwrap();
    let wire_2 = wires.pop().unwrap();

    println!("Part 1: {}", part1(&wire_1, &wire_2)?);
    println!("Part 2: {}", part2(&wire_1, &wire_2)?);
    Ok(())
}

fn part1(wire_1: &Wire, wire_2: &Wire) -> Result<i64> {
    let wire_1_points = wire_1.points().collect::<FxHashSet<_>>();
    let wire_2_points = wire_2.points().collect::<FxHashSet<_>>();
    let intersections = wire_1_points.intersection(&wire_2_points);
    intersections
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .ok_or(anyhow!("No intersections found"))
}

fn part2(wire_1: &Wire, wire_2: &Wire) -> Result<u64> {
    let wire_1_points = wire_1.points().collect::<FxHashSet<_>>();
    let wire_2_points = wire_2.points().collect::<FxHashSet<_>>();
    let intersections = wire_1_points.intersection(&wire_2_points);
    intersections
        .map(|p| wire_1.signal_distance(p) + wire_2.signal_distance(p))
        .min()
        .ok_or(anyhow!("No intersections found"))
}

struct Wire {
    signal_distances: FxHashMap<Point, u64>,
}

impl Wire {
    fn points(&self) -> impl Iterator<Item = &Point> {
        self.signal_distances.keys()
    }

    fn signal_distance(&self, point: &Point) -> u64 {
        self.signal_distances[point]
    }
}

mod parsing {
    use super::{Point, Wire};

    use anyhow::{anyhow, Result};
    use rustc_hash::FxHashMap;

    use aoc::direction::Direction;

    impl std::str::FromStr for Wire {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self> {
            fn direction_from_char(c: char) -> Result<Direction> {
                match c {
                    'U' => Ok(Direction::North),
                    'R' => Ok(Direction::East),
                    'D' => Ok(Direction::South),
                    'L' => Ok(Direction::West),
                    _ => Err(anyhow!("Unexpected character: {}", c)),
                }
            }

            let mut signal_distances: FxHashMap<Point, u64> = FxHashMap::default();
            let mut signal_distance = 0;
            let mut pos = Point { x: 0, y: 0 };
            for instruction in s.split(',') {
                let direction = direction_from_char(
                    instruction
                        .chars()
                        .nth(0)
                        .ok_or(anyhow!("No direction found"))?,
                )?;
                let distance: u64 = instruction[1..].parse()?;
                for _ in 0..distance {
                    signal_distance += 1;
                    pos = direction.next_position(pos);
                    signal_distances.entry(pos).or_insert(signal_distance);
                }
            }

            Ok(Wire { signal_distances })
        }
    }
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
