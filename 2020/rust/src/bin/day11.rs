use aoc::aoc_error::AOCError;
use aoc::tile_map::{TileMap, DIRECTIONS};
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type NeighborCounter = fn(&SeatMap, i64, i64) -> usize;
type SeatMap = TileMap<TileType>;

fn main() -> Result<()> {
    let input = get_input()?;
    let seat_map: SeatMap = input.parse()?;

    println!("Part 1: {}", part1(&seat_map));
    println!("Part 2: {}", part2(&seat_map));
    Ok(())
}

fn part1(seat_map: &SeatMap) -> usize {
    let count_neighbors = |map: &SeatMap, x, y| {
        map.neighbors(x, y)
            .into_iter()
            .filter(|&&tile| tile == TileType::Occupied)
            .count()
    };
    find_steady_state(seat_map, count_neighbors, 4).variant_count(&TileType::Occupied)
}

fn part2(seat_map: &SeatMap) -> usize {
    let count_neighbors = |map: &SeatMap, x, y| {
        DIRECTIONS
            .iter()
            .filter(|(dx, dy)| {
                let (mut nx, mut ny) = (x, y);
                loop {
                    nx += dx;
                    ny += dy;
                    match map.get_tile(nx, ny) {
                        Some(TileType::Floor) => continue,
                        Some(TileType::Occupied) => return true,
                        _ => return false,
                    }
                }
            })
            .count()
    };
    find_steady_state(seat_map, count_neighbors, 5).variant_count(&TileType::Occupied)
}

fn find_steady_state(
    seat_map: &SeatMap,
    count_neighbors: NeighborCounter,
    leave_threshold: usize,
) -> SeatMap {
    let mut old_seat_map = (*seat_map).clone();
    let mut seat_map = step(&old_seat_map, count_neighbors, leave_threshold);
    while seat_map != old_seat_map {
        old_seat_map = seat_map;
        seat_map = step(&old_seat_map, count_neighbors, leave_threshold);
    }

    seat_map
}

fn step(seat_map: &SeatMap, count_neighbors: NeighborCounter, leave_threshold: usize) -> SeatMap {
    let next_tile = |x, y| match seat_map.get_tile(x, y).unwrap() {
        TileType::Floor => TileType::Floor,
        TileType::Empty => {
            if count_neighbors(seat_map, x, y) == 0 {
                TileType::Occupied
            } else {
                TileType::Empty
            }
        }
        TileType::Occupied => {
            if count_neighbors(seat_map, x, y) >= leave_threshold {
                TileType::Empty
            } else {
                TileType::Occupied
            }
        }
    };

    let rows = seat_map
        .rows
        .iter()
        .enumerate()
        .map(|(y, row)| {
            (0..row.len())
                .map(|x| next_tile(x as i64, y as i64))
                .collect()
        })
        .collect();
    SeatMap { rows }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Floor,
    Empty,
    Occupied,
}

impl std::convert::TryFrom<char> for TileType {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(TileType::Floor),
            'L' => Ok(TileType::Empty),
            '#' => Ok(TileType::Occupied),
            _ => Err(AOCError::new("Unrecognized tile").into()),
        }
    }
}

impl std::convert::From<TileType> for char {
    fn from(tile: TileType) -> Self {
        match tile {
            TileType::Floor => '.',
            TileType::Empty => 'L',
            TileType::Occupied => '#',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "#.##.##.##\
				     \n#######.##\
				     \n#.#.#..#..\
				     \n####.##.##\
				     \n#.##.##.##\
				     \n#.#####.##\
				     \n..#.#.....\
				     \n##########\
				     \n#.######.#\
				     \n#.#####.##";

    #[test]
    fn simple_count() {
        let seat_map: SeatMap = TEST_INPUT.parse().unwrap();
        assert_eq!(part1(&seat_map), 37);
    }

    #[test]
    fn real_count() {
        let seat_map: SeatMap = TEST_INPUT.parse().unwrap();
        assert_eq!(part2(&seat_map), 26);
    }
}
