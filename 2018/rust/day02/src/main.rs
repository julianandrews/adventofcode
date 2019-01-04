use std::io::{self, Read, Write};
use std::collections::HashMap;
use std::hash::Hash;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

pub fn get_counts<K: Hash + Eq>(iter: impl Iterator<Item = K>) -> HashMap<K, usize> {
    let mut counter = HashMap::new();
    for key in iter {
        *counter.entry(key).or_insert(0) += 1;
    }

    counter
}

fn part1(input: &str) -> Result<()> {
    let (mut two_counts, mut three_counts) = (0, 0);

    for line in input.lines() {
        let counter = get_counts(line.bytes());
        if counter.values().any(|&b| b == 2) { two_counts += 1 };
        if counter.values().any(|&b| b == 3) { three_counts += 1 };
    }

    writeln!(io::stdout(), "{}", two_counts * three_counts)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let lines: Vec<&str> = input.lines().collect();

    for (i, first) in lines.iter().enumerate() {
        for second in lines[i+1..].iter() {
            let shared: String = first.chars()
                .zip(second.chars())
                .filter(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect();
            if shared.len() == first.len() - 1 {
                writeln!(io::stdout(), "{}", shared)?;
            }
        }
    }

    Ok(())
}
