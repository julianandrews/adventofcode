use aoc::utils::parse_fields;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn simple_fuel(mass: i64) -> i64 {
    std::cmp::max(0, mass / 3 - 2)
}

fn fuel(mass: i64) -> i64 {
    let fuel_needed = |&m: &i64| Some(simple_fuel(m)).filter(|&m| m > 0);

    std::iter::successors(Some(simple_fuel(mass)), fuel_needed).sum()
}

fn part1(masses: &Vec<i64>) -> i64 {
    masses.iter().cloned().map(simple_fuel).sum()
}

fn part2(masses: &Vec<i64>) -> i64 {
    masses.iter().cloned().map(fuel).sum()
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let masses = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&masses));
    println!("Part 2: {}", part2(&masses));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_fuel_tests() {
        assert!(simple_fuel(12) == 2);
        assert!(simple_fuel(14) == 2);
        assert!(simple_fuel(1969) == 654);
        assert!(simple_fuel(100756) == 33583);
    }

    #[test]
    fn fuel_tests() {
        assert!(fuel(14) == 2);
        assert!(fuel(1969) == 966);
        assert!(fuel(100756) == 50346);
    }
}
