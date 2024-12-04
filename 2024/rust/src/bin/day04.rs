use anyhow::{anyhow, Result};
use aoc::planar::{CompassPoint, TileMap};
use num_enum::IntoPrimitive;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let wordsearch: TileMap<Letter> = input.parse()?;

    println!("Part 1: {}", part1(&wordsearch));
    println!("Part 2: {}", part2(&wordsearch));

    Ok(())
}

fn part1(wordsearch: &TileMap<Letter>) -> usize {
    wordsearch
        .iter_coords()
        .flat_map(|(x, y)| CompassPoint::iter().filter(move |&d| is_xmas(wordsearch, x, y, d)))
        .count()
}

fn part2(wordsearch: &TileMap<Letter>) -> usize {
    wordsearch
        .iter_coords()
        .filter(|&(x, y)| is_x_mas(wordsearch, x, y))
        .count()
}

fn is_xmas(
    wordsearch: &TileMap<Letter>,
    mut x: usize,
    mut y: usize,
    direction: CompassPoint,
) -> bool {
    for letter in [Letter::X, Letter::M, Letter::A, Letter::S] {
        if wordsearch.get(x, y) != Some(&letter) {
            return false;
        }
        (x, y) = match wordsearch.step(x, y, direction) {
            Some((x, y)) => (x, y),
            None if letter == Letter::S => break,
            None => return false,
        }
    }
    true
}

fn is_x_mas(wordsearch: &TileMap<Letter>, x: usize, y: usize) -> bool {
    fn diagonal_bitsets(wordsearch: &TileMap<Letter>, x: usize, y: usize) -> Option<(u8, u8)> {
        let get_in_dir = |d: CompassPoint| {
            let (a, b) = wordsearch.step(x, y, d)?;
            wordsearch.get(a, b).map(|&l| u8::from(l))
        };

        let nw = get_in_dir(CompassPoint::NW)?;
        let se = get_in_dir(CompassPoint::SE)?;
        let ne = get_in_dir(CompassPoint::NE)?;
        let sw = get_in_dir(CompassPoint::SW)?;

        Some((nw | se, ne | sw))
    }

    wordsearch.get(x, y) == Some(&Letter::A)
        && diagonal_bitsets(wordsearch, x, y) == Some((0b1010, 0b1010))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
enum Letter {
    X = 1,
    M = 2,
    A = 4,
    S = 8,
}

impl TryFrom<char> for Letter {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'X' => Ok(Letter::X),
            'M' => Ok(Letter::M),
            'A' => Ok(Letter::A),
            'S' => Ok(Letter::S),
            _ => Err(anyhow!("Unrecognized character")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX";

    #[test]
    fn count_xmas() {
        let wordsearch: TileMap<Letter> = TEST_DATA.parse().unwrap();
        assert_eq!(part1(&wordsearch), 18);
    }

    #[test]
    fn count_x_mas() {
        let wordsearch: TileMap<Letter> = TEST_DATA.parse().unwrap();
        assert_eq!(part2(&wordsearch), 9);
    }
}
