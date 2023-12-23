use anyhow::Result;

use aoc::planar::Direction;
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: LightMap = input.trim().parse()?;

    println!("Part 1: {}", map.energized(OrientedPoint::default()));
    println!("Part 2: {}", map.max_energized());

    Ok(())
}

#[derive(Debug, Clone)]
struct LightMap {
    elements: Vec<Element>,
    width: usize,
    height: usize,
}

impl LightMap {
    fn energized(&self, start: OrientedPoint) -> u32 {
        let mut energized: [Vec<u128>; 4] = [
            vec![0; self.height],
            vec![0; self.height],
            vec![0; self.height],
            vec![0; self.height],
        ];
        let mut positions = vec![start];
        while let Some(p) = positions.pop() {
            if let Some(element) = self.get(p.x, p.y) {
                let row = energized[p.direction as usize].get_mut(p.y).unwrap();
                if *row & (1 << p.x) != 0 {
                    continue;
                }
                *row |= 1 << p.x;
                for &direction in element.redirect(p.direction) {
                    if let Some((x, y)) = self.step(p.x, p.y, direction) {
                        positions.push(OrientedPoint::new(x, y, direction));
                    }
                }
            }
        }
        let mut count = 0;
        for y in 0..self.height {
            count +=
                (energized[0][y] | energized[1][y] | energized[2][y] | energized[3][y]).count_ones()
        }
        count
    }

    fn max_energized(&self) -> u32 {
        let mut max = 0;
        for x in 0..self.width {
            let p1 = OrientedPoint::new(x, 0, Direction::South);
            let p2 = OrientedPoint::new(x, self.height - 1, Direction::North);
            max = max.max(self.energized(p1)).max(self.energized(p2));
        }
        for y in 0..self.height {
            let p1 = OrientedPoint::new(0, y, Direction::East);
            let p2 = OrientedPoint::new(self.width - 1, y, Direction::West);
            max = max.max(self.energized(p1)).max(self.energized(p2));
        }
        max
    }

    fn get(&self, x: usize, y: usize) -> Option<&Element> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.elements.get(x + y * self.width)
    }

    fn step(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        Some(match direction {
            Direction::North => (x, y.checked_sub(1)?),
            Direction::East => (x.checked_add(1)?, y),
            Direction::South => (x, y.checked_add(1)?),
            Direction::West => (x.checked_sub(1)?, y),
        })
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
#[repr(u8)]
enum Element {
    Space,
    UpMirror,
    DownMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Element {
    fn redirect(&self, heading: Direction) -> &'static [Direction] {
        match self {
            Element::Space => match heading {
                Direction::North => &[Direction::North],
                Direction::East => &[Direction::East],
                Direction::South => &[Direction::South],
                Direction::West => &[Direction::West],
            },
            Element::UpMirror => match heading {
                Direction::North => &[Direction::East],
                Direction::East => &[Direction::North],
                Direction::South => &[Direction::West],
                Direction::West => &[Direction::South],
            },
            Element::DownMirror => match heading {
                Direction::North => &[Direction::West],
                Direction::East => &[Direction::South],
                Direction::South => &[Direction::East],
                Direction::West => &[Direction::North],
            },
            Element::VerticalSplitter => match heading {
                Direction::North => &[Direction::North],
                Direction::South => &[Direction::South],
                _ => &[Direction::North, Direction::South],
            },
            Element::HorizontalSplitter => match heading {
                Direction::East => &[Direction::East],
                Direction::West => &[Direction::West],
                _ => &[Direction::East, Direction::West],
            },
        }
    }
}

mod parsing {
    use super::{Element, LightMap};
    use anyhow::{bail, Result};

    impl std::str::FromStr for LightMap {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let width = s.lines().next().map(|line| line.len()).unwrap_or(0);
            if width > 128 {
                bail!("Map too wide, different data structure needed.");
            }
            if s.lines().any(|line| line.len() != width) {
                bail!("Non-rectangular map");
            }
            let height = s.lines().count();
            let elements = s
                .chars()
                .filter_map(|c| match c {
                    '\n' => None,
                    _ => Some(Element::try_from(c)),
                })
                .collect::<Result<_>>()?;
            Ok(LightMap {
                elements,
                width,
                height,
            })
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
        let map: LightMap = TEST_DATA.parse().unwrap();
        let result = map.energized(OrientedPoint::default());
        assert_eq!(result, 46);
    }

    #[test]
    fn max_energized() {
        let map: LightMap = TEST_DATA.parse().unwrap();
        assert_eq!(map.max_energized(), 51);
    }
}
