use anyhow::{bail, Result};
use rustc_hash::FxHashMap;

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let dirs = parse_directions(input.trim())?;

    println!("Part 1: {}", part1(&dirs));
    println!("Part 2: {}", part2(&dirs));

    Ok(())
}

fn part1(dirs: &[Direction]) -> usize {
    tower_height(dirs, 2022)
}

fn part2(dirs: &[Direction]) -> usize {
    tower_height(dirs, 1000000000000)
}

fn tower_height(dirs: &[Direction], n: usize) -> usize {
    // Two flat pieces can't slip by each other. Track the coordinates of five-rock groups and the
    // index of the direction to find a cycle.

    // Run until we detect a cycle (or reach n).
    let mut rockfall = RockFall { lines: vec![] };
    let mut dir_iter = dirs.iter().cloned().enumerate().cycle();
    let mut seen: FxHashMap<State, (usize, usize)> = FxHashMap::default();
    let mut rocks = FallingRock::iter();
    let mut rock_count = 0;
    let (cycle_start, initial_height) = loop {
        let mut state = State::default();
        for i in 0..5 {
            let (dir_index, p) =
                rockfall.add_rock(*rocks.next().expect("Rocks forever!"), &mut dir_iter);
            state.rocks[i] = p;
            state.dir_index = dir_index;
            rock_count += 1;
            if rock_count == n {
                return rockfall.height();
            }
        }
        if let Some(value) = seen.get(&state) {
            break *value;
        }
        seen.insert(state, (rock_count, rockfall.height()));
    };

    // Next figure out how many cycles we need and how many extra rocks we'll need to finish up.
    let cycle_height = rockfall.height() - initial_height;
    let cycle_length = rock_count - cycle_start;
    let cycles = (n - cycle_start) / cycle_length;
    let extra_rocks = (n - cycle_start) % cycle_length;

    // Finally finish up the extra rocks.
    for _ in 0..extra_rocks {
        rockfall.add_rock(*rocks.next().expect("Rocks forever!"), &mut dir_iter);
    }
    let first_run = rockfall.height();

    // The first run includes pre-cycle rocks, one full cycle, and the extra rocks. The remaining
    // `cycles - 1` cycles will contribute `cycle_height` each.
    first_run + (cycles - 1) * cycle_height
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct State {
    rocks: [(u8, u8); 5],
    dir_index: usize,
}

#[derive(Debug, Clone)]
struct RockFall {
    lines: Vec<Line>,
}

impl RockFall {
    fn add_rock(
        &mut self,
        mut rock: FallingRock,
        mut dirs: impl Iterator<Item = (usize, Direction)>,
    ) -> (usize, (u8, u8)) {
        let mut height: i32 = 3;
        loop {
            let (i, dir) = dirs.next().expect("Wind ran out!");
            rock = {
                let shifted = rock.shift(dir);
                if self.intersects(shifted, height) {
                    rock
                } else {
                    shifted
                }
            };
            let at_bottom = -height == self.height() as i32;
            if self.intersects(rock, height - 1) || at_bottom {
                self.place(rock, height);
                return (i, (rock.offset, -height as u8));
            }
            height -= 1;
        }
    }

    fn height(&self) -> usize {
        self.lines.len()
    }

    fn intersects(&mut self, rock: FallingRock, height: i32) -> bool {
        let lines = rock.lines();
        self.overlap(lines, height).any(|(a, b)| a.intersects(&b))
    }

    fn place(&mut self, rock: FallingRock, height: i32) {
        let mut overlap_count = 0;
        let lines = rock.lines();
        for (a, b) in self.overlap(lines.clone(), height) {
            overlap_count += 1;
            a.merge(&b);
        }
        for line in lines.iter().skip(overlap_count) {
            self.lines.push(*line);
        }
    }

    fn overlap(
        &mut self,
        lines: Vec<Line>,
        height: i32,
    ) -> impl Iterator<Item = (&mut Line, Line)> {
        let overlap_size = (-height).max(0) as usize;
        let i = self.height().saturating_sub(overlap_size);
        self.lines[i..].iter_mut().zip(lines)
    }
}

impl std::fmt::Display for RockFall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.lines.iter().rev() {
            write!(f, "|")?;
            for i in 0..7 {
                let c = match line.0 & (1 << i) {
                    0 => '·',
                    _ => '#',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, "|")?;
        }
        write!(f, "+-------+")
    }
}

#[derive(Debug, Clone, Copy)]
struct FallingRock {
    rock: Rock,
    offset: u8,
}

impl FallingRock {
    const fn new(rock: Rock, offset: u8) -> Self {
        Self { rock, offset }
    }

    fn shift(&self, direction: Direction) -> Self {
        match direction {
            Direction::Left => FallingRock::new(self.rock, self.offset.saturating_sub(1)),
            Direction::Right => {
                FallingRock::new(self.rock, (self.offset + 1).min(7 - self.rock.width()))
            }
        }
    }

    fn lines(&self) -> Vec<Line> {
        self.rock
            .lines()
            .iter()
            .map(|line| line.shift(self.offset))
            .collect()
    }

    fn iter() -> impl Iterator<Item = &'static Self> {
        static ROCKS: &[FallingRock] = &[
            FallingRock::new(Rock::Flat, 2),
            FallingRock::new(Rock::Plus, 2),
            FallingRock::new(Rock::L, 2),
            FallingRock::new(Rock::Vertical, 2),
            FallingRock::new(Rock::Square, 2),
        ];

        ROCKS.iter().cycle()
    }
}

#[derive(Debug, Clone, Copy)]
enum Rock {
    Flat,
    Plus,
    L,
    Vertical,
    Square,
}

impl Rock {
    fn lines(&self) -> &[Line] {
        static LINES: &[&[Line]] = &[
            &[Line(0b1111)],
            &[Line(0b010), Line(0b111), Line(0b010)],
            &[Line(0b111), Line(0b100), Line(0b100)],
            &[Line(0b1), Line(0b1), Line(0b1), Line(0b1)],
            &[Line(0b11), Line(0b11)],
        ];

        match self {
            Rock::Flat => LINES[0],
            Rock::Plus => LINES[1],
            Rock::L => LINES[2],
            Rock::Vertical => LINES[3],
            Rock::Square => LINES[4],
        }
    }

    fn width(&self) -> u8 {
        match self {
            Rock::Flat => 4,
            Rock::Plus => 3,
            Rock::L => 3,
            Rock::Vertical => 1,
            Rock::Square => 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line(u8);

impl Line {
    fn shift(&self, offset: u8) -> Self {
        Line(self.0 << offset)
    }

    fn intersects(&self, other: &Line) -> bool {
        self.0 & other.0 != 0
    }

    fn merge(&mut self, other: &Line) {
        self.0 |= other.0
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

fn parse_directions(s: &str) -> Result<Vec<Direction>> {
    s.bytes()
        .map(|b| match b {
            b'>' => Ok(Direction::Right),
            b'<' => Ok(Direction::Left),
            _ => bail!("Unexpected byte {:?}", b),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn short_tower() {
        let dirs = parse_directions(TEST_DATA).unwrap();
        assert_eq!(tower_height(&dirs, 2022), 3068);
    }

    #[test]
    fn tall_tower() {
        let dirs = parse_directions(TEST_DATA).unwrap();
        assert_eq!(tower_height(&dirs, 1000000000000), 1514285714288);
    }
}
