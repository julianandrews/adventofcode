use std::collections::HashMap;

use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let lines: Vec<Line> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
    Ok(())
}

fn part1(lines: &[Line]) -> usize {
    let mut point_counts = HashMap::new();
    for line in lines {
        if line.is_horizontal() || line.is_vertical() {
            for point in line.points() {
                *point_counts.entry(point).or_insert(0) += 1;
            }
        }
    }
    point_counts.values().filter(|&count| count >= &2).count()
}

fn part2(lines: &[Line]) -> usize {
    let mut point_counts = HashMap::new();
    for line in lines {
        for point in line.points() {
            *point_counts.entry(point).or_insert(0) += 1;
        }
    }
    point_counts.values().filter(|&count| count >= &2).count()
}

#[derive(Debug)]
struct Line {
    start: (i64, i64),
    end: (i64, i64),
}

impl Line {
    fn points(&self) -> impl Iterator<Item = (i64, i64)> {
        let (dx, dy) = (
            Line::direction(self.start.0, self.end.0),
            Line::direction(self.start.1, self.end.1),
        );
        let mut points = vec![self.start];
        let mut p = self.start;
        while p != self.end {
            p = (p.0 + dx, p.1 + dy);
            points.push(p);
        }
        points.into_iter()
    }

    fn is_vertical(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_horizontal(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn direction(a: i64, b: i64) -> i64 {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        }
    }
}

impl std::str::FromStr for Line {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s
            .split_once(" -> ")
            .ok_or(AOCError::new("Failed to split line"))?;
        Ok(Line {
            start: parse_point(start)?,
            end: parse_point(end)?,
        })
    }
}

fn parse_point(s: &str) -> Result<(i64, i64)> {
    let (x, y) = s
        .split_once(',')
        .ok_or(AOCError::new("Failed to split point"))?;
    Ok((x.parse()?, y.parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: [&str; 10] = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn test_part1() {
        let lines: Vec<Line> = TEST_DATA.iter().map(|s| s.parse().unwrap()).collect();
        let result = part1(&lines);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let lines: Vec<Line> = TEST_DATA.iter().map(|s| s.parse().unwrap()).collect();
        let result = part2(&lines);
        assert_eq!(result, 12);
    }
}
