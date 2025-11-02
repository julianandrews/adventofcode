use std::collections::VecDeque;

use anyhow::{anyhow, Result};

use aoc::planar::{CardinalDirection, Direction, Point};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let points = parsing::parse_input(&input)?;

    println!("Part 1: {}", part1(&points)?);
    println!("Part 2: {}", part2(&points)?);

    Ok(())
}

fn part1(points: &[Point]) -> Result<u64> {
    find_path::<71>(&points[0..1024]).ok_or_else(|| anyhow!("Failed to find path"))
}

fn part2(points: &[Point]) -> Result<String> {
    let point = find_last_path::<71>(points).ok_or_else(|| anyhow!("Failed to find last path."))?;
    Ok(format!("{},{}", point.x, point.y))
}

fn find_path<const SIZE: usize>(points: &[Point]) -> Option<u64> {
    // TODO: Check no points out of grid
    let mut grid = vec![vec![false; SIZE]; SIZE];
    let mut visited = vec![vec![false; SIZE]; SIZE];
    for p in points {
        grid[p.y as usize][p.x as usize] = true;
    }
    let mut queue = VecDeque::new();
    queue.push_back((Point { x: 0, y: 0 }, 0));
    while let Some((p, steps)) = queue.pop_front() {
        if p.x as usize == SIZE - 1 && p.y as usize == SIZE - 1 {
            return Some(steps);
        }
        if visited[p.y as usize][p.x as usize] {
            continue;
        }
        visited[p.y as usize][p.x as usize] = true;
        for d in CardinalDirection::iter() {
            let (dx, dy) = d.step();
            let new_p = Point {
                x: p.x + dx as i64,
                y: p.y + dy as i64,
            };
            if new_p.x < 0 || new_p.y < 0 || new_p.x >= SIZE as i64 || new_p.y >= SIZE as i64 {
                continue;
            }
            if !grid[new_p.y as usize][new_p.x as usize] {
                queue.push_back((new_p, steps + 1));
            }
        }
    }
    None
}

fn find_last_path<const SIZE: usize>(points: &[Point]) -> Option<Point> {
    let mut bottom = 0;
    let mut top = points.len();
    while top != bottom {
        let mid = bottom + (top - bottom + 1) / 2;
        if let Some(_) = find_path::<SIZE>(&points[0..mid]) {
            bottom = mid;
            if bottom == top - 1 {
                return Some(points[mid]);
            }
        } else {
            top = mid;
        }
    }
    None
}

mod parsing {
    use anyhow::{anyhow, Result};

    use aoc::planar::Point;

    pub fn parse_input(s: &str) -> Result<Vec<Point>> {
        s.lines().map(parse_point).collect()
    }

    fn parse_point(s: &str) -> Result<Point> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| anyhow!("Failed to parse {}", s))?;
        let (x, y) = (x.parse()?, y.parse()?);
        Ok(Point { x, y })
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_last_path, find_path};

    use super::Point;

    static EXAMPLE: &str = "\
        5,4\n\
        4,2\n\
        4,5\n\
        3,0\n\
        2,1\n\
        6,3\n\
        2,4\n\
        1,5\n\
        0,6\n\
        3,3\n\
        2,6\n\
        5,1\n\
        1,2\n\
        5,5\n\
        2,5\n\
        6,5\n\
        1,4\n\
        0,4\n\
        6,4\n\
        1,1\n\
        6,1\n\
        1,0\n\
        0,5\n\
        1,6\n\
        2,0";

    #[test]
    fn parsing() {
        let points = super::parsing::parse_input(EXAMPLE).unwrap();
        let expected = vec![
            Point { x: 5, y: 4 },
            Point { x: 4, y: 2 },
            Point { x: 4, y: 5 },
            Point { x: 3, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 6, y: 3 },
            Point { x: 2, y: 4 },
            Point { x: 1, y: 5 },
            Point { x: 0, y: 6 },
            Point { x: 3, y: 3 },
            Point { x: 2, y: 6 },
            Point { x: 5, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 5, y: 5 },
            Point { x: 2, y: 5 },
            Point { x: 6, y: 5 },
            Point { x: 1, y: 4 },
            Point { x: 0, y: 4 },
            Point { x: 6, y: 4 },
            Point { x: 1, y: 1 },
            Point { x: 6, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 5 },
            Point { x: 1, y: 6 },
            Point { x: 2, y: 0 },
        ];
        assert_eq!(points, expected);
    }

    #[test]
    fn simple_path() {
        let points = super::parsing::parse_input(EXAMPLE).unwrap();
        let steps = find_path::<7>(&points[0..12]);

        assert_eq!(steps, Some(22));
    }

    #[test]
    fn last_path() {
        let points = super::parsing::parse_input(EXAMPLE).unwrap();
        let last_point = find_last_path::<7>(&points);
        let expected = Some(Point { x: 6, y: 1 });

        assert_eq!(last_point, expected);
    }
}
