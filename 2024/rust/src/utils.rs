use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::str::pattern::Pattern;
use std::str::FromStr;

pub fn get_input() -> std::io::Result<String> {
    let args: Vec<String> = env::args().collect();
    let mut reader: Box<dyn io::Read> = if args.len() == 2 && args[1] != "-" {
        Box::new(io::BufReader::new(File::open(&args[1])?))
    } else {
        Box::new(io::stdin())
    };

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    Ok(input)
}

pub fn parse_fields<P: Pattern, T: FromStr, C: FromIterator<T>>(
    data: &str,
    pat: P,
) -> Result<C, <T as FromStr>::Err> {
    data.split(pat).map(&str::parse).collect()
}
