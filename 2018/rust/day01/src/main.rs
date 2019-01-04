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
    let frequency: isize = input.lines().flat_map(|line| line.parse::<isize>()).sum();

    writeln!(io::stdout(), "{}", frequency)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut frequency = 0;
    let mut seen = ::std::collections::HashSet::new();
    for offset in input.lines().flat_map(|line| line.parse::<isize>()).cycle() {
        frequency += offset;
        if seen.contains(&frequency) {
            break;
        } else {
            seen.insert(frequency);
        }
    }

    writeln!(io::stdout(), "{}", frequency)?;
    Ok(())
}
