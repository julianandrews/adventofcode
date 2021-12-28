use std::collections::HashSet;

use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let (points_part, folds_part) = input
        .trim()
        .split_once("\n\n")
        .ok_or(AOCError::new("Failed to split input"))?;
    let sheet: Sheet = points_part.parse()?;
    let folds = parse_fields(folds_part, '\n')?;

    println!("Part 1: {}", part1(sheet.clone(), &folds));
    println!("Part 2: \n{}", part2(sheet.clone(), &folds));
    Ok(())
}

fn part1(mut sheet: Sheet, folds: &[Fold]) -> usize {
    sheet.fold(folds[0]);
    sheet.point_count()
}

fn part2(mut sheet: Sheet, folds: &[Fold]) -> Sheet {
    for &fold in folds {
        sheet.fold(fold);
    }
    sheet
}

#[derive(Debug, Clone)]
struct Sheet {
    points: HashSet<(i64, i64)>,
}

impl Sheet {
    fn fold(&mut self, fold: Fold) {
        self.points = self
            .points
            .iter()
            .map(|(x, y)| match fold {
                Fold::AlongX(line) => (line - (x - line).abs(), *y),
                Fold::AlongY(line) => (*x, line - (y - line).abs()),
            })
            .collect()
    }

    fn point_count(&self) -> usize {
        self.points.len()
    }
}

impl std::str::FromStr for Sheet {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut points = HashSet::new();
        for line in s.lines() {
            let (a, b) = line
                .split_once(',')
                .ok_or(AOCError::new("Failed to parse line"))?;
            points.insert((a.parse()?, b.parse()?));
        }
        Ok(Sheet { points })
    }
}

impl std::fmt::Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.points.iter().map(|(x, _)| x).min().unwrap_or(&0);
        let max_x = self.points.iter().map(|(x, _)| x).max().unwrap_or(&0);
        let min_y = self.points.iter().map(|(_, y)| y).min().unwrap_or(&0);
        let max_y = self.points.iter().map(|(_, y)| y).max().unwrap_or(&0);
        let mut lines: Vec<String> = vec![];
        for y in *min_y..=*max_y {
            lines.push(
                (*min_x..=*max_x)
                    .map(|x| {
                        if self.points.contains(&(x, y)) {
                            '█'
                        } else {
                            ' '
                        }
                    })
                    .collect(),
            )
        }
        write!(f, "{}", lines.join("\n"))
    }
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    AlongX(i64),
    AlongY(i64),
}

impl std::str::FromStr for Fold {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (initial, rest) = s.split_at(11);
        if initial != "fold along " {
            return Err(Box::new(AOCError::new("Ill formed fold")));
        }
        let (axis, value) = rest
            .split_once('=')
            .ok_or(AOCError::new("Failed to parse fold"))?;
        let value: i64 = value.parse()?;
        match axis {
            "x" => Ok(Fold::AlongX(value)),
            "y" => Ok(Fold::AlongY(value)),
            _ => Err(Box::new(AOCError::new("Invalid axis"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = "6,10\
                            \n0,14\
                            \n9,10\
                            \n0,3\
                            \n10,4\
                            \n4,11\
                            \n6,0\
                            \n6,12\
                            \n4,1\
                            \n0,13\
                            \n10,12\
                            \n3,4\
                            \n3,0\
                            \n8,4\
                            \n1,10\
                            \n2,14\
                            \n8,10\
                            \n9,0";

    #[test]
    fn basic_folds() {
        let mut sheet: Sheet = TEST_DATA.parse().unwrap();
        sheet.fold(Fold::AlongY(7));
        assert_eq!(sheet.point_count(), 17);
        sheet.fold(Fold::AlongX(5));
        assert_eq!(sheet.point_count(), 16);
    }
}
