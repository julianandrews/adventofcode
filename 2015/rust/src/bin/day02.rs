extern crate aoc;

use aoc::Result;
use std::io;
use std::io::{BufRead, Write};

fn main() -> Result<()> {
    let mut boxes: Vec<Box> = vec![];
    let input = io::stdin();
    for line in input.lock().lines() {
        boxes.push(Box::from_string(line?.trim())?);
    }

    writeln!(io::stdout(), "{}", part1(&boxes)?);
    writeln!(io::stdout(), "{}", part2(&boxes)?);

    Ok(())
}

struct Box {
    length: u64,
    width: u64,
    height: u64,
}

impl Box {
    fn from_string(s: &str) -> Result<Box> {
        let dimensions: Vec<u64> = s
            .split('x')
            .map(|x| x.parse())
            .collect::<std::result::Result<_, _>>()?;
        if dimensions.len() != 3 {
            return Err(format!("Failed to parse line: {}", s).into());
        }

        Ok(Box {
            length: dimensions[0],
            width: dimensions[1],
            height: dimensions[2],
        })
    }

    fn wrapping_paper(&self) -> u64 {
        let sides = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];

        2 * sides.iter().sum::<u64>() + sides.iter().min().unwrap()
    }

    fn ribbon(&self) -> u64 {
        let perimeters = [
            2 * (self.length + self.width),
            2 * (self.width + self.height),
            2 * (self.height + self.length),
        ];
        let volume = self.length * self.width * self.height;

        perimeters.iter().min().unwrap() + volume
    }
}

fn part1(boxes: &[Box]) -> Result<u64> {
    Ok(boxes.iter().map(Box::wrapping_paper).sum())
}

fn part2(boxes: &[Box]) -> Result<u64> {
    Ok(boxes.iter().map(Box::ribbon).sum())
}
