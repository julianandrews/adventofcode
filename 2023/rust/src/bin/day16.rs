use std::iter;

use anyhow::{bail, Result};
use rustc_hash::FxHashSet;

use aoc::planar::{Direction, TileMap};
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map = LightMap(input.trim().parse()?);

    println!("Part 1: {}", map.energized(OrientedPoint::default()));
    println!("Part 2: {}", map.max_energized());

    Ok(())
}

#[derive(Debug, Clone)]
struct LightMap(TileMap<Element>);

impl LightMap {
    fn energized(&self, start: OrientedPoint) -> usize {
        let mut energized = FxHashSet::default();
        let mut positions = vec![start];
        while let Some(p) = positions.pop() {
            if !energized.insert(p) {
                continue;
            }
            if let Some(element) = self.0.get(p.x, p.y) {
                for direction in element.redirect(p.direction) {
                    if let Some((x, y)) = self.0.step(p.x, p.y, direction) {
                        positions.push(OrientedPoint::new(x, y, direction));
                    }
                }
            }
        }
        energized
            .into_iter()
            .map(|p| (p.x, p.y))
            .collect::<FxHashSet<_>>()
            .len()
    }

    fn max_energized(&self) -> usize {
        let mut max = 0;
        for x in 0..self.0.width() {
            let p1 = OrientedPoint::new(x, 0, Direction::South);
            let p2 = OrientedPoint::new(x, self.0.height() - 1, Direction::North);
            max = max.max(self.energized(p1)).max(self.energized(p2));
        }
        for y in 0..self.0.height() {
            let p1 = OrientedPoint::new(0, y, Direction::East);
            let p2 = OrientedPoint::new(self.0.width() - 1, y, Direction::West);
            max = max.max(self.energized(p1)).max(self.energized(p2));
        }
        max
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OrientedPoint {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Default for OrientedPoint {
    fn default() -> Self {
        OrientedPoint::new(0, 0, Direction::East)
    }
}

impl OrientedPoint {
    fn new(x: usize, y: usize, direction: Direction) -> OrientedPoint {
        OrientedPoint { x, y, direction }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Space,
    UpMirror,
    DownMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Element {
    fn redirect(&self, heading: Direction) -> Box<dyn Iterator<Item = Direction>> {
        match self {
            Element::Space => Box::new(iter::once(heading)),
            Element::UpMirror => Box::new(iter::once(match heading {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            })),
            Element::DownMirror => Box::new(iter::once(match heading {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
            })),
            Element::VerticalSplitter => match heading {
                Direction::North | Direction::South => Box::new(iter::once(heading)),
                _ => Box::new(iter::once(Direction::North).chain(iter::once(Direction::South))),
            },
            Element::HorizontalSplitter => match heading {
                Direction::East | Direction::West => Box::new(iter::once(heading)),
                _ => Box::new(iter::once(Direction::East).chain(iter::once(Direction::West))),
            },
        }
    }
}

impl TryFrom<char> for Element {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Element::Space),
            '/' => Ok(Element::UpMirror),
            '\\' => Ok(Element::DownMirror),
            '|' => Ok(Element::VerticalSplitter),
            '-' => Ok(Element::HorizontalSplitter),
            _ => bail!("Unrecognized character {}", value),
        }
    }
}

impl From<&Element> for char {
    fn from(value: &Element) -> Self {
        match value {
            Element::Space => '.',
            Element::UpMirror => '/',
            Element::DownMirror => '\\',
            Element::VerticalSplitter => '|',
            Element::HorizontalSplitter => '-',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        .|...\\....\n\
        |.-.\\.....\n\
        .....|-...\n\
        ........|.\n\
        ..........\n\
        .........\\\n\
        ..../.\\\\..\n\
        .-.-/..|..\n\
        .|....-|.\\\n\
        ..//.|....";

    #[test]
    fn energized() {
        let map = LightMap(TEST_DATA.parse().unwrap());
        let result = map.energized(OrientedPoint::default());
        assert_eq!(result, 46);
    }

    #[test]
    fn max_energized() {
        let map = LightMap(TEST_DATA.parse().unwrap());
        assert_eq!(map.max_energized(), 51);
    }
}
