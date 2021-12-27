#![feature(int_abs_diff)]

use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let positions: Vec<u64> = parse_fields(input.trim(), ',')?;

    println!("Part 1: {}", part1(&positions));
    println!("Part 2: {}", part2(&positions));
    Ok(())
}

fn part1(positions: &[u64]) -> u64 {
    min_fuel_cost(positions, simple_fuel_cost)
}

fn part2(positions: &[u64]) -> u64 {
    min_fuel_cost(positions, fuel_cost)
}

fn min_fuel_cost<F>(positions: &[u64], mut cost: F) -> u64
where
    F: FnMut(&[u64], u64) -> u64,
{
    let min = positions.iter().copied().min().unwrap_or(0);
    let max = positions.iter().copied().max().unwrap_or(0);
    (min..=max)
        .map(|target| cost(positions, target))
        .min()
        .unwrap_or(0)
}

fn simple_fuel_cost(positions: &[u64], target: u64) -> u64 {
    positions.iter().map(|&p| p.abs_diff(target)).sum()
}

fn fuel_cost(positions: &[u64], target: u64) -> u64 {
    positions
        .iter()
        .map(|&p| {
            let distance = p.abs_diff(target);
            distance * (distance + 1) / 2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: [u64; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_simple_fuel_cost() {
        assert_eq!(simple_fuel_cost(&TEST_DATA, 2), 37);
        assert_eq!(simple_fuel_cost(&TEST_DATA, 3), 39);
        assert_eq!(simple_fuel_cost(&TEST_DATA, 1), 41);
        assert_eq!(simple_fuel_cost(&TEST_DATA, 10), 71);
    }

    #[test]
    fn test_min_simple_fuel_cost() {
        assert_eq!(min_fuel_cost(&TEST_DATA, simple_fuel_cost), 37);
    }

    #[test]
    fn test_fuel_cost() {
        assert_eq!(fuel_cost(&TEST_DATA, 5), 168);
        assert_eq!(fuel_cost(&TEST_DATA, 2), 206);
    }

    #[test]
    fn test_min_fuel_cost() {
        assert_eq!(min_fuel_cost(&TEST_DATA, fuel_cost), 168);
    }
}
