use std::collections::VecDeque;

use anyhow::{anyhow, bail, Result};

use aoc::planar::TileMap;
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: TileMap<Height> = input.trim().parse()?;

    println!("Part 1: {}", part1(&map)?);
    println!("Part 2: {}", part2(&map)?);

    Ok(())
}

fn part1(map: &TileMap<Height>) -> Result<u32> {
    FromStartMap(map)
        .path_length()
        .ok_or(anyhow!("Failed to find route"))
}

fn part2(map: &TileMap<Height>) -> Result<u32> {
    FromEndMap(map)
        .path_length()
        .ok_or(anyhow!("Failed to find route"))
}

struct FromStartMap<'a>(&'a TileMap<Height>);

impl<'a> PathMap for FromStartMap<'a> {
    fn find_start(&self) -> Option<(usize, usize)> {
        self.0
            .iter_coords()
            .find(|&(x, y)| matches!(self.0.get(x, y), Some(Height::Start)))
    }

    fn map(&self) -> &TileMap<Height> {
        &self.0
    }

    fn can_move(&self, from: u8, to: u8) -> bool {
        to <= from + 1
    }

    fn is_end(&self, tile: &Height) -> bool {
        matches!(tile, Height::End)
    }
}

struct FromEndMap<'a>(&'a TileMap<Height>);

impl<'a> PathMap for FromEndMap<'a> {
    fn find_start(&self) -> Option<(usize, usize)> {
        self.0
            .iter_coords()
            .find(|&(x, y)| matches!(self.0.get(x, y), Some(Height::End)))
    }

    fn map(&self) -> &TileMap<Height> {
        &self.0
    }

    fn can_move(&self, from: u8, to: u8) -> bool {
        to >= from - 1
    }

    fn is_end(&self, tile: &Height) -> bool {
        tile.height() == 0
    }
}

trait PathMap {
    fn find_start(&self) -> Option<(usize, usize)>;

    fn map(&self) -> &TileMap<Height>;

    fn can_move(&self, from: u8, to: u8) -> bool;

    fn is_end(&self, tile: &Height) -> bool;

    fn path_length(&self) -> Option<u32> {
        let start = {
            let (x, y) = self.find_start()?;
            let height = self.map().get(x, y)?.height();
            GraphNode {
                x,
                y,
                height,
                steps: 0,
            }
        };
        let mut visited = vec![vec![false; self.map().width()]; self.map().height()];
        let mut to_visit = VecDeque::new();
        to_visit.push_back(start);
        while let Some(node) = to_visit.pop_front() {
            if visited[node.y][node.x] {
                continue;
            }
            visited[node.y][node.x] = true;
            for (x, y, tile) in self.map().manhattan_neighbors(node.x, node.y) {
                let height = tile.height();
                if self.can_move(node.height, height) {
                    if self.is_end(tile) {
                        return Some(node.steps + 1);
                    }
                    to_visit.push_back(GraphNode {
                        x,
                        y,
                        height,
                        steps: node.steps + 1,
                    });
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
struct GraphNode {
    x: usize,
    y: usize,
    height: u8,
    steps: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Height {
    Start,
    End,
    Height(u8),
}

impl Height {
    fn height(&self) -> u8 {
        match self {
            Self::Start => 0,
            Self::End => 25,
            Self::Height(h) => *h,
        }
    }
}

impl TryFrom<char> for Height {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End),
            'a'..='z' => Ok(Self::Height(c as u8 - b'a')),
            _ => bail!("Unexpected character {}", c),
        }
    }
}

impl From<&Height> for char {
    fn from(h: &Height) -> Self {
        match h {
            Height::Start => 'S',
            Height::End => 'E',
            Height::Height(n) => (n + b'a') as char,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_MAP: &str = "\
        Sabqponm\n\
        abcryxxl\n\
        accszExk\n\
        acctuvwj\n\
        abdefghi";

    #[test]
    fn path_length() {
        let map = TEST_MAP.parse().unwrap();
        assert_eq!(part1(&map).unwrap(), 31);
    }

    #[test]
    fn min_path_length() {
        let map = TEST_MAP.parse().unwrap();
        assert_eq!(part2(&map).unwrap(), 29);
    }
}
