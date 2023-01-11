use anyhow::{anyhow, bail, Result};
use rustc_hash::FxHashSet;

use aoc::planar::{Direction, Point};
use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let motions: Vec<_> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", count_visited::<2>(&motions));
    println!("Part 2: {}", count_visited::<10>(&motions));

    Ok(())
}

fn count_visited<const N: usize>(motions: &[Motion]) -> usize {
    let mut knots = [Knot::default(); N];
    let mut visited: FxHashSet<Knot> = FxHashSet::default();
    visited.insert(knots[N - 1]);
    for motion in motions {
        let diff = motion.direction.unit_vector();
        for _ in 0..motion.distance {
            knots[0].0 += diff;
            for i in 1..N {
                knots[i].follow(knots[i - 1]);
            }
            visited.insert(knots[N - 1]);
        }
    }

    visited.len()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Knot(Point);

impl Knot {
    fn follow(&mut self, head: Knot) {
        let Point { x: dx, y: dy } = head.0 - self.0;
        if dx.abs() > 1 || dy.abs() > 1 {
            self.0.x += dx.signum();
            self.0.y += dy.signum();
        }
    }
}

#[derive(Debug, Clone)]
struct Motion {
    direction: Direction,
    distance: u32,
}

impl std::str::FromStr for Motion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid motion: {}", s))?;
        let direction = match direction {
            "U" => Direction::North,
            "D" => Direction::South,
            "L" => Direction::West,
            "R" => Direction::East,
            _ => bail!("Invalid direction {}", s),
        };
        let distance = distance.parse()?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SIMPLE_EXAMPLE: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    static LARGER_EXAMPLE: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn two_knots() {
        let motions: Vec<Motion> = parse_fields(SIMPLE_EXAMPLE, '\n').unwrap();
        assert_eq!(count_visited::<2>(&motions), 13);
    }

    #[test]
    fn ten_knots_small() {
        let motions: Vec<Motion> = parse_fields(SIMPLE_EXAMPLE, '\n').unwrap();
        assert_eq!(count_visited::<10>(&motions), 1);
    }

    #[test]
    fn ten_knots_large() {
        let motions: Vec<Motion> = parse_fields(LARGER_EXAMPLE, '\n').unwrap();
        assert_eq!(count_visited::<10>(&motions), 36);
    }
}
