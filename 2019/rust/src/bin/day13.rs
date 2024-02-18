use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::sync::{Arc, Mutex};

use num_enum::TryFromPrimitive;

use aoc::intcode::RegisterValue;
use aoc::point::Point2D;

#[cfg(test)]
use aoc::intcode::FakeVM as VM;
#[cfg(not(test))]
use aoc::intcode::VM;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Point = Point2D<RegisterValue>;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let program = aoc::intcode::parse_program(&input)?;

    println!("Part 1: {}", part1(&program)?);
    println!("Part 2: {}", part2(&program)?);
    Ok(())
}

fn part1(program: &[RegisterValue]) -> Result<usize> {
    let mut machine = ArcadeMachine::new(program);
    machine.run()?;

    Ok(machine.grid.values().filter(|&t| t == &Tile::Block).count())
}

fn part2(program: &[RegisterValue]) -> Result<RegisterValue> {
    let mut machine = ArcadeMachine::new(program);
    machine.vm.set_memory(0, 2);
    machine.run()?;

    Ok(machine.score)
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Tile::Empty => ' ',
            Tile::Wall => '┼',
            Tile::Block => '█',
            Tile::Paddle => '▄',
            Tile::Ball => '°',
        };
        write!(f, "{}", c)
    }
}

struct ArcadeMachine<'a> {
    vm: VM<'a>,
    grid: HashMap<Point, Tile>,
    score: RegisterValue,
    last_ball_location: Arc<Mutex<Option<RegisterValue>>>,
    last_paddle_location: Arc<Mutex<Option<RegisterValue>>>,
}

impl<'a> fmt::Display for ArcadeMachine<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.grid.is_empty() {
            return write!(f, "");
        }
        let min_x = self.grid.keys().map(|p| p.x).min().unwrap();
        let max_x = self.grid.keys().map(|p| p.x).max().unwrap();
        let min_y = self.grid.keys().map(|p| p.y).min().unwrap();
        let max_y = self.grid.keys().map(|p| p.y).max().unwrap();

        let panel_char = |x, y| {
            self.grid
                .get(&Point { x, y })
                .unwrap_or(&Tile::Empty)
                .to_string()
        };

        let s = (min_y..=max_y)
            .map(|y| (min_x..=max_x).map(|x| panel_char(x, y)).collect())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

impl<'a> ArcadeMachine<'a> {
    fn new(program: &[RegisterValue]) -> ArcadeMachine {
        let last_ball_location = Arc::new(Mutex::new(None));
        let last_paddle_location = Arc::new(Mutex::new(None));
        let lb = last_ball_location.clone();
        let lp = last_paddle_location.clone();
        let vm = VM::from_iterator(
            program.to_vec(),
            std::iter::from_fn(move || {
                let offset = lb.lock().unwrap().unwrap_or(0) - lp.lock().unwrap().unwrap_or(0);

                Some(offset.cmp(&0) as i64)
            }),
        );
        ArcadeMachine {
            vm,
            grid: HashMap::new(),
            score: 0,
            last_ball_location,
            last_paddle_location,
        }
    }

    fn run(&mut self) -> Result<()> {
        let mut outputs = self.vm.outputs();
        while let (Some(x), Some(y), Some(tile_id)) =
            (outputs.next(), outputs.next(), outputs.next())
        {
            if x == -1 && y == 0 {
                self.score = tile_id;
                continue;
            }
            let tile = Tile::try_from(u8::try_from(tile_id)?)?;
            match tile {
                Tile::Ball => *self.last_ball_location.lock().unwrap() = Some(x),
                Tile::Paddle => *self.last_paddle_location.lock().unwrap() = Some(x),
                _ => (),
            }
            self.grid.insert(Point { x, y }, tile);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let mut machine = ArcadeMachine::new(&[]);
        machine.vm.set_outputs(vec![1, 2, 3, 6, 5, 4]);
        assert!(machine.run().is_ok());
        assert_eq!(machine.grid.len(), 2);
        assert_eq!(machine.grid.get(&Point { x: 1, y: 2 }), Some(&Tile::Paddle));
        assert_eq!(machine.grid.get(&Point { x: 6, y: 5 }), Some(&Tile::Ball));
    }
}
