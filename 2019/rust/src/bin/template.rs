extern crate aoc;

use aoc::aoc_error::AOCError;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn part1() -> Result<u64> {
    Err(AOCError::new("Unimplemented"))?
}

fn part2() -> Result<u64> {
    Err(AOCError::new("Unimplemented"))?
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1()?);
    println!("Part 2: {}", part2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case() {}
}
