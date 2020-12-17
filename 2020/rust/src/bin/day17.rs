#![feature(min_const_generics)]

use itertools::Itertools;
use std::collections::HashSet;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let dimension_3d = parse_input::<3>(&input)?;
    let dimension_4d = parse_input::<4>(&input)?;

    println!("Part 1: {}", part1(dimension_3d));
    println!("Part 2: {}", part2(dimension_4d));
    Ok(())
}

fn part1(mut dimension: PocketDimension<3>) -> usize {
    dimension.run_boot_cycle();
    dimension.active_count()
}

fn part2(mut dimension: PocketDimension<4>) -> usize {
    dimension.run_boot_cycle();
    dimension.active_count()
}

#[derive(Debug, Clone)]
struct PocketDimension<const N: usize> {
    active_cubes: HashSet<Point<N>>,
}

impl<const N: usize> PocketDimension<N> {
    fn run_boot_cycle(&mut self) {
        for _ in 0..6 {
            self.step();
        }
    }

    fn step(&mut self) {
        let candidate_points: HashSet<_> = self
            .active_cubes
            .iter()
            .flat_map(Point::neighborhood)
            .collect();
        let to_activate: Vec<_> = candidate_points
            .into_iter()
            .filter(|p| !self.active_cubes.contains(p) && self.neighborhood_count(p) == 3)
            .collect();
        let to_deactivate: Vec<_> = self
            .active_cubes
            .iter()
            .filter(|p| !(3..=4).contains(&self.neighborhood_count(p)))
            .cloned()
            .collect();

        for point in to_activate {
            self.active_cubes.insert(point);
        }
        for point in to_deactivate {
            self.active_cubes.remove(&point);
        }
    }

    fn neighborhood_count(&self, point: &Point<N>) -> usize {
        point
            .neighborhood()
            .filter(|p| self.active_cubes.contains(p))
            .count()
    }

    fn active_count(&self) -> usize {
        self.active_cubes.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point<const N: usize> {
    coords: [i64; N],
}

impl<const N: usize> Point<N> {
    fn neighborhood(&self) -> Box<dyn Iterator<Item = Self>> {
        let p = self.clone();
        Box::new(
            (0..self.coords.len())
                .map(|_| (-1..=1))
                .multi_cartesian_product()
                .map(move |offset| {
                    let mut coords = [0; N];
                    for i in 0..N {
                        coords[i] = p.coords[i] + offset[i];
                    }
                    Point { coords }
                }),
        )
    }
}

fn parse_input<const N: usize>(input: &str) -> Result<PocketDimension<N>> {
    let active_cubes = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => {
                    let mut coords = [0; N];
                    coords[0] = x as i64;
                    coords[1] = y as i64;
                    Some(Ok(Point { coords }))
                }
                '.' => None,
                _ => Some(Err(AOCError::new("Invalid input").into())),
            })
        })
        .collect::<Result<_>>()?;
    Ok(PocketDimension { active_cubes })
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = ".#.\
                                     \n..#\
                                     \n###";

    #[test]
    fn three_dimensions() {
        let mut dimension = parse_input::<3>(TEST_INPUT).unwrap();
        dimension.run_boot_cycle();
        assert_eq!(dimension.active_count(), 112);
    }

    #[test]
    #[ignore]
    fn four_dimensions() {
        let mut dimension = parse_input::<4>(TEST_INPUT).unwrap();
        dimension.run_boot_cycle();
        assert_eq!(dimension.active_count(), 848);
    }
}
