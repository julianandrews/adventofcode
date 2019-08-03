extern crate aoc;

use aoc::Result;
use std::io::{self, Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input.trim();

    writeln!(io::stdout(), "{}", part1(&input)?);
    writeln!(io::stdout(), "{}", part2(&input)?);
    Ok(())
}

fn floor_offset(c: char) -> Result<i64> {
    if c == '(' {
        return Ok(1);
    } else if c == ')' {
        return Ok(-1);
    } else {
        return Err(format!("Unrecognized character: {}", c).into());
    }
}

fn part1(input: &str) -> Result<i64> {
    let mut floor: i64 = 0;

    for c in input.chars() {
        floor += floor_offset(c)?;
    }

    Ok(floor)
}

fn part2(input: &str) -> Result<usize> {
    let mut floor: i64 = 0;

    for (i, c) in input.chars().enumerate() {
        floor += floor_offset(c)?;
        if floor < 0 {
            return Ok(i + 1);
        }
    }

    Err("Never entered the basement".into())
}
