use aoc::intcode::RegisterValue;
use aoc::point::Point2D;
use num_enum::TryFromPrimitive;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::rc::Rc;

#[cfg(test)]
use aoc::intcode::FakeVM as VM;
#[cfg(not(test))]
use aoc::intcode::VM;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Point = Point2D<RegisterValue>;

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
    last_ball_location: Rc<RefCell<Option<RegisterValue>>>,
    last_paddle_location: Rc<RefCell<Option<RegisterValue>>>,
}

impl<'a> fmt::Display for ArcadeMachine<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.grid.len() == 0 {
            return write!(f, "");
        }
        let min_x = self.grid.keys().map(|p| p.x).min().unwrap();
        let max_x = self.grid.keys().map(|p| p.x).max().unwrap();
        let min_y = self.grid.keys().map(|p| p.y).min().unwrap();
        let max_y = self.grid.keys().map(|p| p.y).max().unwrap();

        let panel_char = |x, y| {
            self.grid
                .get(&Point { x: x, y: y })
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
    fn new(vm: VM) -> ArcadeMachine {
        let last_ball_location = Rc::new(RefCell::new(None));
        let last_paddle_location = Rc::new(RefCell::new(None));

        let mut machine = ArcadeMachine {
            vm: vm,
            grid: HashMap::new(),
            score: 0,
            last_ball_location: last_ball_location.clone(),
            last_paddle_location: last_paddle_location.clone(),
        };
        machine
            .vm
            .set_inputs(Some(Box::new(std::iter::from_fn(move || {
                let offset = last_ball_location.borrow().unwrap_or(0)
                    - last_paddle_location.borrow().unwrap_or(0);

                Some(offset.cmp(&0) as i64)
            }))));

        machine
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
                Tile::Ball => *self.last_ball_location.borrow_mut() = Some(x),
                Tile::Paddle => *self.last_paddle_location.borrow_mut() = Some(x),
                _ => (),
            }
            self.grid.insert(Point { x: x, y: y }, tile);
        }

        Ok(())
    }
}

fn part1(program: &Vec<RegisterValue>) -> Result<usize> {
    let vm = VM::new(program.clone(), None);
    let mut machine = ArcadeMachine::new(vm);
    machine.run()?;

    Ok(machine.grid.values().filter(|&t| t == &Tile::Block).count())
}

fn part2(program: &Vec<RegisterValue>) -> Result<RegisterValue> {
    let mut vm = VM::new(program.clone(), None);
    vm.set_memory(0, 2);
    let mut machine = ArcadeMachine::new(vm);
    machine.run()?;

    Ok(machine.score)
}

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let program = aoc::intcode::parse_program(&input)?;

    println!("Part 1: {}", part1(&program)?);
    println!("Part 2: {}", part2(&program)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let mut vm = VM::new(vec![], None);
        vm.set_outputs(vec![1, 2, 3, 6, 5, 4]);

        let mut machine = ArcadeMachine::new(vm);
        assert!(machine.run().is_ok());
        assert_eq!(machine.grid.len(), 2);
        assert_eq!(machine.grid.get(&Point { x: 1, y: 2 }), Some(&Tile::Paddle));
        assert_eq!(machine.grid.get(&Point { x: 6, y: 5 }), Some(&Tile::Ball));
    }
}
