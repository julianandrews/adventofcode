use anyhow::Result;

use aoc_2025::disjoint_set::DisjointSet;

fn main() -> Result<()> {
    let input = aoc_2025::utils::get_input()?;
    let mut solver = Solver::new(parsing::parse_input(input.trim())?);

    println!("{}", part1(&mut solver));
    println!("{}", part2(&mut solver)?);

    Ok(())
}

fn part1(solver: &mut Solver) -> usize {
    for _ in 0..1000 {
        solver.add_edge();
    }
    solver.component_sizes()[0..3].iter().product()
}

fn part2(solver: &mut Solver) -> Result<u64> {
    match solver.build_mst_returning_last_edge() {
        Some((a, b)) => Ok(a.x * b.x),
        None => anyhow::bail!("Graph already connected (maybe in part1?)"),
    }
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
    edges: Vec<Edge>,
    disjoint_set: DisjointSet,
    component_count: usize,
}

impl Solver {
    pub fn new(points: Vec<Point>) -> Self {
        let n = points.len();
        let mut edges: Vec<Edge> = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n - 1 {
            for j in i + 1..n {
                let sq_dist = squared_distance(&points[i], &points[j]);
                edges.push(Edge::new(sq_dist, i, j));
            }
        }
        // Descending order for easy popping
        edges.sort_unstable_by_key(|&edge| std::cmp::Reverse(edge));
        let disjoint_set = DisjointSet::new(n);
        Solver {
            points,
            edges,
            disjoint_set,
            component_count: n,
        }
    }

    pub fn add_edge(&mut self) -> Option<(Point, Point)> {
        if let Some(edge) = self.edges.pop() {
            let (i, j) = (edge.i(), edge.j());
            if self.disjoint_set.union(i, j) {
                self.component_count -= 1;
                return Some((self.points[i], self.points[j]));
            }
        }
        None
    }

    pub fn build_mst_returning_last_edge(&mut self) -> Option<(Point, Point)> {
        let mut edge = None;
        while self.component_count > 1 {
            edge = self.add_edge();
        }
        edge
    }

    pub fn component_sizes(&self) -> Vec<usize> {
        let mut sizes = vec![0; self.points.len()];
        for i in 0..self.points.len() {
            sizes[self.disjoint_set.find(i)] += 1;
        }
        sizes.sort_unstable_by_key(|&s| std::cmp::Reverse(s));
        sizes
    }
}

/// Edge packed as: [36-bit weight | 14-bit i | 14-bit j] for faster sorting
/// Weight supports up to 2^36 ≈ 68B (max distance^2 = 3*(100k)^2 = 30B)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Edge(u64);

impl Edge {
    const INDEX_BITS: u32 = 14;
    const INDEX_MASK: u64 = (1 << Self::INDEX_BITS) - 1;
    const WEIGHT_BITS: u32 = 64 - 2 * Self::INDEX_BITS;
    const WEIGHT_SHIFT: u32 = 64 - Self::WEIGHT_BITS;

    fn new(weight: u64, i: usize, j: usize) -> Self {
        assert!(i < 1 << Self::INDEX_BITS, "Index too large");
        assert!(j < 1 << Self::INDEX_BITS, "Index too large");
        assert!(weight < 1 << Self::WEIGHT_BITS, "Weight too large");
        Edge((weight << Self::WEIGHT_SHIFT) | ((i as u64) << Self::INDEX_BITS) | (j as u64))
    }

    fn i(&self) -> usize {
        ((self.0 >> Self::INDEX_BITS) & Self::INDEX_MASK) as usize
    }

    fn j(&self) -> usize {
        (self.0 & Self::INDEX_MASK) as usize
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

    const MAX_COORD: u64 = 100_000;

    pub fn parse_input(s: &str) -> Result<Vec<Point>> {
        s.lines().map(|line| line.parse()).collect()
    }

    impl std::str::FromStr for Point {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut coords = s.splitn(3, ',').map(|part| part.trim().parse());
            let x = coords.next().ok_or_else(|| anyhow!("Missing x"))??;
            let y = coords.next().ok_or_else(|| anyhow!("Missing y"))??;
            let z = coords.next().ok_or_else(|| anyhow!("Missing z"))??;
            if x > MAX_COORD || y > MAX_COORD || z > MAX_COORD {
                anyhow::bail!("Coordinates out of range: {}", s);
            }
            Ok(Point { x, y, z })
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
        assert_eq!(&sizes[0..11], &[5, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1]);
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
