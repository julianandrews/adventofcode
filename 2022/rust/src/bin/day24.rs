use std::collections::{HashSet, VecDeque};

use anyhow::{anyhow, bail, Result};

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: StormMap = input.trim().parse()?;

    println!("Part 1: {}", part1(&map)?);
    println!("Part 2: {}", part2(&map)?);

    Ok(())
}

fn part1(map: &StormMap) -> Result<usize> {
    map.navigate(0, Position::Entrance, Position::Exit)
        .ok_or_else(|| anyhow!("Failed to find path to exit"))
}

fn part2(map: &StormMap) -> Result<usize> {
    let time = map
        .navigate(0, Position::Entrance, Position::Exit)
        .ok_or_else(|| anyhow!("Failed to find path to exit"))?;
    let time = map
        .navigate(time, Position::Exit, Position::Entrance)
        .ok_or_else(|| anyhow!("Failed to find return path to entrance"))?;
    map.navigate(time, Position::Entrance, Position::Exit)
        .ok_or_else(|| anyhow!("Failed to find return path to exit"))
}

#[derive(Debug, Clone)]
struct StormMap {
    entrance: usize,
    exit: usize,
    width: usize,
    height: usize,
    north_bound: Vec<u128>,
    east_bound: Vec<u128>,
    south_bound: Vec<u128>,
    west_bound: Vec<u128>,
}

impl StormMap {
    fn navigate(&self, time: usize, start: Position, destination: Position) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back(Node {
            position: start,
            time,
        });

