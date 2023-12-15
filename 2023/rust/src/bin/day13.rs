// 40269 too high
use anyhow::{bail, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let patterns: Vec<Pattern> = parse_fields(input.trim(), "\n\n")?;

    println!("Part 1: {}", summarize(&patterns, 0)?);
    println!("Part 2: {}", summarize(&patterns, 1)?);

    Ok(())
}

fn summarize(patterns: &[Pattern], smudges: u32) -> Result<usize> {
    let mut summary = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        let mirrors: Vec<Mirror> = pattern.mirrors(smudges).collect();
        if mirrors.len() != 1 {
            bail!("Expected one mirror, got {:?} for pattern {}", mirrors, i);
        }
        summary += match mirrors[0] {
            Mirror::Vertical(x) => x,
            Mirror::Horizontal(y) => 100 * y,
        }
    }
    Ok(summary)
}

#[derive(Debug, Clone)]
struct Pattern {
    rows: Vec<u64>,
    columns: Vec<u64>,
}

impl Pattern {
    fn mirrors(&self, smudges: u32) -> impl Iterator<Item = Mirror> + '_ {
        fn smudge_counts(items: &[u64]) -> impl Iterator<Item = (usize, u32)> + '_ {
            (1..items.len()).map(|x| {
                let count = (0..x)
                    .map(|i| {
                        (items[i] ^ *items.get(2 * x - i - 1).unwrap_or(&items[i])).count_ones()
                    })
                    .sum();
                (x, count)
            })
        }

        let vertical_mirrors = smudge_counts(&self.columns)
            .filter(move |&(_, count)| count == smudges)
            .map(|(x, _)| Mirror::Vertical(x));
        let horizontal_mirrors = smudge_counts(&self.rows)
            .filter(move |&(_, count)| count == smudges)
            .map(|(y, _)| Mirror::Horizontal(y));

        vertical_mirrors.chain(horizontal_mirrors)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mirror {
    Vertical(usize),
    Horizontal(usize),
}

impl std::str::FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(|line| line.len()).unwrap_or(0);
        if width > 64 {
            bail!("Pattern too wide:\n{}", s);
        }
        let rows: Vec<u64> = s
            .lines()
            .map(|line| {
                if line.len() != width {
                    bail!("Non-rectangular pattern");
                }
                Ok(line
                    .bytes()
                    .fold(0, |acc, b| (acc << 1) | (b == b'#') as u64))
            })
            .collect::<Result<_>>()?;
        if rows.len() > 64 {
            bail!("Pattern too long:\n{}", s);
        }
        let columns = (0..width)
            .rev()
            .map(|x| {
                rows.iter()
                    .fold(0, |acc, row| (acc << 1) | ((row >> x) & 1))
            })
            .collect();
        Ok(Pattern { rows, columns })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASE_1: &str = "\
        #.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.";

    static TEST_CASE_2: &str = "\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#";

    #[test]
    fn parse() {
        let pattern: Pattern = TEST_CASE_1.parse().unwrap();
        let rows = vec![
            0b101100110,
            0b001011010,
            0b110000001,
            0b110000001,
            0b001011010,
            0b001100110,
            0b101011010,
        ];
        #[rustfmt::skip]
        let columns = vec![
            0b1011001,
            0b0011000,
            0b1100111,
            0b1000010,
            0b0100101,
            0b0100101,
            0b1000010,
            0b1100111,
            0b0011000,
        ];

        assert_eq!(pattern.rows, rows);
        assert_eq!(pattern.columns, columns);
    }

    #[test]
    fn mirrors_1() {
        let pattern: Pattern = TEST_CASE_1.parse().unwrap();
        let result: Vec<_> = pattern.mirrors(0).collect();
        assert_eq!(result, vec![Mirror::Vertical(5)]);
    }

    #[test]
    fn mirrors_2() {
        let pattern: Pattern = TEST_CASE_2.parse().unwrap();
        let result: Vec<_> = pattern.mirrors(0).collect();
        assert_eq!(result, vec![Mirror::Horizontal(4)]);
    }

    #[test]
    fn smudged_mirrors_1() {
        let pattern: Pattern = TEST_CASE_1.parse().unwrap();
        let result: Vec<_> = pattern.mirrors(1).collect();
        assert_eq!(result, vec![Mirror::Horizontal(3)]);
    }

    #[test]
    fn smudged_mirrors_2() {
        let pattern: Pattern = TEST_CASE_2.parse().unwrap();
        let result: Vec<_> = pattern.mirrors(1).collect();
        assert_eq!(result, vec![Mirror::Horizontal(1)]);
    }

    #[test]
    fn summarize_1() {
        let patterns: Vec<Pattern> =
            vec![TEST_CASE_1.parse().unwrap(), TEST_CASE_2.parse().unwrap()];
        assert_eq!(summarize(&patterns, 0).unwrap(), 405);
    }

    #[test]
    fn summarize_2() {
        let patterns: Vec<Pattern> =
            vec![TEST_CASE_1.parse().unwrap(), TEST_CASE_2.parse().unwrap()];
        assert_eq!(summarize(&patterns, 1).unwrap(), 400);
    }
}
