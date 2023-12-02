use anyhow::{anyhow, bail, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let games: Vec<Game> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));

    Ok(())
}

fn part1(games: &[Game]) -> u64 {
    let cubeset = CubeSet::new(12, 13, 14);
    games
        .iter()
        .filter(|game| cubeset.contains_game(game))
        .map(|game| game.id)
        .sum()
}

fn part2(games: &[Game]) -> u64 {
    games
        .iter()
        .map(|game| CubeSet::smallest_containing(game).power())
        .sum()
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct CubeSet {
    red: u64,
    green: u64,
    blue: u64,
}

impl CubeSet {
    fn new(red: u64, green: u64, blue: u64) -> Self {
        CubeSet { red, green, blue }
    }

    fn smallest_containing(game: &Game) -> Self {
        let red = game.draws.iter().map(|draw| draw.red).max().unwrap_or(0);
        let green = game.draws.iter().map(|draw| draw.green).max().unwrap_or(0);
        let blue = game.draws.iter().map(|draw| draw.blue).max().unwrap_or(0);
        CubeSet { red, green, blue }
    }

    fn contains_game(&self, game: &Game) -> bool {
        game.draws.iter().all(|draw| self.contains(draw))
    }

    fn contains(&self, other: &CubeSet) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

impl std::str::FromStr for CubeSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubeset = CubeSet::default();
        for entry in s.split(", ") {
            let (n, color) = entry
                .split_once(' ')
                .ok_or_else(|| anyhow!("Invalid entry: {}", entry))?;
            match color {
                "red" => cubeset.red = n.parse()?,
                "green" => cubeset.green = n.parse()?,
                "blue" => cubeset.blue = n.parse()?,
                _ => bail!("Invalid color: {}", color),
            }
        }
        Ok(cubeset)
    }
}

struct Game {
    id: u64,
    draws: Vec<CubeSet>,
}

impl std::str::FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_part, draw_part) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Invalid game: {}", s))?;
        let id = id_part.trim_start_matches("Game ").parse()?;
        let draws = draw_part
            .split("; ")
            .map(|s| s.parse())
            .collect::<Result<_>>()?;
        Ok(Game { id, draws })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn contains_game() {
        let games: Vec<Game> = parse_fields(TEST_DATA, '\n').unwrap();
        let cubeset = CubeSet::new(12, 13, 14);
        let expected = vec![true, true, false, false, true];

        for (game, b) in games.iter().zip(expected) {
            assert_eq!(cubeset.contains_game(&game), b);
        }

        assert_eq!(part1(&games), 8);
    }

    #[test]
    fn smallest_containing() {
        let games: Vec<Game> = parse_fields(TEST_DATA, '\n').unwrap();
        let cubesets = vec![
            CubeSet::new(4, 2, 6),
            CubeSet::new(1, 3, 4),
            CubeSet::new(20, 13, 6),
            CubeSet::new(14, 3, 15),
            CubeSet::new(6, 3, 2),
        ];

        for (game, expected) in games.iter().zip(cubesets) {
            assert_eq!(CubeSet::smallest_containing(&game), expected);
        }

        assert_eq!(part2(&games), 2286);
    }
}