        while let Some(node) = to_visit.pop_front() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);

            for n in self.neighbors(&node) {
                if n.position == destination {
                    return Some(n.time);
                }
                to_visit.push_back(n);
            }
        }
        None
    }

    fn neighbors(&self, node: &Node) -> impl Iterator<Item = Node> + '_ {
        let mut neighbors = vec![];
        match node.position {
            Position::Entrance => neighbors.push(Position::Position((self.entrance, 0))),
            Position::Exit => neighbors.push(Position::Position((self.exit, self.height - 1))),
            Position::Position((x, y)) => {
                if y == 0 && x == self.entrance {
                    neighbors.push(Position::Entrance);
                }
                if y == self.height - 1 && x == self.exit {
                    neighbors.push(Position::Exit);
                }
                if x > 0 {
                    neighbors.push(Position::Position((x - 1, y)));
                }
                if x < self.width - 1 {
                    neighbors.push(Position::Position((x + 1, y)));
                }
                if y > 0 {
                    neighbors.push(Position::Position((x, y - 1)));
                }
                if y < self.height - 1 {
                    neighbors.push(Position::Position((x, y + 1)));
                }
            }
        }
        // Waiting is always an option
        neighbors.push(node.position);

        let time = node.time + 1;
        neighbors
            .into_iter()
            .map(move |position| Node { position, time })
            .filter(|node| self.is_clear(node.position, node.time))
    }

    fn is_clear(&self, position: Position, time: usize) -> bool {
        let (x, y) = match position {
            Position::Position((x, y)) => (x, y),
            _ => return true,
        };

        let mod_add = |a, b, m| (a + b) % m;
        let mod_sub = |a, b, m| (a + m - b % m) % m;
        let north_clear = self.north_bound[mod_add(y, time, self.height)] & (1 << x) == 0;
        let south_clear = self.south_bound[mod_sub(y, time, self.height)] & (1 << x) == 0;
        let east_clear = self.east_bound[mod_sub(x, time, self.width)] & (1 << y) == 0;
        let west_clear = self.west_bound[mod_add(x, time, self.width)] & (1 << y) == 0;

        north_clear && south_clear && east_clear && west_clear
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    position: Position,
    time: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Position {
    Entrance,
    Exit,
    Position((usize, usize)),
}

impl std::str::FromStr for StormMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let line_width = lines[0].len();
        if lines.len() < 3 || line_width < 3 {
            bail!("Invalid storm map - too small");
        }
        if lines.len() > 130 || line_width > 130 {
            bail!("Invalid storm map - too large");
        }
        if lines.iter().any(|line| line.len() != line_width) {
            bail!("Invalid storm map - not rectangular");
        }
        if lines
            .iter()
            .any(|line| !(line.starts_with('#') && line.ends_with('#')))
        {
            bail!("Invalid storm map - incorrect border");
        }

        let width = line_width - 2;
        let height = lines.len() - 2;
        let entrance = lines[0]
            .chars()
            .position(|c| c == '.')
            .ok_or_else(|| anyhow!("Failed to find entrance"))?
            - 1;
        let exit = lines[lines.len() - 1]
            .chars()
            .position(|c| c == '.')
            .ok_or_else(|| anyhow!("Failed to find exit"))?
            - 1;

        let mut north_bound = vec![0; height];
        let mut east_bound = vec![0; width];
        let mut south_bound = vec![0; height];
        let mut west_bound = vec![0; width];

        for (y, line) in lines[1..lines.len() - 1].iter().enumerate() {
            for (x, b) in line.as_bytes()[1..line.len() - 1].iter().enumerate() {
                match b {
                    b'.' => {}
                    b'^' => north_bound[y] |= 1 << x,
                    b'>' => east_bound[x] |= 1 << y,
                    b'v' => south_bound[y] |= 1 << x,
                    b'<' => west_bound[x] |= 1 << y,
                    _ => bail!("Unexpected character in {}", line),
                }
            }
        }

        Ok(Self {
            entrance,
            exit,
            width,
            height,
            north_bound,
            east_bound,
            south_bound,
            west_bound,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    static SIMPLE_MAP: &str = "\
        #.#####\n\
        #.....#\n\
        #>....#\n\
        #.....#\n\
        #...v.#\n\
        #.....#\n\
        #####.#";

    static SIMPLE_MAP_STORM_POINTS: [(usize, usize, usize); 12] = [
        (0, 1, 0),
        (3, 3, 0),
        (1, 1, 1),
        (3, 4, 1),
        (2, 1, 2),
        (3, 0, 2),
        (3, 1, 3),
        (3, 1, 3),
        (4, 1, 4),
        (3, 2, 4),
        (0, 1, 5),
        (3, 3, 5),
    ];

    static TEST_MAP: &str = "\
        #.######\n\
        #>>.<^<#\n\
        #.<..<<#\n\
        #>v.><>#\n\
        #<^v^^>#\n\
        ######.#";

    #[test]
    fn open_tiles() {
        let map: StormMap = SIMPLE_MAP.parse().unwrap();
        let storm_points: HashSet<_> = SIMPLE_MAP_STORM_POINTS.into_iter().collect();
        for t in 0..=5 {
            for x in 0..5 {
                for y in 0..5 {
                    let p = Position::Position((x, y));
                    if storm_points.contains(&(x, y, t)) {
                        assert!(!map.is_clear(p, t), "Open: x={}, y={}, t={}", x, y, t);
                    } else {
                        assert!(map.is_clear(p, t), "Storm: x={}, y={}, t={}", x, y, t);
                    }
                }
            }
        }
    }

    #[test]
    fn entrance_to_exit() {
        let map: StormMap = TEST_MAP.parse().unwrap();
        let time = map.navigate(0, Position::Entrance, Position::Exit).unwrap();
        assert_eq!(time, 18);
    }

    #[test]
    fn return_trip() {
        let map: StormMap = TEST_MAP.parse().unwrap();
        let time = map
            .navigate(18, Position::Exit, Position::Entrance)
            .unwrap();
        assert_eq!(time, 41);
    }

    #[test]
    fn final_exit() {
        let map: StormMap = TEST_MAP.parse().unwrap();
        let time = map
            .navigate(41, Position::Entrance, Position::Exit)
            .unwrap();
        assert_eq!(time, 54);
    }
}
