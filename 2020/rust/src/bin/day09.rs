use itertools::Itertools;
use std::collections::HashSet;

use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let decoder = Decoder::new(parse_fields(&input.trim(), '\n')?, 25)?;

    println!("Part 1: {}", part1(&decoder)?);
    println!("Part 2: {}", part2(&decoder)?);
    Ok(())
}

fn part1(decoder: &Decoder) -> Result<usize> {
    decoder
        .first_invalid()
        .ok_or(AOCError::new("No invalid numbers found").into())
}

fn part2(decoder: &Decoder) -> Result<usize> {
    decoder
        .find_weakness()
        .ok_or(AOCError::new("No weakness found").into())
}

struct Decoder {
    numbers: Vec<usize>,
    preamble_length: usize,
}

impl Decoder {
    fn new(numbers: Vec<usize>, preamble_length: usize) -> Result<Decoder> {
        if numbers.len() < preamble_length {
            return Err(AOCError::new("Invalid input"))?;
        }
        Ok(Decoder {
            numbers,
            preamble_length,
        })
    }

    fn is_valid(&self, n: usize) -> bool {
        if n < self.preamble_length || n >= self.numbers.len() {
            return false;
        }
        let sums: HashSet<usize> = self.numbers[n - self.preamble_length..n]
            .iter()
            .combinations(2)
            .filter(|pair| pair[0] != pair[1])
            .map(|pair| pair.into_iter().sum())
            .collect();
        sums.contains(&self.numbers[n])
    }

    fn first_invalid(&self) -> Option<usize> {
        (self.preamble_length..self.numbers.len())
            .filter(|&n| !self.is_valid(n))
            .map(|n| self.numbers[n])
            .next()
    }

    fn find_weakness(&self) -> Option<usize> {
        let target = self.first_invalid()?;
        let mut sum = 0;
        let mut start = 0;
        for end in 0..self.numbers.len() {
            sum += self.numbers[end];
            while sum > target {
                sum -= self.numbers[start];
                start += 1;
            }
            if sum == target {
                let smallest = self.numbers[start..=end].iter().min().unwrap();
                let largest = self.numbers[start..=end].iter().max().unwrap();
                return Some(smallest + largest);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_NUMBERS: [usize; 20] = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn first_invalid() {
        let decoder = Decoder::new(TEST_NUMBERS.to_vec(), 5).unwrap();
        assert_eq!(decoder.first_invalid(), Some(127));
    }

    #[test]
    fn find_weakness() {
        let decoder = Decoder::new(TEST_NUMBERS.to_vec(), 5).unwrap();
        assert_eq!(decoder.find_weakness(), Some(62));
    }
}
