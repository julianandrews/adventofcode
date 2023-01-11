use std::ops::RangeInclusive;

use anyhow::{anyhow, Result};
use rustc_hash::FxHashSet;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let points: FxHashSet<Point3d> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&points));
    println!("Part 2: {}", part2(&points));

    Ok(())
}

fn part1(points: &FxHashSet<Point3d>) -> usize {
    empty_neighbors(points).count()
}

fn part2(points: &FxHashSet<Point3d>) -> usize {
    exterior_neighbors(points).count()
}

/// Returns an iterator over all empty neighbors of each point.
/// Values will repeat if they are neighbors of multiple input points.
fn empty_neighbors(points: &FxHashSet<Point3d>) -> impl Iterator<Item = Point3d> + '_ {
    points
        .iter()
        .flat_map(|p| p.neighbors().filter(|q| !points.contains(q)))
}

/// Returns an iterator over all exterior neighbors of each point.
/// Values will repeat if they are neighbors of multiple input points.
fn exterior_neighbors(points: &FxHashSet<Point3d>) -> impl Iterator<Item = Point3d> + '_ {
    let bounds = Bounds::from_points(points);
    let mut to_visit: Vec<Point3d> = vec![bounds.corner()];
    let mut exterior: FxHashSet<Point3d> = FxHashSet::default();
    while let Some(point) = to_visit.pop() {
        if bounds.contains(&point) && !exterior.contains(&point) && !points.contains(&point) {
            exterior.insert(point);
            to_visit.extend(point.neighbors());
        }
    }
    empty_neighbors(points).filter(move |p| exterior.contains(p))
}

#[derive(Debug, Clone)]
struct Bounds([RangeInclusive<i8>; 3]);

impl Bounds {
    /// Construct bounds extending one space beyond any point in the set.
    fn from_points(points: &FxHashSet<Point3d>) -> Self {
        let mut bounds = vec![];
        for i in 0..3 {
            let min = points.iter().map(|p| p.0[i]).min().unwrap_or(0);
            let max = points.iter().map(|p| p.0[i]).max().unwrap_or(0);
            bounds.push(min - 1..=max + 1);
        }
        Self(bounds.try_into().unwrap())
    }

    fn contains(&self, point: &Point3d) -> bool {
        (0..3).all(|i| self.0[i].contains(&point.0[i]))
    }

    fn corner(&self) -> Point3d {
        Point3d([*self.0[0].start(), *self.0[1].start(), *self.0[2].start()])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3d([i8; 3]);

impl Point3d {
    fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        (0..3).flat_map(move |i| {
            [1, -1].iter().map(move |d| {
                let mut p = *self;
                p.0[i] += d;
                p
            })
        })
    }
}

impl std::str::FromStr for Point3d {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: [_; 3] = parse_fields::<_, Vec<_>>(s, ',')?
            .try_into()
            .map_err(|_| anyhow!("Invalid point {}", s))?;
        Ok(Self(coordinates))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &[&str] = &[
        "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6", "1,2,5",
        "3,2,5", "2,1,5", "2,3,5",
    ];

    #[test]
    fn small_example() {
        let points: FxHashSet<Point3d> = ["1,1,1", "2,1,1"]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(empty_neighbors(&points).count(), 10);
    }

    #[test]
    fn larger_example() {
        let points: FxHashSet<Point3d> = TEST_DATA.iter().map(|s| s.parse().unwrap()).collect();
        assert_eq!(empty_neighbors(&points).count(), 64);
    }

    #[test]
    fn exterior_surface() {
        let points: FxHashSet<Point3d> = TEST_DATA.iter().map(|s| s.parse().unwrap()).collect();
        assert_eq!(exterior_neighbors(&points).count(), 58);
    }
}
