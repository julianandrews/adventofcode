use anyhow::{anyhow, Result};
use rustc_hash::FxHashMap;

use aoc::planar::{Direction, TileMap};
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: LoopMap = input.trim().parse()?;

    println!("Part 1: {}", part1(&map)?);
    println!("Part 2: {}", part2(&map)?);

    Ok(())
}

fn part1(map: &LoopMap) -> Result<u64> {
    Ok(map.loop_length().ok_or(anyhow!("Loop not found"))? / 2)
}

fn part2(map: &LoopMap) -> Result<u64> {
    map.enclosed_count().ok_or(anyhow!("Loop not found"))
}

struct LoopMap {
    map: TileMap<Pipe>,
    start_point: (usize, usize),
    start_pipe: Pipe,
}

impl LoopMap {
    fn loop_length(&self) -> Option<u64> {
        for (count, (point, _)) in self.walk().enumerate() {
            if point == self.start_point {
                return Some(count as u64 + 1);
            }
        }
        None
    }

    fn enclosed_count(&self) -> Option<u64> {
        let mut loop_points = FxHashMap::default();
        let mut points = self.walk();
        loop {
            let ((x, y), pipe) = points.next()?;
            loop_points.insert((x, y), pipe);
            if (x, y) == self.start_point {
                break;
            }
        }

        let mut count = 0;
        for y in 0..self.map.height() {
            let mut edge_parity = 0;
            for x in 0..self.map.width() {
                match loop_points.get(&(x, y)) {
                    None => {
                        if edge_parity % 4 != 0 {
                            count += 1;
                        }
                    }
                    Some(pipe) => edge_parity += pipe.edge_parity(),
                }
            }
        }
        Some(count)
    }

