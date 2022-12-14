use anyhow::{anyhow, bail, Result};

use aoc::planar::Point;
use aoc::utils::{get_input, parse_fields};

static CENTER: i64 = 500;

fn main() -> Result<()> {
    let input = get_input()?;
    let diagram: SandDiagram = input.trim().parse()?;

    println!("Part 1: {}", part1(diagram.clone()));
    println!("Part 2: {}", part2(&diagram));

    Ok(())
}

fn part1(mut diagram: SandDiagram) -> usize {
    std::iter::from_fn(|| diagram.add_sand().then_some(true)).count()
}

fn part2(diagram: &SandDiagram) -> usize {
    diagram.count_sand_with_floor()
}

#[derive(Debug, Clone)]
struct SandDiagram {
    filled: Vec<Vec<bool>>,
    bottom: i64,
}

impl SandDiagram {
    fn add_sand(&mut self) -> bool {
        let mut p = Point::new(CENTER, 0);
        loop {
            let next = self.neighbors(p).next();
            match next {
                Some(next) if next.y > self.bottom => return false,
                Some(next) => p = next,
                None => {
                    self.filled[p.y as usize][p.x as usize] = true;
                    return true;
                }
            }
        }
    }

    fn count_sand_with_floor(&self) -> usize {
        // Use DFS to find all reachable points.
        let mut count = 0;
        let mut visited = vec![vec![false; self.filled[0].len()]; self.filled.len()];
        let mut to_visit = vec![Point::new(CENTER, 0)];
        while let Some(point) = to_visit.pop() {
            if visited[point.y as usize][point.x as usize] {
                continue;
            }
            count += 1;
            visited[point.y as usize][point.x as usize] = true;
            if point.y <= self.bottom {
                for neighbor in self.neighbors(point) {
                    to_visit.push(neighbor);
                }
            }
        }
        count
    }

    fn neighbors(&'_ self, point: Point) -> impl Iterator<Item = Point> + '_ {
        static DIRS: [Point; 3] = [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)];
        DIRS.iter()
            .map(move |direction| point + *direction)
            .filter(|p| !self.filled[p.y as usize][p.x as usize])
    }
}

impl std::str::FromStr for SandDiagram {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<RockLine> = parse_fields(s, '\n')?;
        let bottom = {
            let points = lines.iter().flat_map(RockLine::iter);
            points.map(|p| p.y).max().unwrap_or(0)
        };

        // Ensure our map is big enough to extend past the furthest point we might check.
        // This wastes a little memory, but makes the arithmetic easy.
        // We must be able to index one past the ends, and indexes start at 0!
        if bottom + 1 > CENTER {
            bail!("Diagram too deep - sand could overflow");
        }
        let mut filled = vec![vec![false; (CENTER + bottom + 2) as usize]; bottom as usize + 2];

        for line in lines {
            for pair in line.0.windows(2) {
                let (a, b) = (pair[0], pair[1]);
                let direction = {
                    let d = b - a;
                    let m = (d.x + d.y).abs();
                    d / m
                };
                let mut p = a;
                while p != b {
                    filled[p.y as usize][p.x as usize] = true;
                    p += direction;
                }
                filled[b.y as usize][b.x as usize] = true;
            }
        }
        Ok(Self { filled, bottom })
    }
}

#[derive(Debug, Clone)]
struct RockLine(Vec<Point>);

impl RockLine {
    fn iter(&self) -> impl Iterator<Item = &Point> {
        self.0.iter()
    }
}

impl std::str::FromStr for RockLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|p| {
                let (x, y) = p
                    .split_once(',')
                    .ok_or_else(|| anyhow!("Failed to parse {}", s))?;
                Ok(Point::new(x.parse()?, y.parse()?))
            })
            .collect::<Result<_>>()?;
        Ok(Self(points))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn without_floor() {
        let diagram: SandDiagram = TEST_DATA.parse().unwrap();
        assert_eq!(part1(diagram), 24);
    }

    #[test]
    fn with_floor() {
        let diagram: SandDiagram = TEST_DATA.parse().unwrap();
        assert_eq!(diagram.count_sand_with_floor(), 93);
    }
}
