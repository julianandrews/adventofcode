use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PointParseError(());

impl ::std::str::FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let v: Vec<isize> = s.split(", ").flat_map(|v| v.parse()).collect();
        if let [x, y] = v[..] {
            Ok(Point { x, y })
        } else {
            Err(PointParseError(()))
        }
    }
}

impl Point {
    fn ring(&self, d: usize) -> HashSet<Point> {
        let mut points = HashSet::with_capacity(4 * d);
        let d: isize = d as isize;
        for x in 0..d + 1 {
            let y = d - x;
            points.insert(Point {
                x: self.x + x,
                y: self.y + y,
            });
            points.insert(Point {
                x: self.x + x,
                y: self.y - y,
            });
            points.insert(Point {
                x: self.x - x,
                y: self.y + y,
            });
            points.insert(Point {
                x: self.x - x,
                y: self.y - y,
            });
        }

        points
    }
}

struct Grid {
    size: usize,
    points: Vec<Point>,
    grid: HashMap<Point, Option<usize>>,
}

impl ::std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
        for y in 0..self.size {
            writeln!(
                f,
                "{}",
                (0..self.size)
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
    fn new(points: Vec<Point>, size: usize) -> Grid {
        let mut grid = HashMap::new();

        for d in 0..size {
            let rings: Vec<HashSet<Point>> = points.iter().map(|point| point.ring(d)).collect();

            let mut counts = HashMap::new();
            for point in rings.iter().flatten() {
                *counts.entry(point.clone()).or_insert(0) += 1;
            }
            for (i, ring) in rings.iter().enumerate() {
                for point in ring.iter() {
                    if !grid.contains_key(point) {
                        let value;
                        let &count = counts.get(point).unwrap();
                        if count == 1 {
                            value = Some(i)
                        } else {
                            value = None
                        };
                        grid.insert(point.clone(), value);
                    }
                }
            }
        }

        Grid {
            size: size,
            points: points,
            grid: grid,
        }
    }

    fn areas(&self) -> HashMap<Point, usize> {
        let mut areas = HashMap::new();
        for foo in self.grid.values() {
            if let Some(i) = foo {
                *areas
                    .entry(self.points.get(*i).unwrap().clone())
                    .or_insert(0) += 1;
            }
        }

        areas
    }

    fn edge(&self) -> HashSet<Point> {
        let mut edge = HashSet::new();
        for v in 0..self.size as isize {
            edge.insert(Point { x: v, y: 0 });
            edge.insert(Point {
                x: v,
                y: self.size as isize - 1,
            });
            edge.insert(Point { x: 0, y: v });
            edge.insert(Point {
                x: self.size as isize - 1,
                y: v,
            });
        }

        edge
    }

    fn finite_points(&self) -> HashSet<Point> {
        let mut edge_points = HashSet::new();
        for point in self.edge().iter() {
            if let Some(foo) = self.grid.get(point) {
                if let Some(i) = foo {
                    edge_points.insert(self.points[*i].clone());
                }
            }
        }

        self.points
            .iter()
            .cloned()
            .filter(|point| !edge_points.contains(point))
            .collect()
    }
}

fn part1(input: &str) -> Result<()> {
    let points: Vec<Point> = input.lines().flat_map(|line| line.parse()).collect();
    let size = 400;
    let grid = Grid::new(points, size);
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

fn part2(_input: &str) -> Result<()> {
    // writeln!(io::stdout(), "{}", 0)?;
    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}
