#![feature(iterator_try_collect)]

use anyhow::Result;

use aoc::planar::{CardinalDirection, TileMap};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let (map, directions) = parsing::parse_input(input.trim())?;

    println!("Part 1: {}", part1(&map, &directions));
    println!("Part 2: {}", part2(&map, &directions));

    Ok(())
}

fn part1(map: &WarehouseMap, directions: &[CardinalDirection]) -> usize {
    let mut map = map.clone();
    map.walk(directions);
    map.gps_score()
}

fn part2(map: &WarehouseMap, directions: &[CardinalDirection]) -> usize {
    let mut map = map.widen();
    map.walk(directions);
    map.gps_score()
}

#[derive(Debug, Clone)]
struct WarehouseMap(TileMap<Tile>);

impl WarehouseMap {
    fn step(
        &mut self,
        direction: CardinalDirection,
        robot: (usize, usize),
    ) -> Option<(usize, usize)> {
        let mut points = vec![];
        let (mut x, mut y) = robot;
        while let Some((nx, ny)) = self.0.step(x, y, direction) {
            points.push((nx, ny));
            match self.0.get(nx, ny).unwrap() {
                Tile::Space => break,
                Tile::Wall => return None,
                Tile::Box => {}
                Tile::Robot => unreachable!(),
            }
            (x, y) = (nx, ny);
        }
        self.0.set(robot.0, robot.1, Tile::Space).unwrap();
        let mut tile = Tile::Robot;
        for &(x, y) in &points {
            tile = self.0.set(x, y, tile).unwrap();
        }
        points.first().copied()
    }

    fn walk(&mut self, directions: &[CardinalDirection]) {
        let mut robot = self.0.find(Tile::Robot).unwrap();
        for &direction in directions {
            if let Some(r) = self.step(direction, robot) {
                robot = r;
            }
        }
    }

    fn gps_score(&self) -> usize {
        self.0
            .iter_coords()
            .filter(|&(x, y)| self.0.get(x, y) == Some(&Tile::Box))
            .map(|(x, y)| x + 100 * y)
            .sum()
    }

    fn widen(&self) -> WideWarehouseMap {
        let width = self.0.width() * 2;
        let mut rows = vec![vec![WideTile::Space; width]; self.0.height()];
        for (x, y) in self.0.iter_coords() {
            let (t1, t2) = match *self.0.get(x, y).unwrap() {
                Tile::Space => (WideTile::Space, WideTile::Space),
                Tile::Wall => (WideTile::Wall, WideTile::Wall),
                Tile::Box => (WideTile::BoxLeft, WideTile::BoxRight),
                Tile::Robot => (WideTile::Robot, WideTile::Space),
            };
            rows[y][2 * x] = t1;
            rows[y][2 * x + 1] = t2;
        }
        WideWarehouseMap(TileMap { rows, width })
    }
}

#[derive(Debug, Clone)]
struct WideWarehouseMap(TileMap<WideTile>);

impl WideWarehouseMap {
    fn step(
        &mut self,
        direction: CardinalDirection,
        robot: (usize, usize),
    ) -> Option<(usize, usize)> {
        let moves = {
            use CardinalDirection::*;
            let mut moves = vec![];
            let mut to_visit = vec![robot];
            while let Some((x, y)) = to_visit.pop() {
                let (nx, ny) = self.0.step(x, y, direction).unwrap();
                let tile = self.0.get(nx, ny)?;
                moves.push(((x, y), (nx, ny)));
                match (tile, direction) {
                    (WideTile::Space, _) => continue,
                    (WideTile::Wall, _) => return None,
                    (WideTile::BoxLeft, North | South) => to_visit.push((nx + 1, ny)),
                    (WideTile::BoxRight, North | South) => to_visit.push((nx - 1, ny)),
                    (WideTile::Robot, _) => unreachable!(),
                    _ => {}
                }
                to_visit.push((nx, ny));
            }
            moves
        };
        let new_tiles: Vec<_> = moves
            .iter()
            .map(|&((x, y), to)| (to, *self.0.get(x, y).unwrap()))
            .collect();
        for &((x, y), _) in &moves {
            self.0.set(x, y, WideTile::Space);
        }
        for ((x, y), tile) in new_tiles {
            self.0.set(x, y, tile);
        }
        moves.first().map(|(_, to)| to).copied()
    }

