use anyhow::{anyhow, Result};
use itertools::Itertools;
use rustc_hash::FxHashSet;

use aoc::planar::{CardinalDirection, Direction, TileMap, Turn};

// Optimizations to consider:
//      Replace FxHashSet with Vectors
//      Jump Map?
//      For part 2, Can just step through the path, and try an obstruction at each point, then
//      check for a loop

type LabMap = TileMap<Tile>;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let (map, guard) = parse_input(&input)?;

    println!("Part 1: {}", part1(&map, guard)?);
    println!("Part 2: {}", part2(&map, guard)?);

    Ok(())
}

fn parse_input(input: &str) -> Result<(LabMap, Guard)> {
    let map: LabMap = input.trim().parse()?;
    let (x, y) = map
        .iter_coords()
        .filter(|(x, y)| map.get(*x, *y) == Some(&Tile::Guard))
        .exactly_one()
        .map_err(|_| anyhow!("Failed to find unique guard"))?;
    let direction = CardinalDirection::North;
    Ok((map, Guard { x, y, direction }))
}

fn part1(map: &LabMap, guard: Guard) -> Result<usize> {
    let visited = guard
        .visited(map, None)
        .ok_or_else(|| anyhow!("Loop detected"))?;
    let coords: FxHashSet<(usize, usize)> = visited
        .into_iter()
        .map(|guard| (guard.x, guard.y))
        .collect();
    Ok(coords.len())
}

fn part2(map: &LabMap, guard: Guard) -> Result<usize> {
    let candidates = guard
        .visited(map, None)
        .ok_or_else(|| anyhow!("Loop detected"))?;
    let mut count = 0;
    for mut g in candidates {
        let mut obstruction = g;
        obstruction.step(map, None);
        g.direction = g.direction.turn(Turn::Clockwise);
        if g.visited(map, Some((obstruction.x, obstruction.y)))
            .is_none()
        {
            count += 1;
        }
    }
    Ok(count)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    x: usize,
    y: usize,
    direction: CardinalDirection,
}

impl Guard {
    fn step(&mut self, map: &LabMap, obstruction: Option<(usize, usize)>) -> bool {
        let (x, y) = match map.step(self.x, self.y, self.direction) {
            Some((x, y)) => (x, y),
            None => return false,
        };
        if Some((x, y)) == obstruction || map.get(x, y) == Some(&Tile::Obstruction) {
            self.direction = self.direction.turn(Turn::Clockwise);
        } else {
            (self.x, self.y) = (x, y);
        }
        true
    }

    fn visited(
        &self,
        map: &LabMap,
        obstruction: Option<(usize, usize)>,
    ) -> Option<FxHashSet<Guard>> {
        let mut guard = *self;
        let mut visited: FxHashSet<Guard> = FxHashSet::default();
        visited.insert(guard);
        while guard.step(map, obstruction) {
            if !visited.insert(guard) {
                return None;
            }
        }
        Some(visited)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Space,
    Obstruction,
    Guard,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Space),
            '#' => Ok(Tile::Obstruction),
            '^' => Ok(Tile::Guard),
            _ => Err(anyhow!("Unrecognized tile: {}", value)),
        }
    }
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
        assert_eq!(part2(&map, guard).unwrap(), 6);
    }
}
