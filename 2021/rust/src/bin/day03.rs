use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let numbers: Vec<&str> = input.trim().split('\n').collect();

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers)?);
    Ok(())
}

fn part1(numbers: &[&str]) -> u64 {
    gamma_rate(numbers) * epsilon_rate(numbers)
}

fn part2(numbers: &[&str]) -> Result<u64> {
    Ok(oxygen_generator_rating(numbers)? * co2_scrubber_rating(numbers)?)
}

fn gamma_rate(numbers: &[&str]) -> u64 {
    let num_digits = numbers.get(0).map(|s| s.len()).unwrap_or(0);
    (0..num_digits)
        .filter(|&i| one_is_most_common(numbers, i))
        .fold(0, |acc, i| acc | 1 << (num_digits - i - 1))
}

fn epsilon_rate(numbers: &[&str]) -> u64 {
    let num_digits = numbers.get(0).map(|s| s.len()).unwrap_or(0);
    !gamma_rate(numbers) & ((1 << num_digits) - 1)
}

fn oxygen_generator_rating(numbers: &[&str]) -> Result<u64> {
    bit_filter(numbers, one_is_most_common)
}

fn co2_scrubber_rating(numbers: &[&str]) -> Result<u64> {
    bit_filter(numbers, |ns, i| !one_is_most_common(ns, i))
}

fn one_is_most_common(numbers: &[&str], i: usize) -> bool {
    let count = numbers
        .iter()
        .filter(|&s| s.chars().nth(i).unwrap_or('0') == '1')
        .count();
    2 * count >= numbers.len()
}

fn bit_filter<C>(numbers: &[&str], mut criterion: C) -> Result<u64>
where
    C: FnMut(&[&str], usize) -> bool,
{
    let num_digits = numbers.get(0).map(|s| s.len()).unwrap_or(0);
    let mut numbers = numbers.to_vec();
    for i in 0..num_digits {
        if numbers.len() == 1 {
            break;
        }
        let digit = if criterion(&numbers, i) { '1' } else { '0' };
        numbers = numbers
            .into_iter()
            .filter(|s| s.chars().nth(i) == Some(digit))
            .collect();
    }
    let number = numbers.get(0).ok_or(AOCError::new("No number found"))?;
    Ok(u64::from_str_radix(&number, 2)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &[&str] = &[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_gamma_rate() {
        let result = gamma_rate(TEST_DATA);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_epsilon_rate() {
        let result = epsilon_rate(TEST_DATA);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_oxygen_generator_rating() {
        let result = oxygen_generator_rating(TEST_DATA).unwrap();
        assert_eq!(result, 23);
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let result = co2_scrubber_rating(TEST_DATA).unwrap();
        assert_eq!(result, 10);
    }
}
