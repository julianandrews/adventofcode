use anyhow::Result;

use aoc_2025::disjoint_set::DisjointSet;

fn main() -> Result<()> {
    let input = aoc_2025::utils::get_input()?;
    let mut solver = Solver::new(parsing::parse_input(input.trim())?);

    println!("{}", part1(&mut solver));
    println!("{}", part2(&mut solver));

    Ok(())
}

fn part1(solver: &mut Solver) -> usize {
    for _ in 0..1000 {
        solver.add_edge();
    }
    solver.component_sizes()[0..3].iter().product()
}

fn part2(solver: &mut Solver) -> u64 {
    let (a, b) = match solver.build_mst_returning_last_edge() {
        Some(pair) => pair,
        None => return 0,
    };
    a.x * b.x
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug, Clone)]
struct Solver {
    points: Vec<Point>,
    edges: Vec<(u64, usize, usize)>,
    disjoint_set: DisjointSet,
    component_count: usize,
}

impl Solver {
    pub fn new(points: Vec<Point>) -> Self {
        let n = points.len();
        let mut edges: Vec<(u64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n - 1 {
            for j in i + 1..n {
                edges.push((squared_distance(&points[i], &points[j]), i, j));
            }
        }
        // Descending order for easy popping
        edges.sort_unstable_by_key(|&(d, _, _)| std::cmp::Reverse(d));
        let disjoint_set = DisjointSet::new(n);
        Solver {
            points,
            edges,
            disjoint_set,
            component_count: n,
        }
    }

    pub fn add_edge(&mut self) -> Option<(Point, Point)> {
        match self.edges.pop() {
            Some((_, i, j)) => {
                if self.disjoint_set.union(i, j) {
                    self.component_count -= 1;
                    Some((self.points[i], self.points[j]))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn build_mst_returning_last_edge(&mut self) -> Option<(Point, Point)> {
        let mut edge = None;
        while self.component_count > 1 {
            edge = self.add_edge();
        }
        edge
    }

    pub fn component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = vec![0; self.points.len()];
        for i in 0..self.points.len() {
            sizes[self.disjoint_set.find(i)] += 1;
        }
        sizes.sort_unstable_by_key(|&s| std::cmp::Reverse(s));
        sizes
    }
}

fn squared_distance(a: &Point, b: &Point) -> u64 {
    let dx = a.x.abs_diff(b.x);
    let dy = a.y.abs_diff(b.y);
    let dz = a.z.abs_diff(b.z);
    dx * dx + dy * dy + dz * dz
}

mod parsing {
    use anyhow::{anyhow, Result};

    use crate::Point;

    pub fn parse_input(s: &str) -> Result<Vec<Point>> {
        s.lines().map(|line| line.parse()).collect()
    }

    impl std::str::FromStr for Point {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut coords = s.splitn(3, ',').map(|part| part.trim().parse());
            Ok(Point {
                x: coords
                    .next()
                    .ok_or_else(|| anyhow!("Missing x coordinate"))??,
                y: coords
                    .next()
                    .ok_or_else(|| anyhow!("Missing x coordinate"))??,
                z: coords
                    .next()
                    .ok_or_else(|| anyhow!("Missing x coordinate"))??,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parsing, Point, Solver};

    static TEST_DATA: &str = concat!(
        "162,817,812\n",
        "57,618,57\n",
        "906,360,560\n",
        "592,479,940\n",
        "352,342,300\n",
        "466,668,158\n",
        "542,29,236\n",
        "431,825,988\n",
        "739,650,466\n",
        "52,470,668\n",
        "216,146,977\n",
        "819,987,18\n",
        "117,168,530\n",
        "805,96,715\n",
        "346,949,466\n",
        "970,615,88\n",
        "941,993,340\n",
        "862,61,35\n",
        "984,92,344\n",
        "425,690,689",
    );

    fn make_point(x: u64, y: u64, z: u64) -> Point {
        Point { x, y, z }
    }

    #[test]
    fn parsing() {
        let points = parsing::parse_input(TEST_DATA).unwrap();
        let expected = vec![
            make_point(162, 817, 812),
            make_point(57, 618, 57),
            make_point(906, 360, 560),
            make_point(592, 479, 940),
            make_point(352, 342, 300),
            make_point(466, 668, 158),
            make_point(542, 29, 236),
            make_point(431, 825, 988),
            make_point(739, 650, 466),
            make_point(52, 470, 668),
            make_point(216, 146, 977),
            make_point(819, 987, 18),
            make_point(117, 168, 530),
            make_point(805, 96, 715),
            make_point(346, 949, 466),
            make_point(970, 615, 88),
            make_point(941, 993, 340),
            make_point(862, 61, 35),
            make_point(984, 92, 344),
            make_point(425, 690, 689),
        ];

        assert_eq!(points, expected);
    }

    #[test]
    fn add_edge() {
        let mut solver = Solver::new(parsing::parse_input(TEST_DATA).unwrap());
        for _ in 0..10 {
            solver.add_edge();
        }
        assert_eq!(solver.component_count, 11);
        let sizes = solver.component_sizes();
        assert_eq!(
            sizes,
            vec![5, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn build_mst() {
        let mut solver = Solver::new(parsing::parse_input(TEST_DATA).unwrap());
        let edge = solver.build_mst_returning_last_edge();
        assert_eq!(
            edge,
            Some((
                Point {
                    x: 216,
                    y: 146,
                    z: 977
                },
                Point {
                    x: 117,
                    y: 168,
                    z: 530
                }
            ))
        );
    }
}
