use anyhow::{anyhow, bail, Result};
use rustc_hash::FxHashSet;

use aoc::planar::{CardinalDirection, Direction, Turn};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let (map, guard) = parse_input(&input)?;

    println!("Part 1: {}", part1(&map, guard)?);
    println!("Part 2: {}", part2(&map, guard)?);

    Ok(())
}

fn part1(map: &LabMap, mut guard: Guard) -> Result<usize> {
    let mut visited: FxHashSet<Guard> = FxHashSet::default();
    visited.insert(guard);
    while let Some(((x, y), obstructed)) = map.step(&guard) {
        if obstructed {
            guard.turn(Turn::Clockwise);
        } else {
            (guard.x, guard.y) = (x, y);
            if !visited.insert(guard) {
                bail!("Loop detected");
            }
        }
    }
    let visited: FxHashSet<(usize, usize)> = visited
        .into_iter()
        .map(|guard| (guard.x, guard.y))
        .collect();
    Ok(visited.len())
}

fn part2(map: &LabMap, guard: Guard) -> Result<usize> {
    Ok(map.obstruction_options(guard)?.len())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    x: usize,
    y: usize,
    direction: CardinalDirection,
}

impl Guard {
    fn turn(&mut self, turn: Turn) {
        self.direction = self.direction.turn(turn);
    }
}

#[derive(Debug, Clone)]
struct LabMap {
    width: usize,
    height: usize,
    obstructions: Vec<Vec<bool>>,
}

impl LabMap {
    fn step(&self, point: &Guard) -> Option<((usize, usize), bool)> {
        fn clamped_increment(value: usize, limit: usize) -> Option<usize> {
            (value + 1 < limit).then_some(value + 1)
        }
        let (x, y) = match point.direction {
            CardinalDirection::North => (point.x, clamped_increment(point.y, self.height)?),
            CardinalDirection::East => (clamped_increment(point.x, self.width)?, point.y),
            CardinalDirection::South => (point.x, point.y.checked_sub(1)?),
            CardinalDirection::West => (point.x.checked_sub(1)?, point.y),
        };
        Some(((x, y), self.obstructions[y][x]))
    }

    fn step_with_obstruction(
        &self,
        point: &Guard,
        obstruction: (usize, usize),
    ) -> Option<((usize, usize), bool)> {
        let ((x, y), obstructed) = self.step(point)?;
        Some(((x, y), obstructed || (x, y) == obstruction))
    }

    fn makes_loop(&self, mut guard: Guard, obstruction: (usize, usize)) -> bool {
        let mut landmarks: FxHashSet<Guard> = FxHashSet::default();
        while let Some(((x, y), obstructed)) = self.step_with_obstruction(&guard, obstruction) {
            if obstructed {
                if !landmarks.insert(guard) {
                    return true;
                }
                guard.turn(Turn::Clockwise);
            } else {
                (guard.x, guard.y) = (x, y);
            }
        }
        false
    }

    fn obstruction_options(&self, mut guard: Guard) -> Result<Vec<(usize, usize)>> {
        let mut visited: FxHashSet<Guard> = FxHashSet::default();
        let mut tested: FxHashSet<(usize, usize)> = FxHashSet::default();
        let mut result = vec![];
        while let Some(((x, y), obstructed)) = self.step(&guard) {
            if !visited.insert(guard) {
                bail!("Loop detected");
            }
            if obstructed {
                guard.turn(Turn::Clockwise);
            } else {
                // See if we get a loop by sticking an obstruction right in front of the guard.
                // Avoid retesting points both to improve performance and to avoid false positives.
                if tested.insert((x, y)) && self.makes_loop(guard, (x, y)) {
                    result.push((x, y));
                }
                (guard.x, guard.y) = (x, y);
            }
        }
        Ok(result)
    }
}

fn parse_input(input: &str) -> Result<(LabMap, Guard)> {
    let height = input.lines().count();
    let width = input.lines().next().map(|line| line.len()).unwrap_or(0);
    let mut guard = None;
    let mut obstructions = vec![vec![false; width]; height];
    for (y, line) in input.lines().rev().enumerate() {
        if line.len() != width {
            bail!("Non square map detected");
        }
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                '#' => obstructions[y][x] = true,
                '^' => {
                    let direction = CardinalDirection::North;
                    guard = Some(Guard { x, y, direction })
                }
                _ => bail!("Unrecognized character {} at {}, {}", c, x, y),
            }
        }
    }
    let guard = guard.ok_or_else(|| anyhow!("Failed to find guard"))?;
    Ok((
        LabMap {
            width,
            height,
            obstructions,
        },
        guard,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        ....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...";

    #[test]
    fn step_count() {
        let (map, guard) = parse_input(TEST_DATA).unwrap();
        assert_eq!(part1(&map, guard).unwrap(), 41);
    }

    #[test]
    fn obstruction_options() {
        let (map, guard) = parse_input(TEST_DATA).unwrap();
        let expected = vec![(3, 3), (6, 2), (3, 1), (1, 1), (7, 2), (7, 0)];
        let result = map.obstruction_options(guard).unwrap();
        assert_eq!(result, expected);
    }
}
