#![feature(iterator_try_collect)]

use anyhow::Result;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let reports: Vec<Report> = aoc::utils::parse_fields(input.trim(), '\n')?;

    println!("{}", part1(&reports));
    println!("{}", part2(&reports));

    Ok(())
}

fn part1(reports: &[Report]) -> usize {
    reports.iter().filter(|report| report.is_safe()).count()
}

fn part2(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|report| report.is_damp_safe())
        .count()
}

struct Report {
    levels: Vec<i64>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let signum = self.signum();
        self.diffs().all(|diff| Self::diff_ok(diff, signum))
    }

    fn is_damp_safe(&self) -> bool {
        for i in 0..self.levels.len() {
            let mut levels = self.levels.clone();
            levels.remove(i);
            if (Report { levels }).is_safe() {
                return true;
            }
        }
        false
    }

    fn diffs(&self) -> impl Iterator<Item = i64> + '_ {
        self.levels.windows(2).map(|w| w[1] - w[0])
    }

    fn diff_ok(diff: i64, signum: i64) -> bool {
        diff.abs() >= 1 && diff.abs() <= 3 && diff.signum() == signum
    }

    fn signum(&self) -> i64 {
        let signums = self.diffs().map(i64::signum);
        signums.take(3).sum::<i64>().signum()
    }
}

impl std::str::FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let levels: Vec<i64> = s.split(' ').map(|n| n.parse()).try_collect()?;
        Ok(Report { levels })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9";

    #[test]
    fn is_safe() {
        let reports: Vec<Report> = aoc::utils::parse_fields(TEST_DATA, '\n').unwrap();
        let expected = vec![true, false, false, false, false, true];
        let result: Vec<_> = reports.iter().map(|report| report.is_safe()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn is_damp_safe() {
        let reports: Vec<Report> = aoc::utils::parse_fields(TEST_DATA, '\n').unwrap();
        let expected = vec![true, false, false, true, true, true];
        let result: Vec<_> = reports.iter().map(|report| report.is_damp_safe()).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn drop_first() {
        let report: Report = "1 5 6 7".parse().unwrap();
        assert!(report.is_damp_safe());
    }

    #[test]
    fn drop_second() {
        let report: Report = "2 1 3 4".parse().unwrap();
        assert!(report.is_damp_safe());
    }

    #[test]
    fn drop_early() {
        let report: Report = "1 2 4 3 4".parse().unwrap();
        assert!(report.is_damp_safe());
    }

    #[test]
    fn drop_last() {
        let report: Report = "1 2 3 7".parse().unwrap();
        assert!(report.is_damp_safe());
    }
}
