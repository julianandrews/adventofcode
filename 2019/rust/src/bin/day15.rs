extern crate aoc;

use aoc::aoc_error::AOCError;
use aoc::direction::Direction;
use aoc::graphs;
use aoc::intcode::{RegisterValue, VM};
use aoc::point::Point2D;
use num_enum::TryFromPrimitive;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::io::{self, Read};
use std::rc::Rc;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Point = Point2D<i64>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum StatusCode {
    HitWall,
    Moved,
    FoundOxygen,
    Unexplored,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            StatusCode::Moved => '·',
            StatusCode::HitWall => '█',
            StatusCode::FoundOxygen => '$',
            StatusCode::Unexplored => '▒',
        };
        write!(f, "{}", c)
    }
}

struct ShipMap {
    status_map: HashMap<Point, StatusCode>,
    explorer_position: Point,
}

impl fmt::Display for ShipMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.status_map.len() == 0 {
            return write!(f, "");
        }
        let min_x = self.status_map.keys().map(|p| p.x).min().unwrap();
        let max_x = self.status_map.keys().map(|p| p.x).max().unwrap();
        let min_y = self.status_map.keys().map(|p| p.y).min().unwrap();
        let max_y = self.status_map.keys().map(|p| p.y).max().unwrap();

        let map_char = |x, y| {
            let p = Point { x: x, y: y };
            if p == self.explorer_position {
                "@".to_string()
            } else {
                self.get_status(&p).to_string()
            }
        };

        let s = (min_y..=max_y)
            .rev()
            .map(|y| (min_x..=max_x).map(|x| map_char(x, y)).collect())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

impl<'a> graphs::Graph<'a> for ShipMap {
    type Item = Point;

    fn nodes(&'a self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        Box::new(self.status_map.keys().cloned())
    }

    fn neighbors(&'a self, point: &Self::Item) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        let mut neighbors = vec![];
        for d in Direction::iterator() {
            let p = d.next_position(*point);
            if self.status_map.get(&p) != Some(&StatusCode::HitWall) {
                neighbors.push(p);
            }
        }

        Box::new(neighbors.into_iter())
    }
}

impl ShipMap {
    fn new() -> ShipMap {
        let mut status_map = HashMap::new();
        let explorer_position = Point { x: 0, y: 0 };
        status_map.insert(explorer_position, StatusCode::Moved);

        ShipMap {
            status_map: status_map,
            explorer_position: explorer_position,
        }
    }

    fn get_status(&self, position: &Point) -> &StatusCode {
        self.status_map
            .get(position)
            .unwrap_or(&StatusCode::Unexplored)
    }

    fn find_oxygen(&self, start: Point) -> Option<Rc<graphs::TraversalNode<Point>>> {
        for node in graphs::bfs(self, start) {
            if self.status_map.get(&node.value) == Some(&StatusCode::FoundOxygen) {
                return Some(node);
            }
        }
        None
    }
}

struct ShipExplorer<'a> {
    vm: VM<'a>,
    ship_map: ShipMap,
    route: Vec<Direction>,
    next_input: Rc<RefCell<RegisterValue>>,
}

impl<'a> ShipExplorer<'a> {
    fn new(mut vm: VM) -> ShipExplorer {
        let next_input = Rc::new(RefCell::new(0));
        let next_input_cloned = next_input.clone();
        vm.set_inputs(Some(Box::new(std::iter::from_fn(move || {
            Some(*next_input_cloned.borrow())
        }))));

        ShipExplorer {
            vm: vm,
            ship_map: ShipMap::new(),
            route: vec![],
            next_input: next_input,
        }
    }

    fn set_next_input(&mut self, direction: Direction) {
        *self.next_input.borrow_mut() = match direction {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        };
    }

    fn get_next_output(&mut self) -> Result<StatusCode> {
        Ok(StatusCode::try_from(u8::try_from(
            self.vm
                .outputs()
                .next()
                .ok_or(AOCError::new("Outputs exhausted"))?,
        )?)?)
    }

    fn get_next_position(&self, direction: Direction) -> Point {
        direction.next_position(self.ship_map.explorer_position)
    }

    fn try_move(&mut self, direction: Direction) -> Result<()> {
        self.set_next_input(direction);
        let status_code = self.get_next_output()?;
        let next_position = self.get_next_position(direction);
        self.ship_map.status_map.insert(next_position, status_code);
        match status_code {
            StatusCode::Moved | StatusCode::FoundOxygen => {
                self.ship_map.explorer_position = next_position;
                self.route.push(direction);
            }
            StatusCode::HitWall => (),
            StatusCode::Unexplored => Err(AOCError::new("Should never happen"))?,
        }
        Ok(())
    }
    fn backtrack(&mut self) -> Result<()> {
        let direction = self
            .route
            .pop()
            .ok_or(AOCError::new("No route to backtrack"))?
            .reverse();
        self.set_next_input(direction);
        self.ship_map.explorer_position = self.get_next_position(direction);
        if &self.get_next_output()? != self.ship_map.get_status(&self.ship_map.explorer_position) {
            return Err(AOCError::new("Different status on backtrack"))?;
        }
        Ok(())
    }

    fn explore(&mut self) -> Result<()> {
        loop {
            let mut unexplored_directions = Direction::iterator().filter(|&d| {
                self.ship_map.get_status(&self.get_next_position(d)) == &StatusCode::Unexplored
            });
            if let Some(direction) = unexplored_directions.next() {
                self.try_move(direction)?;
            } else if self.route.len() > 0 {
                self.backtrack()?;
            } else {
                break;
            }
        }
        Ok(())
    }
}

fn part1(ship_map: &ShipMap) -> Result<u64> {
    Ok(ship_map
        .find_oxygen(Point { x: 0, y: 0 })
        .ok_or(AOCError::new("Oxygen not found"))?
        .depth)
}

fn part2(ship_map: &ShipMap) -> Result<u64> {
    let start_position = ship_map
        .find_oxygen(Point { x: 0, y: 0 })
        .ok_or(AOCError::new("Oxygen not found"))?
        .value;

    Ok(graphs::bfs(ship_map, start_position)
        .last()
        .map(|node| node.depth)
        .ok_or(AOCError::new("No positions connected to start"))?)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program = aoc::intcode::parse_program(&input)?;
    let vm = VM::new(program.clone(), None);
    let mut explorer = ShipExplorer::new(vm);
    explorer.explore()?;
    let ship_map = explorer.ship_map;

    println!("{}", ship_map);
    println!("Part 1: {}", part1(&ship_map)?);
    println!("Part 2: {}", part2(&ship_map)?);
    Ok(())
}
