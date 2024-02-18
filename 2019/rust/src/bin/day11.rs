use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use aoc::direction::Direction;
use aoc::intcode::RegisterValue;
use aoc::point::Point2D;

#[cfg(test)]
use aoc::intcode::FakeVM as VM;
#[cfg(not(test))]
use aoc::intcode::VM;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Point = Point2D<i64>;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let program = aoc::intcode::parse_program(&input)?;

    println!("Part 1: {}", part1(&program));
    println!("Part 2: \n{}", part2(&program));
    Ok(())
}

fn part1(program: &[RegisterValue]) -> usize {
    let mut robot = Robot::new(program);
    let painted_locations = robot
        .instructions()
        .filter(|i| i.paint_white)
        .map(|i| i.paint_location)
        .collect::<HashSet<_>>();

    painted_locations.len()
}

fn part2(program: &[RegisterValue]) -> String {
    let mut robot = Robot::new(program);
    robot.paint_panel(Point { x: 0, y: 0 });
    for _ in robot.instructions() {}

    robot.panel_string()
}

#[derive(Debug, PartialEq, Eq)]
struct RobotInstruction {
    paint_location: Point,
    paint_white: bool,
}

struct Robot<'a> {
    vm: VM<'a>,
    position: Arc<Mutex<Point>>,
    direction: Direction,
    painted_panels: Arc<Mutex<HashSet<Point>>>,
}

impl<'a> Robot<'a> {
    fn new(program: &[RegisterValue]) -> Robot {
        let position = Arc::new(Mutex::new(Point { x: 0, y: 0 }));
        let painted_panels = Arc::new(Mutex::new(HashSet::new()));
        let input_iterator = RobotInputIterator {
            position: position.clone(),
            painted_panels: painted_panels.clone(),
        };
        let vm = VM::from_iterator(program.to_vec(), input_iterator);
        Robot {
            vm,
            position: position.clone(),
            direction: Direction::North,
            painted_panels: painted_panels.clone(),
        }
    }

    fn instructions<'b>(&'b mut self) -> RobotInstructionIterator<'b, 'a> {
        RobotInstructionIterator { robot: self }
    }

    fn move_robot(&mut self) {
        *self.position.lock().unwrap() += self.direction.offset().into();
    }

    fn paint_panel(&mut self, point: Point) {
        self.painted_panels.lock().unwrap().insert(point);
    }

    fn panel_string(&self) -> String {
        let panels = self.painted_panels.lock().unwrap();
        if panels.len() == 0 {
            return "".to_string();
        }
        let min_x = panels.iter().map(|p| p.x).min().unwrap();
        let max_x = panels.iter().map(|p| p.x).max().unwrap();
        let min_y = panels.iter().map(|p| p.y).min().unwrap();
        let max_y = panels.iter().map(|p| p.y).max().unwrap();

        let panel_char = |x, y| match panels.contains(&Point { x, y }) {
            true => '█',
            false => ' ',
        };

        (min_y..=max_y)
            .rev()
            .map(|y| (min_x..=max_x).map(|x| panel_char(x, y)).collect())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

struct RobotInputIterator {
    position: Arc<Mutex<Point>>,
    painted_panels: Arc<Mutex<HashSet<Point>>>,
}

impl Iterator for RobotInputIterator {
    type Item = RegisterValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self
            .painted_panels
            .lock()
            .unwrap()
            .contains(&self.position.lock().unwrap())
        {
            Some(1)
        } else {
            Some(0)
        }
    }
}

struct RobotInstructionIterator<'a, 'b> {
    robot: &'a mut Robot<'b>,
}

impl<'a, 'b> Iterator for RobotInstructionIterator<'a, 'b> {
    type Item = RobotInstruction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut outputs = self.robot.vm.outputs();
        if let (Some(paint_white), Some(turn_right)) = (outputs.next(), outputs.next()) {
            drop(outputs);
            if paint_white != 0 {
                self.robot
                    .painted_panels
                    .lock()
                    .unwrap()
                    .insert(*self.robot.position.lock().unwrap());
            } else {
                self.robot
                    .painted_panels
                    .lock()
                    .unwrap()
                    .remove(&self.robot.position.lock().unwrap());
            }
            if turn_right != 0 {
                self.robot.direction = self.robot.direction.right_turn();
            } else {
                self.robot.direction = self.robot.direction.left_turn();
            }

            let paint_location = *self.robot.position.lock().unwrap();
            self.robot.move_robot();

            return Some(RobotInstruction {
                paint_location,
                paint_white: paint_white != 0,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_instruction(x: RegisterValue, y: RegisterValue, paint_white: bool) -> RobotInstruction {
        RobotInstruction {
            paint_location: Point { x, y },
            paint_white,
        }
    }

    #[test]
    fn test_case() {
        let program = vec![];
        let mut robot = Robot::new(&program);
        robot
            .vm
            .set_outputs(vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0]);
        let mut instructions = robot.instructions();

        assert_eq!(instructions.next(), Some(make_instruction(0, 0, true)));
        assert_eq!(instructions.next(), Some(make_instruction(-1, 0, false)));
        assert_eq!(instructions.next(), Some(make_instruction(-1, -1, true)));
        assert_eq!(instructions.next(), Some(make_instruction(0, -1, true)));
        assert_eq!(instructions.next(), Some(make_instruction(0, 0, false)));
        assert_eq!(instructions.next(), Some(make_instruction(1, 0, true)));
        assert_eq!(instructions.next(), Some(make_instruction(1, 1, true)));
    }
}