    fn walk(&self) -> impl Iterator<Item = ((usize, usize), Pipe)> + '_ {
        let (mut x, mut y) = self.start_point;
        let mut from = self.start_pipe.starting_direction();
        std::iter::from_fn(move || {
            let to = self.get(x, y)?.exit(from)?;
            (x, y) = self.map.step(x, y, to)?;
            from = to.reverse();
            let pipe = self.get(x, y)?;
            Some(((x, y), pipe))
        })
    }

    fn get(&self, x: usize, y: usize) -> Option<Pipe> {
        match self.map.get(x, y)? {
            Pipe::Start => Some(self.start_pipe),
            &pipe => Some(pipe),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Start,
    Ground,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Pipe {
    fn starting_direction(&self) -> Direction {
        match self {
            Pipe::NS | Pipe::SE => Direction::South,
            Pipe::EW | Pipe::SW => Direction::West,
            Pipe::NE => Direction::East,
            Pipe::NW | Pipe::Start | Pipe::Ground => Direction::North,
        }
    }

    fn missing_piece(map: &TileMap<Pipe>, x: usize, y: usize) -> Result<Pipe> {
        fn dir_open(map: &TileMap<Pipe>, x: usize, y: usize, direction: Direction) -> bool {
            if let Some(pipe) = map.step(x, y, direction).and_then(|(x, y)| map.get(x, y)) {
                Direction::iterator()
                    .filter(|&dir| pipe.exit(dir).is_some())
                    .any(|d| d == direction.reverse())
            } else {
                false
            }
        }
        let north = dir_open(map, x, y, Direction::North);
        let south = dir_open(map, x, y, Direction::South);
        let east = dir_open(map, x, y, Direction::East);
        let west = dir_open(map, x, y, Direction::West);
        match (north, south, east, west) {
            (true, true, false, false) => Ok(Pipe::NS),
            (true, false, true, false) => Ok(Pipe::NE),
            (true, false, false, true) => Ok(Pipe::NW),
            (false, true, true, false) => Ok(Pipe::SE),
            (false, true, false, true) => Ok(Pipe::SW),
            (false, false, true, true) => Ok(Pipe::SE),
            _ => Err(anyhow!("Can't identify unique starting pipe.")),
        }
    }

    fn exit(&self, from: Direction) -> Option<Direction> {
        use Direction::*;
        match (self, from) {
            (Pipe::NS, North) => Some(South),
            (Pipe::NS, South) => Some(North),
            (Pipe::EW, East) => Some(West),
            (Pipe::EW, West) => Some(East),
            (Pipe::NE, North) => Some(East),
            (Pipe::NE, East) => Some(North),
            (Pipe::NW, North) => Some(West),
            (Pipe::NW, West) => Some(North),
            (Pipe::SW, South) => Some(West),
            (Pipe::SW, West) => Some(South),
            (Pipe::SE, East) => Some(South),
            (Pipe::SE, South) => Some(East),
            _ => None,
        }
    }

    fn edge_parity(&self) -> i64 {
        match self {
            Pipe::NS => 2,
            Pipe::NE => 1,
            Pipe::NW => -1,
            Pipe::SW => 1,
            Pipe::SE => -1,
            _ => 0,
        }
    }
}

impl std::str::FromStr for LoopMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: TileMap<Pipe> = s.parse()?;
        let (x, y) = map
            .iter_coords()
            .find(|&(x, y)| map.get(x, y) == Some(&Pipe::Start))
            .ok_or(anyhow!("No start found on map"))?;
        let start_pipe = Pipe::missing_piece(&map, x, y)?;
        Ok(LoopMap {
            map,
            start_point: (x, y),
            start_pipe,
        })
    }
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Pipe::Start),
            '.' => Ok(Pipe::Ground),
            '|' => Ok(Pipe::NS),
            '-' => Ok(Pipe::EW),
            'L' => Ok(Pipe::NE),
            'J' => Ok(Pipe::NW),
            '7' => Ok(Pipe::SW),
            'F' => Ok(Pipe::SE),
            _ => Err(anyhow!("Unrecognized pipe section: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_length_1() {
        let test_data = "\
            -L|F7\n\
            7S-7|\n\
            L|7||\n\
            -L-J|\n\
            L|-JF";

        let map: LoopMap = test_data.parse().unwrap();
        assert_eq!(map.loop_length(), Some(8));
    }

    #[test]
    fn loop_length_2() {
        let test_data = "\
            7-F7-\n\
            .FJ|7\n\
            SJLL7\n\
            |F--J\n\
            LJ.LJ";
        let map: LoopMap = test_data.parse().unwrap();
        assert_eq!(map.loop_length(), Some(16));
    }

    #[test]
    fn enclosed_1() {
        let test_data = "\
            ...........\n\
            .S-------7.\n\
            .|F-----7|.\n\
            .||.....||.\n\
            .||.....||.\n\
            .|L-7.F-J|.\n\
            .|..|.|..|.\n\
            .L--J.L--J.\n\
            ...........";
        let map: LoopMap = test_data.parse().unwrap();
        assert_eq!(map.enclosed_count(), Some(4));
    }

    #[test]
    fn enclosed_2() {
        let test_data = "\
            .F----7F7F7F7F-7....\n\
            .|F--7||||||||FJ....\n\
            .||.FJ||||||||L7....\n\
            FJL7L7LJLJ||LJ.L-7..\n\
            L--J.L7...LJS7F-7L7.\n\
            ....F-J..F7FJ|L7L7L7\n\
            ....L7.F7||L7|.L7L7|\n\
            .....|FJLJ|FJ|F7|.LJ\n\
            ....FJL-7.||.||||...\n\
            ....L---J.LJ.LJLJ...";
        let map: LoopMap = test_data.parse().unwrap();
        assert_eq!(map.enclosed_count(), Some(8));
    }

    #[test]
    fn enclosed_3() {
        let test_data = "\
            FF7FSF7F7F7F7F7F---7\n\
            L|LJ||||||||||||F--J\n\
            FL-7LJLJ||||||LJL-77\n\
            F--JF--7||LJLJ7F7FJ-\n\
            L---JF-JLJ.||-FJLJJ7\n\
            |F|F-JF---7F7-L7L|7|\n\
            |FFJF7L7F-JF7|JL---7\n\
            7-L-JL7||F7|L7F-7F7|\n\
            L.L7LFJ|||||FJL7||LJ\n\
            L7JLJL-JLJLJL--JLJ.L";

        let map: LoopMap = test_data.parse().unwrap();
        assert_eq!(map.enclosed_count(), Some(10));
    }
}
