use anyhow::{anyhow, Result};

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let lines: Vec<_> = input.lines().collect();

    println!("Part 1: {}", part1(&lines)?);
    println!("Part 2: {}", part2(&lines)?);

    Ok(())
}

fn part1(lines: &[&str]) -> Result<u32> {
    lines.iter().try_fold(0, |total, line| {
        Ok(total + fake_calibration_value(line).ok_or_else(|| anyhow!("No value for {}", line))?)
    })
}

fn part2(lines: &[&str]) -> Result<u32> {
    lines.iter().try_fold(0, |total, line| {
        Ok(total + calibration_value(line).ok_or_else(|| anyhow!("No value for {}", line))?)
    })
}

fn fake_calibration_value(s: &str) -> Option<u32> {
    let digits: Vec<_> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    Some(10 * digits.first()? + digits.last()?)
}

fn calibration_value(s: &str) -> Option<u32> {
    #[rustfmt::skip]
    static DIGITS: [(&str, u32); 18] = [
        ("one", 1), ("1", 1), ("two", 2), ("2", 2), ("three", 3), ("3", 3),
        ("four", 4), ("4", 4), ("five", 5), ("5", 5), ("six", 6), ("6", 6),
        ("seven", 7), ("7", 7), ("eight", 8), ("8", 8), ("nine", 9), ("9", 9),
    ];
    let digit_at = |i| -> Option<u32> {
        let (_, substr) = s.split_at(i);
        DIGITS
            .iter()
            .filter(|&(word, _)| substr.starts_with(word))
            .map(|&(_, n)| n)
            .next()
    };
    let first_digit = (0..s.len()).find_map(digit_at)?;
    let last_digit = (0..s.len()).rev().find_map(digit_at)?;
    Some(10 * first_digit + last_digit)
}

#[cfg(test)]
mod tests {
    use super::{calibration_value, fake_calibration_value};

    #[test]
    fn fake_calibration() {
        assert_eq!(fake_calibration_value("1abc2"), Some(12));
        assert_eq!(fake_calibration_value("pqr3stu8vwx"), Some(38));
        assert_eq!(fake_calibration_value("a1b2c3d4e5f"), Some(15));
        assert_eq!(fake_calibration_value("treb7uchet"), Some(77));
    }

    #[test]
    fn real_calibration() {
        assert_eq!(calibration_value("two1nine"), Some(29));
        assert_eq!(calibration_value("eightwothree"), Some(83));
        assert_eq!(calibration_value("abcone2threexyz"), Some(13));
        assert_eq!(calibration_value("xtwone3four"), Some(24));
        assert_eq!(calibration_value("4nineeightseven2"), Some(42));
        assert_eq!(calibration_value("zoneight234"), Some(14));
        assert_eq!(calibration_value("7pqrstsixteen"), Some(76));
    }
}
