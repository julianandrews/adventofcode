extern crate aoc;

use aoc::graphs::{bfs, Graph};
use aoc::points::Point;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

struct Grid {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    points: Vec<Point>,
    grid: HashMap<Point, Option<usize>>,
}

impl ::std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
        for y in self.min_y..self.max_y + 1 {
            writeln!(
                f,
                "{}",
                (self.min_x..self.max_x + 1)
                    .map(|x| self
                        .grid
                        .get(&Point {
                            x: x as isize,
                            y: y as isize
                        }).unwrap_or(&Some(9))
                        .map(|n| if n == 9 {
                            "-".to_string()
                        } else {
                            n.to_string()
                        }).unwrap_or(".".to_string())).collect::<Vec<_>>()
                    .join(" ")
            )?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(points: Vec<Point>) -> Grid {
        let mut grid = HashMap::new();
        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();

        let mut working_points = vec![];
        for point in points.clone() {
            let mut ps = HashSet::new();
            ps.insert(point);
            working_points.push(ps);
        }
        while working_points.iter().any(|ps| ps.len() > 0) {
            let new_points: Vec<HashSet<Point>> = working_points
                .iter()
                .map(|ps| ps.iter().flat_map(|p| p.manhattan_neighbors()).collect())
                .collect();
            let mut counts: HashMap<&Point, u32> = HashMap::new();
            for point in new_points.iter().flatten() {
                *counts.entry(point).or_insert(0) += 1;
            }
            for (i, ps) in new_points.iter().enumerate() {
                working_points[i].clear();
                for &p in ps {
                    if min_x <= p.x
                        && p.x <= max_x
                        && min_y <= p.y
                        && p.y <= max_y
                        && !grid.contains_key(&p)
                    {
                        if counts[&p] == 1 {
                            working_points[i].insert(p.clone());
                            grid.insert(p, Some(i));
                        } else {
                            grid.insert(p, None);
                        }
                    }
                }
            }
        }

        Grid {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
            points: points,
            grid: grid,
        }
    }

    fn areas(&self) -> HashMap<Point, usize> {
        let mut areas = HashMap::new();
        for points in self.grid.values() {
            if let Some(i) = points {
                *areas
                    .entry(self.points.get(*i).unwrap().clone())
                    .or_insert(0) += 1;
            }
        }

        areas
    }

    fn edge(&self) -> HashSet<Point> {
        let mut edge = HashSet::new();
        for x in self.min_x..self.max_x + 1 {
            edge.insert(Point::new(x, self.min_y));
            edge.insert(Point::new(x, self.max_y));
        }

        for y in self.min_y..self.max_y + 1 {
            edge.insert(Point::new(self.min_x, y));
            edge.insert(Point::new(self.max_x, y));
        }

        edge
    }

    fn finite_points(&self) -> HashSet<Point> {
        let mut edge_values = HashSet::new();
        for p in self.edge() {
            if let Some(i) = self.grid.get(&p) {
                if let Some(i) = i {
                    edge_values.insert(i);
                }
            }
        }

        self.points
            .iter()
            .cloned()
            .enumerate()
            .filter(|(i, _)| !edge_values.contains(i))
            .map(|(_, point)| point)
            .collect()
    }
}

struct Part2Graph {
    points: Vec<Point>,
}

impl Graph<Point> for Part2Graph {
    fn neighbors(&self, point: &Point) -> Vec<Point> {
        point
            .manhattan_neighbors()
            .into_iter()
            .filter(|p| self.total_manhattan_distance(p) < 10000)
            .collect()
    }

    fn values(&self) -> Vec<Point> {
        self.points.clone()
    }
}

impl Part2Graph {
    fn total_manhattan_distance(&self, point: &Point) -> usize {
        self.points
            .iter()
            .map(|p| p.manhattan_distance(point))
            .sum()
    }
}

fn part1(points: Vec<Point>) -> Result<()> {
    let grid = Grid::new(points);
    let areas = grid.areas();

    let area = grid
        .finite_points()
        .iter()
        .flat_map(|point| areas.get(point))
        .max()
        .unwrap();

    writeln!(io::stdout(), "{}", area)?;

    Ok(())
}

fn part2(points: Vec<Point>) -> Result<()> {
    let center = Point {
        x: points.iter().map(|p| p.x).sum::<isize>() / points.len() as isize,
        y: points.iter().map(|p| p.y).sum::<isize>() / points.len() as isize,
    };

    let graph = Part2Graph { points: points };

    let mut count = 0;
    for _node in bfs(graph, center) {
        count += 1;
    }

    writeln!(io::stdout(), "{}", count)?;
    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let points: Vec<Point> = input.lines().flat_map(|line| line.parse()).collect();

    part1(points.clone())?;
    part2(points.clone())?;
    Ok(())
}
