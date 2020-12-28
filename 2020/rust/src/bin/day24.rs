use std::collections::{HashMap, HashSet};

use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let directions_list: Vec<_> = parse_fields(input.trim(), '\n')?;

    println!("Part 1 {}", part1(&directions_list));
    println!("Part 2 {}", part2(&directions_list));

    Ok(())
}

fn part1(directions_list: &[Directions]) -> usize {
    let floor = Floor::from_directions(directions_list);
    floor.tile_count()
}

fn part2(directions_list: &[Directions]) -> usize {
    let mut floor = Floor::from_directions(directions_list);
    (0..100).for_each(|_| floor.evolve());
    floor.tile_count()
}

struct Floor {
    black_tiles: HashSet<HexCoordinate>,
}

impl Floor {
    fn from_directions(directions_list: &[Directions]) -> Self {
        let mut black_tiles = HashSet::new();
        for directions in directions_list {
            let tile = directions.destination();
            if black_tiles.contains(&tile) {
                black_tiles.remove(&tile);
            } else {
                black_tiles.insert(tile);
            }
        }
        Floor { black_tiles }
    }

    fn evolve(&mut self) {
        let mut neighbor_counts = HashMap::new();
        for tile in &self.black_tiles {
            neighbor_counts.entry(tile.clone()).or_insert(0);
            for neighbor in tile.neighbors() {
                *neighbor_counts.entry(neighbor).or_insert(0) += 1;
            }
        }
        for (tile, count) in neighbor_counts {
            if count == 2 {
                self.black_tiles.insert(tile);
            } else if count != 1 {
                self.black_tiles.remove(&tile);
            }
        }
    }

    fn tile_count(&self) -> usize {
        self.black_tiles.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash)]
struct HexCoordinate {
    q: i64,
    r: i64,
}

impl HexCoordinate {
    fn new(q: i64, r: i64) -> Self {
        HexCoordinate { q, r }
    }

    fn neighbors<'a>(&'a self) -> impl Iterator<Item = HexCoordinate> + 'a {
        [
            HexDirection::East,
            HexDirection::SouthEast,
            HexDirection::SouthWest,
            HexDirection::West,
            HexDirection::NorthWest,
            HexDirection::NorthEast,
        ]
        .iter()
        .map(move |&direction| self.neighbor(direction))
    }

    fn neighbor(&self, direction: HexDirection) -> Self {
        match direction {
            HexDirection::East => HexCoordinate {
                q: self.q + 1,
                r: self.r,
            },
            HexDirection::SouthEast => HexCoordinate {
                q: self.q,
                r: self.r + 1,
            },
            HexDirection::SouthWest => HexCoordinate {
                q: self.q - 1,
                r: self.r + 1,
            },
            HexDirection::West => HexCoordinate {
                q: self.q - 1,
                r: self.r,
            },
            HexDirection::NorthWest => HexCoordinate {
                q: self.q,
                r: self.r - 1,
            },
            HexDirection::NorthEast => HexCoordinate {
                q: self.q + 1,
                r: self.r - 1,
            },
        }
    }
}

struct Directions {
    directions: Vec<HexDirection>,
}

impl Directions {
    fn destination(&self) -> HexCoordinate {
        self.directions
            .iter()
            .fold(HexCoordinate::new(0, 0), |coord, direction| {
                coord.neighbor(*direction)
            })
    }
}

impl std::str::FromStr for Directions {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        if !s.is_ascii() {
            return Err(AOCError::new("Invalid input").into());
        }
        let s = s.as_bytes();
        let mut directions = vec![];
        let mut i = 0;
        while i < s.len() {
            let direction = match s[i] {
                b'e' => HexDirection::East,
                b'w' => HexDirection::West,
                b'n' => {
                    i += 1;
                    match s[i] {
                        b'e' => HexDirection::NorthEast,
                        b'w' => HexDirection::NorthWest,
                        _ => return Err(AOCError::new("Invalid input").into()),
                    }
                }
                b's' => {
                    i += 1;
                    match s[i] {
                        b'e' => HexDirection::SouthEast,
                        b'w' => HexDirection::SouthWest,
                        _ => return Err(AOCError::new("Invalid input").into()),
                    }
                }
                _ => return Err(AOCError::new("Invalid input").into()),
            };
            directions.push(direction);
            i += 1;
        }

        Ok(Directions { directions })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HexDirection {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "sesenwnenenewseeswwswswwnenewsewsw\
                                     \nneeenesenwnwwswnenewnwwsewnenwseswesw\
                                     \nseswneswswsenwwnwse\
                                     \nnwnwneseeswswnenewneswwnewseswneseene\
                                     \nswweswneswnenwsewnwneneseenw\
                                     \neesenwseswswnenwswnwnwsewwnwsene\
                                     \nsewnenenenesenwsewnenwwwse\
                                     \nwenwwweseeeweswwwnwwe\
                                     \nwsweesenenewnwwnwsenewsenwwsesesenwne\
                                     \nneeswseenwwswnwswswnw\
                                     \nnenwswwsewswnenenewsenwsenwnesesenew\
                                     \nenewnwewneswsewnwswenweswnenwsenwsw\
                                     \nsweneswneswneneenwnewenewwneswswnese\
                                     \nswwesenesewenwneswnwwneseswwne\
                                     \nenesenwswwswneneswsenwnewswseenwsese\
                                     \nwnwnesenesenenwwnenwsewesewsesesew\
                                     \nnenewswnwewswnenesenwnesewesw\
                                     \neneswnwswnwsenenwnwnwwseeswneewsenese\
                                     \nneswnwewnwnwseenwseesewsenwsweewe\
                                     \nwseweeenwnesenwwwswnew";

    #[test]
    fn simple_case() {
        let directions: Directions = "esenee".parse().unwrap();
        assert_eq!(
            directions.directions,
            vec![
                HexDirection::East,
                HexDirection::SouthEast,
                HexDirection::NorthEast,
                HexDirection::East
            ]
        );
        assert_eq!(directions.destination(), HexCoordinate::new(3, 0));
    }

    #[test]
    fn larger_example() {
        let directions_list: Vec<_> = parse_fields(TEST_INPUT, '\n').unwrap();
        let floor = Floor::from_directions(&directions_list);
        assert_eq!(floor.tile_count(), 10);
    }

    #[test]
    fn evolve() {
        let directions_list: Vec<_> = parse_fields(TEST_INPUT, '\n').unwrap();
        let mut floor = Floor::from_directions(&directions_list);
        let mut tile_counts = vec![floor.tile_count()];
        for _ in 0..100 {
            floor.evolve();
            tile_counts.push(floor.tile_count());
        }

        assert_eq!(
            &tile_counts[..=10],
            vec![10, 15, 12, 25, 14, 23, 28, 41, 37, 49, 37]
        );
        assert_eq!(tile_counts[20], 132);
        assert_eq!(tile_counts[30], 259);
        assert_eq!(tile_counts[40], 406);
        assert_eq!(tile_counts[50], 566);
        assert_eq!(tile_counts[60], 788);
        assert_eq!(tile_counts[70], 1106);
        assert_eq!(tile_counts[80], 1373);
        assert_eq!(tile_counts[90], 1844);
        assert_eq!(tile_counts[100], 2208);
    }
}
