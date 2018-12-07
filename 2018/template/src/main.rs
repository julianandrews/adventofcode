use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    writeln!(io::stdout(), "{}", 0)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    writeln!(io::stdout(), "{}", 0)?;
    Ok(())
}
