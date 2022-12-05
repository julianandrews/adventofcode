use anyhow::Result;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let calories: Vec<Vec<u32>> = input
        .trim()
        .split("\n\n")
        .map(|s| parse_fields(s, '\n'))
        .collect::<std::result::Result<_, _>>()?;

    println!("Part 1: {}", part1(&calories));
    println!("Part 2: {}", part2(&calories));

    Ok(())
}

fn part1(calories: &[Vec<u32>]) -> u32 {
    max_calories(calories, 1)
}

fn part2(calories: &[Vec<u32>]) -> u32 {
    max_calories(calories, 3)
}

fn max_calories<T: AsRef<[u32]>>(calories: &[T], n: usize) -> u32 {
    let mut calories: Vec<_> = calories.iter().map(|x| x.as_ref().iter().sum()).collect();
    calories.sort();
    calories.iter().rev().take(n).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &[&[u32]] = &[
        &[1000, 2000, 3000],
        &[4000],
        &[5000, 6000],
        &[7000, 8000, 9000],
        &[10000],
    ];

    #[test]
    fn max_1() {
        let value = max_calories(&TEST_DATA, 1);
        assert_eq!(value, 24000);
    }

    #[test]
    fn max_3() {
        let value = max_calories(&TEST_DATA, 3);
        assert_eq!(value, 45000);
    }
}
