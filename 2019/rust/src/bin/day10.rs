extern crate num_integer;

use aoc::aoc_error::AOCError;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::f64::consts::PI;
use std::io::{self, Read};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Point = aoc::point::Point2D<i64>;

enum MapTile {
    Full,
    Empty,
}

impl TryFrom<char> for MapTile {
    type Error = Box<dyn std::error::Error>;

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '#' => Ok(MapTile::Full),
            '.' => Ok(MapTile::Empty),
            _ => Err(AOCError::new("Unrecognized map tile"))?,
        }
    }
}

struct AsteroidField {
    grid: Vec<Vec<MapTile>>,
}

impl FromStr for AsteroidField {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| line.chars().map(MapTile::try_from).collect::<Result<_>>())
            .collect::<Result<Vec<Vec<MapTile>>>>()?;
        let width = if grid.len() > 0 { grid[0].len() } else { 0 };
        if grid.iter().any(|row| row.len() != width) {
            Err(AOCError::new("Non rectangular asteroid field input"))?;
        }

        Ok(AsteroidField { grid: grid })
    }
}

impl AsteroidField {
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        if self.grid.len() > 0 {
            self.grid[0].len()
        } else {
            0
        }
    }

    fn in_field(&self, location: &Point) -> bool {
        location.x >= 0
            && location.x < (self.width() as i64)
            && location.y >= 0
            && location.y < (self.height() as i64)
    }

    fn has_asteroid(&self, location: &Point) -> bool {
        if let Some(row) = self.grid.get(location.y as usize) {
            if let Some(tile) = row.get(location.x as usize) {
                match tile {
                    MapTile::Full => return true,
                    MapTile::Empty => return false,
                }
            }
        }

        false
    }

    fn get_directions(&self, location: &Point) -> Vec<Point> {
        let mut direction_set = HashSet::new();
        for x in 0..self.width() {
            let dx = (x as i64) - location.x;
            for y in 0..self.height() {
                let dy = (y as i64) - location.y;
                let denom = num_integer::gcd(dx, dy);
                if dx != 0 || dy != 0 {
                    direction_set.insert(Point {
                        x: dx / denom,
                        y: dy / denom,
                    });
                }
            }
        }

        let mut directions = direction_set.into_iter().collect::<Vec<_>>();

        let key = |p: &Point| (-(p.x as f64).atan2(p.y as f64) + PI) % (2.0 * PI);
        directions.sort_by(|a, b| key(a).partial_cmp(&key(b)).unwrap_or(Ordering::Equal));

        directions
    }

    fn first_visible_asteroid(&self, location: &Point, direction: &Point) -> Option<Point> {
        let mut point = *location;
        while self.in_field(&point) {
            point.x += direction.x;
            point.y += direction.y;
            if self.has_asteroid(&point) {
                return Some(point);
            }
        }

        None
    }

    fn visible_count(&self, location: &Point) -> usize {
        self.get_directions(&location)
            .iter()
            .filter(|&d| self.first_visible_asteroid(&location, &d).is_some())
            .count()
    }

    fn monitoring_station(&self) -> Option<Point> {
        (0..self.width())
            .cartesian_product(0..self.height())
            .filter(|(x, y)| match self.grid[*y][*x] {
                MapTile::Full => true,
                MapTile::Empty => false,
            })
            .map(|(x, y)| Point {
                x: x as i64,
                y: y as i64,
            })
            .max_by_key(|p| self.visible_count(p))
    }

    fn destroy_n(&mut self, location: &Point, n: usize) -> Option<Point> {
        let mut count = 0;
        for direction in self.get_directions(location).iter().cycle() {
            if let Some(coords) = self.first_visible_asteroid(location, &direction) {
                self.grid[coords.y as usize][coords.x as usize] = MapTile::Empty;
                count += 1;
                if count == n {
                    return Some(coords);
                }
            }
        }

        None
    }
}

fn part1(asteroid_field: &AsteroidField) -> Result<usize> {
    Ok(asteroid_field.visible_count(
        &asteroid_field
            .monitoring_station()
            .ok_or(AOCError::new("No monitoring station found"))?,
    ))
}

fn part2(asteroid_field: &mut AsteroidField) -> Result<i64> {
    let monitoring_station = asteroid_field
        .monitoring_station()
        .ok_or(AOCError::new("No monitoring station found"))?;
    let coords = asteroid_field
        .destroy_n(&monitoring_station, 200)
        .ok_or(AOCError::new("nth asteroid not found"))?;

    Ok(100 * coords.x + coords.y)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut asteroid_field = input.trim().parse()?;

    println!("Part 1: {}", part1(&asteroid_field)?);
    println!("Part 2: {}", part2(&mut asteroid_field)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let field: AsteroidField = ".#..#\n\
                                    .....\n\
                                    #####\n\
                                    ....#\n\
                                    ...##"
            .trim()
            .parse()
            .unwrap();
        let point = Point { x: 3, y: 4 };
        assert_eq!(field.monitoring_station(), Some(point.clone()));
        assert_eq!(field.visible_count(&point), 8);
    }

    #[test]
    fn larger_test() {
        let field: AsteroidField = "......#.#.\n\
                                    #..#.#....\n\
                                    ..#######.\n\
                                    .#.#.###..\n\
                                    .#..#.....\n\
                                    ..#....#.#\n\
                                    #..#....#.\n\
                                    .##.#..###\n\
                                    ##...#..#.\n\
                                    .#....####"
            .trim()
            .parse()
            .unwrap();
        let point = Point { x: 5, y: 8 };
        assert_eq!(field.monitoring_station(), Some(point.clone()));
        assert_eq!(field.visible_count(&point), 33);
    }

    #[test]
    fn test_nth_asteroid() {
        let mut field: AsteroidField = ".#....#####...#..\n\
                                        ##...##.#####..##\n\
                                        ##...#...#.#####.\n\
                                        ..#.........###..\n\
                                        ..#.#.....#....##"
            .trim()
            .parse()
            .unwrap();
        assert_eq!(
            field.destroy_n(&Point { x: 8, y: 3 }, 8),
            Some(Point { x: 11, y: 2 })
        );
    }
}
