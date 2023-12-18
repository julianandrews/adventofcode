use std::marker::PhantomData;

use anyhow::{anyhow, bail, Result};

use aoc::planar::Direction;
use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let dig_plan: Vec<Instruction<Distance>> = parse_fields(input.trim(), '\n')?;
    let fixed_dig_plan: Vec<Instruction<Color>> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", loop_size(&dig_plan)?);
    println!("Part 2: {}", loop_size(&fixed_dig_plan)?);

    Ok(())
}

fn loop_size<T>(instructions: &[Instruction<T>]) -> Result<i64> {
    // Note: This algorithm assumes the loop is a simple polygon. It would
    // be better to validate this by looking at the line segments.
    let (mut x, mut y) = (0, 0);
    let mut corners = vec![(x, y)];
    let mut perimeter = 0;
    for instruction in instructions {
        let (dx, dy) = instruction.direction.step();
        let distance = instruction.distance as i64;
        perimeter += distance;
        (x, y) = (x + dx as i64 * distance, y + dy as i64 * distance);
        corners.push((x, y));
    }
    if corners.last() != corners.first() {
        bail!("Loop isn't closed");
    }
    let segments = corners.windows(2).map(|pair| (pair[0], pair[1]));
    let area = segments
        .map(|((x1, y1), (x2, y2))| (y1 + y2) * (x1 - x2) / 2)
        .sum::<i64>()
        .abs();
    Ok(area + perimeter / 2 + 1)
}

#[derive(Debug, Clone)]
struct Instruction<T> {
    direction: Direction,
    distance: u64,
    marker: PhantomData<T>,
}

// Marker structs for how to parse instructions.
#[derive(Debug)]
struct Distance;
#[derive(Debug)]
struct Color;

mod parsing {
    use super::*;

    impl std::str::FromStr for Instruction<Distance> {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (dir_part, dist_part, _) =
                split_parts(s).ok_or_else(|| anyhow!("Invalid instruction {}", s))?;
            let direction = match dir_part {
                "U" => Direction::North,
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                _ => bail!("Invalid direction in instruction {}", s),
            };
            let distance = dist_part.parse()?;

            Ok(Instruction {
                direction,
                distance,
                marker: PhantomData,
            })
        }
    }

    impl std::str::FromStr for Instruction<Color> {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (_, _, color_part) =
                split_parts(s).ok_or_else(|| anyhow!("Invalid instruction {}", s))?;
            let n = u64::from_str_radix(color_part, 16)?;
            let direction = match n & 0b11 {
                0 => Direction::East,
                1 => Direction::South,
                2 => Direction::West,
                3 => Direction::North,
                _ => bail!("Invalid direction from color in instruction {}", s),
            };
            let distance = n >> 4;

            Ok(Instruction {
                direction,
                distance,
                marker: PhantomData,
            })
        }
    }

    fn split_parts(s: &str) -> Option<(&str, &str, &str)> {
        let (dir_part, rest) = s.split_once(' ')?;
        let (dist_part, color_part) = rest.split_once(' ')?;
        let color_part = color_part.strip_prefix("(#")?.strip_suffix(')')?;
        Some((dir_part, dist_part, color_part))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        R 6 (#70c710)\n\
        D 5 (#0dc571)\n\
        L 2 (#5713f0)\n\
        D 2 (#d2c081)\n\
        R 2 (#59c680)\n\
        D 2 (#411b91)\n\
        L 5 (#8ceee2)\n\
        U 2 (#caa173)\n\
        L 1 (#1b58a2)\n\
        U 2 (#caa171)\n\
        R 2 (#7807d2)\n\
        U 3 (#a77fa3)\n\
        L 2 (#015232)\n\
        U 2 (#7a21e3)";

    #[test]
    fn using_distance() {
        let instructions: Vec<Instruction<Distance>> = parse_fields(TEST_DATA, '\n').unwrap();
        assert_eq!(loop_size(&instructions).unwrap(), 62);
    }

    #[test]
    fn using_color() {
        let instructions: Vec<Instruction<Color>> = parse_fields(TEST_DATA, '\n').unwrap();
        assert_eq!(loop_size(&instructions).unwrap(), 952408144115);
    }
}
