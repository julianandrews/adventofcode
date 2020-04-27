use aoc::direction::Direction;
use aoc::intcode::RegisterValue;
use aoc::point::Point2D;
use std::cell::RefCell;
use std::collections::HashSet;
use std::io::{self, Read};
use std::rc::Rc;

#[cfg(test)]
use aoc::intcode::FakeVM as VM;
#[cfg(not(test))]
use aoc::intcode::VM;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Point = Point2D<i64>;

#[derive(Debug, PartialEq, Eq)]
struct RobotInstruction {
    paint_location: Point,
    paint_white: bool,
}

struct Robot<'a> {
    vm: VM<'a>,
    position: Rc<RefCell<Point>>,
    direction: Direction,
    painted_panels: Rc<RefCell<HashSet<Point>>>,
}

impl<'a> Robot<'a> {
    fn new(vm: VM) -> Robot {
        let position = Rc::new(RefCell::new(Point { x: 0, y: 0 }));
        let painted_panels = Rc::new(RefCell::new(HashSet::new()));
        let mut robot = Robot {
            vm: vm,
            position: position.clone(),
            direction: Direction::North,
            painted_panels: painted_panels.clone(),
        };

        let input_iterator = RobotInputIterator {
            position: position,
            painted_panels: painted_panels,
        };
        robot.vm.set_inputs(Some(Box::new(input_iterator)));

        robot
    }

    fn instructions<'b>(&'b mut self) -> RobotInstructionIterator<'b, 'a> {
        RobotInstructionIterator { robot: self }
    }

    fn move_robot(&mut self) {
        *self.position.borrow_mut() += self.direction.offset().into();
    }

    fn paint_panel(&mut self, point: Point) {
        self.painted_panels.borrow_mut().insert(point);
    }

    fn panel_string(&self) -> String {
        let panels = self.painted_panels.borrow();
        if panels.len() == 0 {
            return "".to_string();
        }
        let min_x = panels.iter().map(|p| p.x).min().unwrap();
        let max_x = panels.iter().map(|p| p.x).max().unwrap();
        let min_y = panels.iter().map(|p| p.y).min().unwrap();
        let max_y = panels.iter().map(|p| p.y).max().unwrap();

        let panel_char = |x, y| match panels.contains(&Point { x: x, y: y }) {
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
    position: Rc<RefCell<Point>>,
    painted_panels: Rc<RefCell<HashSet<Point>>>,
}

impl Iterator for RobotInputIterator {
    type Item = RegisterValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self
            .painted_panels
            .borrow()
            .contains(&self.position.borrow())
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
                    .borrow_mut()
                    .insert(*self.robot.position.borrow());
            } else {
                self.robot
                    .painted_panels
                    .borrow_mut()
                    .remove(&self.robot.position.borrow());
            }
            if turn_right != 0 {
                self.robot.direction = self.robot.direction.right_turn();
            } else {
                self.robot.direction = self.robot.direction.left_turn();
            }

            let paint_location = self.robot.position.borrow().clone();
            self.robot.move_robot();

            return Some(RobotInstruction {
                paint_location: paint_location,
                paint_white: paint_white != 0,
            });
        }

        None
    }
}

fn part1(program: &Vec<RegisterValue>) -> usize {
    let vm = VM::new(program.clone(), None);
    let mut robot = Robot::new(vm);
    let painted_locations = robot
        .instructions()
        .filter(|i| i.paint_white)
        .map(|i| i.paint_location)
        .collect::<HashSet<_>>();

    painted_locations.len()
}

fn part2(program: &Vec<RegisterValue>) -> String {
    let vm = VM::new(program.clone(), None);
    let mut robot = Robot::new(vm);
    robot.paint_panel(Point { x: 0, y: 0 });
    for _ in robot.instructions() {}

    robot.panel_string()
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program = aoc::intcode::parse_program(&input)?;

    println!("Part 1: {}", part1(&program));
    println!("Part 2: \n{}", part2(&program));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_instruction(x: RegisterValue, y: RegisterValue, paint_white: bool) -> RobotInstruction {
        RobotInstruction {
            paint_location: Point { x: x, y: y },
            paint_white: paint_white,
        }
    }

    #[test]
    fn test_case() {
        let mut vm = VM::new(vec![], None);
        vm.set_outputs(vec![1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0]);
        let mut robot = Robot::new(vm);
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
