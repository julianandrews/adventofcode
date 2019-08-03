extern crate aoc;
extern crate crypto;

use aoc::Result;
use crypto::digest::Digest;
use std::io;
use std::io::{Read, Write};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let secret = input.trim();

    writeln!(io::stdout(), "{}", part1(secret)?);
    writeln!(io::stdout(), "{}", part2(secret)?);

    Ok(())
}

fn part1(secret: &str) -> Result<usize> {
    let secret = secret.as_bytes();
    let result: &mut [u8; 16] = &mut [0; 16];
    for i in 0.. {
        let mut digest = crypto::md5::Md5::new();
        digest.input(secret);
        digest.input(i.to_string().as_bytes());
        digest.result(result);
        if result[0] | result[1] | (result[2] >> 4) == 0 {
            return Ok(i);
        }
    }

    Err("Unexpected termination.".into())
}

fn part2(secret: &str) -> Result<usize> {
    let secret = secret.as_bytes();
    let result: &mut [u8; 16] = &mut [0; 16];
    for i in 0.. {
        let mut digest = crypto::md5::Md5::new();
        digest.input(secret);
        digest.input(i.to_string().as_bytes());
        digest.result(result);
        if result[0] | result[1] | result[2] == 0 {
            return Ok(i);
        }
    }

    Err("Unexpected termination.".into())
}
