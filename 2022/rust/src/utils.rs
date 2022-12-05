use std::env;
use std::fs::File;
use std::io::{self, Read};
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

// TODO: See if I can use `std::str::Pattern` instead of `char`
pub fn parse_fields<T: FromStr>(
    data: &str,
    delimiter: char,
) -> Result<Vec<T>, <T as FromStr>::Err> {
    data.split(delimiter).map(&str::parse).collect()
}
