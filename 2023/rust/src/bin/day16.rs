use anyhow::{bail, Result};

use aoc::planar::{Direction, TileMap};
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: LightMap = LightMap::new(input.trim().parse()?);

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn part1(map: &LightMap) -> u32 {
    let energized = map.energized(OrientedPoint::default());
    #[cfg(feature = "verbose")]
    println!("{}", energized);
    energized.count()
}

fn part2(map: &LightMap) -> u32 {
    map.max_energized()
}

#[derive(Debug, Clone)]
struct LightMap {
    height: usize,
    width: usize,
    segments: Vec<Option<(EnergizedSet, Vec<OrientedPoint>)>>,
}

impl LightMap {
    fn new(map: TileMap<Element>) -> LightMap {
        let (height, width) = (map.height(), map.width());
        let mut segments = vec![None; height * width * 4];
        let mut to_visit = entrypoints(height, width);
        while let Some(start) = to_visit.pop() {
            if segments[start.simple_hash(width)].is_some() {
                continue;
            }

            let mut point = start;
            let mut energized = EnergizedSet::new(height, width);
            loop {
                energized.set(point.x, point.y);
                let element = map.get(point.x, point.y).unwrap();
                match element.redirect(point.direction) {
                    &[direction] => match map.step(point.x, point.y, direction) {
                        Some((x, y)) => point = OrientedPoint::new(x, y, direction),
                        None => {
                            segments[start.simple_hash(width)] =
                                Some((energized.clone(), vec![point]));
                            break;
                        }
                    },
                    directions => {
                        let mut neighbors = vec![];
                        for &direction in directions {
                            point = OrientedPoint::new(point.x, point.y, direction);
                            to_visit.push(point);
                            neighbors.push(point);
                        }
                        segments[start.simple_hash(width)] = Some((energized.clone(), neighbors));
                        break;
                    }
                }
            }
        }

        LightMap {
            height,
            width,
            segments,
        }
    }

    fn energized(&self, start: OrientedPoint) -> EnergizedSet {
        let mut to_visit = vec![start];
        let mut energized = EnergizedSet::new(self.height, self.width);
        let mut seen = vec![false; self.segments.len()];
        while let Some(point) = to_visit.pop() {
            let index = point.simple_hash(self.width);
            if seen[index] {
                continue;
            }
            seen[index] = true;
            if let Some((new, neighbors)) = self.segments[index].as_ref() {
                energized.merge(new);
                to_visit.extend(neighbors);
            }
        }
        energized
    }

    fn max_energized(&self) -> u32 {
        entrypoints(self.height, self.width)
            .iter()
            .map(|&p| self.energized(p).count())
            .max()
            .unwrap_or(0)
    }
}

fn entrypoints(height: usize, width: usize) -> Vec<OrientedPoint> {
    let mut points = vec![];
    for x in 0..width {
        points.push(OrientedPoint::new(x, 0, Direction::South));
        points.push(OrientedPoint::new(x, height - 1, Direction::North));
    }
    for y in 0..height {
        points.push(OrientedPoint::new(0, y, Direction::East));
        points.push(OrientedPoint::new(width - 1, y, Direction::West));
    }
    points
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

    fn simple_hash(&self, width: usize) -> usize {
        self.direction as usize | ((self.x + self.y * width) << 2)
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

#[derive(Debug, Clone)]
struct EnergizedSet {
    width: usize,
    rows: Vec<u128>,
}

impl EnergizedSet {
    fn new(height: usize, width: usize) -> EnergizedSet {
        EnergizedSet {
            width,
            rows: vec![0; height],
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        self.rows[y] |= 1 << x;
    }

    fn count(&self) -> u32 {
        self.rows.iter().map(|row| row.count_ones()).sum()
    }

    fn merge(&mut self, other: &EnergizedSet) {
        for (a, b) in self.rows.iter_mut().zip(&other.rows) {
            *a |= b;
        }
    }
}

impl std::fmt::Display for EnergizedSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in &self.rows {
            for x in 0..self.width {
                write!(f, "{}", if row & 1 << x == 0 { '.' } else { '#' })?;
            }
            writeln!(f)?
        }
        Ok(())
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
        let map: LightMap = LightMap::new(TEST_DATA.parse().unwrap());
        let energized = map.energized(OrientedPoint::default());
        let expected = "\
            ######....\n\
            .#...#....\n\
            .#...#####\n\
            .#...##...\n\
            .#...##...\n\
            .#...##...\n\
            .#..####..\n\
            ########..\n\
            .#######..\n\
            .#...#.#..\n";

        assert_eq!(energized.to_string(), expected);
        assert_eq!(energized.count(), 46);
    }

    #[test]
    fn max_energized() {
        let map: LightMap = LightMap::new(TEST_DATA.parse().unwrap());
        let energized = map.energized(OrientedPoint::new(3, 0, Direction::South));
        let expected = "\
            .#####....\n\
            .#.#.#....\n\
            .#.#.#####\n\
            .#.#.##...\n\
            .#.#.##...\n\
            .#.#.##...\n\
            .#.#####..\n\
            ########..\n\
            .#######..\n\
            .#...#.#..\n";

        assert_eq!(energized.to_string(), expected);
        assert_eq!(map.max_energized(), 51);
    }
}