    fn walk(&mut self, directions: &[CardinalDirection]) {
        let mut robot = self.0.find(WideTile::Robot).unwrap();
        for &direction in directions {
            if let Some(r) = self.step(direction, robot) {
                robot = r;
            }
        }
    }

    fn gps_score(&self) -> usize {
        self.0
            .iter_coords()
            .filter(|&(x, y)| self.0.get(x, y) == Some(&WideTile::BoxLeft))
            .map(|(x, y)| x + 100 * y)
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Space,
    Wall,
    Box,
    Robot,
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Space => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::Robot => '@',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WideTile {
    Space,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

impl From<&WideTile> for char {
    fn from(value: &WideTile) -> Self {
        match value {
            WideTile::Space => '.',
            WideTile::Wall => '#',
            WideTile::BoxLeft => '[',
            WideTile::BoxRight => ']',
            WideTile::Robot => '@',
        }
    }
}

mod parsing {
    use anyhow::{anyhow, bail, Result};

    use super::{CardinalDirection, Tile, TileMap, WarehouseMap};

    pub fn parse_input(input: &str) -> Result<(WarehouseMap, Vec<CardinalDirection>)> {
        let (map_part, directions_part) = input
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("Failed to split input."))?;
        let directions = directions_part
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| match c {
                '^' => Ok(CardinalDirection::North),
                '>' => Ok(CardinalDirection::East),
                'v' => Ok(CardinalDirection::South),
                '<' => Ok(CardinalDirection::West),
                _ => bail!("Unepected direction {}", c),
            })
            .try_collect()?;
        let map: TileMap<Tile> = map_part.parse()?;
        map.find(Tile::Robot)
            .ok_or_else(|| anyhow!("Failed to find robot"))?;
        Ok((WarehouseMap(map), directions))
    }

    impl TryFrom<char> for Tile {
        type Error = anyhow::Error;

        fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
            match value {
                '.' => Ok(Tile::Space),
                '#' => Ok(Tile::Wall),
                'O' => Ok(Tile::Box),
                '@' => Ok(Tile::Robot),
                _ => bail!("Unrecognized tile {}", value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SMALL_EXAMPLE: &str = "\
        ########\n\
        #..O.O.#\n\
        ##@.O..#\n\
        #...O..#\n\
        #.#.O..#\n\
        #...O..#\n\
        #......#\n\
        ########\n\
        \n\
        <^^>>>vv<v>>v<<";

    static FULL_EXAMPLE: &str = "\
        ##########\n\
        #..O..O.O#\n\
        #......O.#\n\
        #.OO..O.O#\n\
        #..O@..O.#\n\
        #O#..O...#\n\
        #O..O..O.#\n\
        #.OO.O.OO#\n\
        #....O...#\n\
        ##########\n\
        \n\
        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn small_example() {
        let (mut map, directions) = parsing::parse_input(SMALL_EXAMPLE).unwrap();
        map.walk(&directions);
        assert_eq!(map.gps_score(), 2028);
    }

    #[test]
    fn full_example() {
        let (mut map, directions) = parsing::parse_input(FULL_EXAMPLE).unwrap();
        map.walk(&directions);
        assert_eq!(map.gps_score(), 10092);
    }

    #[test]
    fn full_example_wide() {
        let (map, directions) = parsing::parse_input(FULL_EXAMPLE).unwrap();
        let mut map = map.widen();
        map.walk(&directions);
        assert_eq!(map.gps_score(), 9021);
    }
}
