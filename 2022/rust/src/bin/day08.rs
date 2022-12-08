use anyhow::{anyhow, Result};

use aoc::planar::{Direction, TileMap};
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map = TreeMap(input.trim().parse()?);

    println!("Part 1: {}", map.visible_count());
    println!("Part 2: {}", map.max_scenic_score());

    Ok(())
}

#[derive(Debug, Clone)]
struct TreeMap(TileMap<Height>);

impl TreeMap {
    fn visible_count(&self) -> usize {
        self.0
            .iter_coords()
            .filter(|&(x, y)| self.is_visible(x, y))
            .count()
    }

    fn max_scenic_score(&self) -> usize {
        self.0
            .iter_coords()
            .map(|(x, y)| self.scenic_score(x, y))
            .max()
            .unwrap_or(0)
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        Direction::iterator().any(|dir| self.is_visible_from(x, y, dir))
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        Direction::iterator().fold(1, |score, dir| score * self.viewing_distance(x, y, dir))
    }

    fn is_visible_from(&self, x: usize, y: usize, dir: Direction) -> bool {
        let height = self.0.get(x, y).unwrap_or(&Height(0));
        self.0.iter_dir(x, y, dir).all(|h| h < height)
    }

    fn viewing_distance(&self, x: usize, y: usize, dir: Direction) -> usize {
        let height = self.0.get(x, y).unwrap_or(&Height(10));
        let mut distance = 0;
        for h in self.0.iter_dir(x, y, dir) {
            distance += 1;
            if h >= height {
                break;
            }
        }
        distance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Height(u32);

impl TryFrom<char> for Height {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        c.to_digit(10)
            .map(Height)
            .ok_or_else(|| anyhow!("Failed to parse height: {}", c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        30373\n\
        25512\n\
        65332\n\
        33549\n\
        35390";

    #[test]
    fn is_visible() {
        let map = TreeMap(TEST_DATA.parse().unwrap());
        for i in 0..5 {
            assert!(map.is_visible(i, 0));
            assert!(map.is_visible(i, 4));
            assert!(map.is_visible(0, i));
            assert!(map.is_visible(4, i));
        }
        assert!(map.is_visible(1, 1));
        assert!(map.is_visible(2, 1));
        assert!(!map.is_visible(3, 1));
        assert!(map.is_visible(1, 2));
        assert!(!map.is_visible(2, 2));
        assert!(map.is_visible(3, 2));
        assert!(!map.is_visible(1, 3));
        assert!(map.is_visible(2, 3));
        assert!(!map.is_visible(3, 3));
    }

    #[test]
    fn visible_count() {
        let map = TreeMap(TEST_DATA.parse().unwrap());
        assert_eq!(map.visible_count(), 21);
    }

    #[test]
    fn scenic_score_1() {
        let map = TreeMap(TEST_DATA.parse().unwrap());
        assert_eq!(map.scenic_score(2, 1), 4);
    }

    #[test]
    fn scenic_score_2() {
        let map = TreeMap(TEST_DATA.parse().unwrap());
        assert_eq!(map.scenic_score(2, 3), 8);
    }

    #[test]
    fn scenic_score_3() {
        let map = TreeMap(TEST_DATA.parse().unwrap());
        assert_eq!(map.scenic_score(0, 2), 0);
    }

    #[test]
    fn max_scenic_score() {
        let map = TreeMap(TEST_DATA.parse().unwrap());
        assert_eq!(map.max_scenic_score(), 8);
    }
}
