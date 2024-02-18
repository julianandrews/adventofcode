use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt;

use anyhow::{anyhow, bail, Result};

use aoc::direction::Direction;
use aoc::intcode::{RegisterValue, VM};
use aoc::point::Point2D;

type Point = Point2D<i64>;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let program = aoc::intcode::parse_program(&input)?;

    println!("Part 1: {}", part1(&program)?);
    println!("Part 2: {}", part2(&program)?);

    Ok(())
}

fn part1(program: &[RegisterValue]) -> Result<usize> {
    let scaffold = Scaffold::from_program(program)?;
    Ok(scaffold
        .intersections()
        .filter_map(|p| scaffold.alignment_parameter(p))
        .sum())
}

fn part2(program: &[RegisterValue]) -> Result<RegisterValue> {
    let scaffold = Scaffold::from_program(program)?;
    let instructions = scaffold.full_instructions()?;
    let (func_a, func_b, func_c) =
        get_functions(20, &instructions).ok_or(anyhow!("No functions found for routine"))?;
    let main_routine = instructions
        .replace(func_a, "A")
        .replace(func_b, "B")
        .replace(func_c, "C");
    let input_string = [
        main_routine,
        func_a.to_string(),
        func_b.to_string(),
        func_c.to_string(),
        "n".to_string(),
        "".to_string(),
    ]
    .join("\n");
    let inputs = input_string.chars().map(|c| c as RegisterValue);
    let mut vm = VM::from_iterator(program.to_vec(), inputs);
    vm.set_memory(0, 2);

    vm.outputs().last().ok_or(anyhow!("No output generated"))
}

#[derive(Clone, Copy)]
enum ScaffoldTile {
    Space,
    Scaffold,
    Vacuum(Direction),
}

impl TryFrom<char> for ScaffoldTile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(ScaffoldTile::Space),
            '#' => Ok(ScaffoldTile::Scaffold),
            '<' => Ok(ScaffoldTile::Vacuum(Direction::West)),
            '>' => Ok(ScaffoldTile::Vacuum(Direction::East)),
            '^' => Ok(ScaffoldTile::Vacuum(Direction::North)),
            'v' => Ok(ScaffoldTile::Vacuum(Direction::South)),
            _ => Err(anyhow!("Unrecognized map tile: {}", value)),
        }
    }
}

impl fmt::Display for ScaffoldTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            ScaffoldTile::Space => '.',
            ScaffoldTile::Scaffold => '#',
            ScaffoldTile::Vacuum(direction) => match direction {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
            },
        };

        write!(f, "{}", c)
    }
}

struct Scaffold {
    map: Box<[ScaffoldTile]>,
    width: usize,
    height: usize,
}

