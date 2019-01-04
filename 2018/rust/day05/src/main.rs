use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn collapse(input: impl Iterator<Item = u8>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    for byte in input.filter(|&v| (v as char).is_ascii_alphabetic()) {
        bytes.push(byte);
        while bytes.len() >= 2
            && (bytes[bytes.len() - 1] as i8 - bytes[bytes.len() - 2] as i8).abs() == 32
        {
            bytes.pop();
            bytes.pop();
        }
    }

    bytes
}

fn part1(input: &str) -> Result<()> {
    let result: usize = collapse(input.bytes()).len();
    writeln!(io::stdout(), "{}", result)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let shortest_polymer_length: usize = (b'A'..b'Z')
        .map(|c| collapse(input.bytes().filter(|x| !c.eq_ignore_ascii_case(x))).len())
        .min()
        .unwrap();

    writeln!(io::stdout(), "{}", shortest_polymer_length)?;
    Ok(())
}