impl fmt::Display for Scaffold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(move |x| {
                        self.at(Point {
                            x: x as i64,
                            y: y as i64,
                        })
                        .to_string()
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

impl Scaffold {
    fn from_program(program: &[RegisterValue]) -> Result<Scaffold> {
        let mut vm = VM::from_iterator(program.to_vec(), std::iter::empty());
        let scaffold_string: String = vm
            .outputs()
            .map(|x| u8::try_from(x).map(|y| y as char))
            .collect::<std::result::Result<String, _>>()?;
        let scaffold = scaffold_string.trim().parse()?;
        Ok(scaffold)
    }

    fn at(&self, point: Point) -> ScaffoldTile {
        if point.x < 0
            || point.x as usize >= self.width
            || point.y < 0
            || point.y as usize >= self.height
        {
            return ScaffoldTile::Space;
        }
        self.map[point.y as usize * self.width + point.x as usize]
    }

    fn on_scaffold(&self, point: Point) -> bool {
        !matches!(self.at(point), ScaffoldTile::Space)
    }

    fn alignment_parameter(&self, point: Point) -> Option<usize> {
        if self.on_scaffold(point) {
            Some(point.x as usize * point.y as usize)
        } else {
            None
        }
    }

    fn neighbors(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        Direction::iterator().filter_map(move |d| {
            let p = Scaffold::step(point, d);
            if self.on_scaffold(p) {
                Some(p)
            } else {
                None
            }
        })
    }

    fn step(position: Point, direction: Direction) -> Point {
        match direction {
            Direction::North => Point {
                x: position.x,
                y: position.y - 1,
            },
            Direction::East => Point {
                x: position.x + 1,
                y: position.y,
            },
            Direction::South => Point {
                x: position.x,
                y: position.y + 1,
            },
            Direction::West => Point {
                x: position.x - 1,
                y: position.y,
            },
        }
    }

    fn intersections(&self) -> impl Iterator<Item = Point> + '_ {
        (1..self.height as i64 - 1).flat_map(move |y| {
            (1..self.width as i64 - 1)
                .filter_map(|x| {
                    let p = Point { x, y };
                    if self.on_scaffold(p) && self.neighbors(p).count() == 4 {
                        Some(p)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
    }

    fn vacuum_locations(&self) -> impl Iterator<Item = Point> + '_ {
        self.map
            .iter()
            .enumerate()
            .filter_map(move |(i, tile)| match tile {
                ScaffoldTile::Vacuum(_) => Some(Point {
                    x: (i % self.width) as i64,
                    y: (i / self.width) as i64,
                }),
                _ => None,
            })
    }

    fn full_instructions(&self) -> Result<String> {
        let points: Vec<Point> = self.vacuum_locations().collect();
        if points.len() != 1 {
            bail!("No unique vacuum_location found.");
        }
        let mut current_location = points[0];
        let mut instructions: Vec<String> = vec![];
        let mut current_direction = match self.at(current_location) {
            ScaffoldTile::Vacuum(d) => d,
            _ => bail!("Should not happen"),
        };
        let mut visited = HashSet::new();
        visited.insert(current_location);
        loop {
            let mut distance: usize = 0;
            while self.on_scaffold(Scaffold::step(current_location, current_direction)) {
                current_location = Scaffold::step(current_location, current_direction);
                visited.insert(current_location);
                distance += 1;
            }
            if distance > 0 {
                instructions.push(distance.to_string());
            } else {
                let mut candidates: HashSet<Direction> = Direction::iterator()
                    .filter(|d| self.on_scaffold(Scaffold::step(current_location, *d)))
                    .collect();
                candidates.remove(&current_direction.reverse());
                if candidates.is_empty() {
                    break;
                } else if candidates.len() > 1 {
                    bail!("Multiple paths found.");
                }
                let new_direction = candidates.into_iter().next().unwrap();
                if current_direction.right_turn() == new_direction {
                    instructions.push("R".to_string());
                } else {
                    instructions.push("L".to_string());
                }
                current_direction = new_direction;
            }
        }

        Ok(instructions.join(","))
    }
}

fn get_functions(max_func_length: usize, full_routine: &str) -> Option<(&str, &str, &str)> {
    for i in (0..max_func_length).rev() {
        if full_routine.as_bytes()[i] != b',' {
            continue;
        }
        let func_a = &full_routine[..i];

        // Drop any leading 'A,' ...
        let next_chunk = &full_routine
            .split(func_a)
            .map(|s| s.trim_start_matches(','))
            .find(|s| !s.is_empty())
            .unwrap_or("");
        // ... and truncate to max_func_length.
        let next_chunk = &next_chunk[..std::cmp::min(max_func_length, next_chunk.len())];

        for (j, c) in next_chunk.char_indices().rev() {
            if c != ',' {
                continue;
            }
            let func_b = &next_chunk[..j];
            let func_c = full_routine
                .split(func_a)
                .flat_map(|s| s.split(func_b).map(|s| s.trim_matches(',')))
                .find(|s| !s.is_empty())
                .unwrap_or("");
            if func_c.len() > max_func_length {
                continue;
            }
            let all_bits_covered = full_routine
                .split(func_a)
                .flat_map(|s| s.split(func_b))
                .flat_map(|s| s.split(func_c))
                .all(|s| s.trim_matches(',').is_empty());
            if all_bits_covered {
                return Some((func_a, func_b, func_c));
            }
        }
    }
    None
}

mod parsing {
    use super::{Scaffold, ScaffoldTile};

    use anyhow::{bail, Result};

    impl std::str::FromStr for Scaffold {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self> {
            let height = s.lines().count();
            let width = if height == 0 {
                0
            } else {
                s.lines().next().unwrap().len()
            };
            if s.lines().any(|line| line.len() != width) {
                bail!("Non-rectangular scaffold grid");
            }
            let map: Vec<ScaffoldTile> = s
                .lines()
                .flat_map(|line| line.chars().map(ScaffoldTile::try_from))
                .collect::<Result<_>>()?;

            let vacuum_count = map
                .iter()
                .filter(|tile| matches!(tile, ScaffoldTile::Vacuum(_)))
                .count();
            if vacuum_count != 1 {
                bail!("Multiple vacuums found.'");
            }

            Ok(Scaffold {
                map: map.into_boxed_slice(),
                width,
                height,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_parameter_sum() {
        let scaffold: Scaffold = "..#..........\n\
                                  ..#..........\n\
                                  #######...###\n\
                                  #.#...#...#.#\n\
                                  #############\n\
                                  ..#...#...#..\n\
                                  ..#####...^.."
            .parse()
            .unwrap();

        let result: usize = scaffold
            .intersections()
            .filter_map(|p| scaffold.alignment_parameter(p))
            .sum();

        assert_eq!(result, 76);
    }

    #[test]
    fn test_full_instructions() {
        let scaffold: Scaffold = "#######...#####\n\
                                  #.....#...#...#\n\
                                  #.....#...#...#\n\
                                  ......#...#...#\n\
                                  ......#...###.#\n\
                                  ......#.....#.#\n\
                                  ^########...#.#\n\
                                  ......#.#...#.#\n\
                                  ......#########\n\
                                  ........#...#..\n\
                                  ....#########..\n\
                                  ....#...#......\n\
                                  ....#...#......\n\
                                  ....#...#......\n\
                                  ....#####......"
            .parse()
            .unwrap();
        let result = scaffold.full_instructions().unwrap();
        assert_eq!(
            result,
            "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2",
        );
    }
}
