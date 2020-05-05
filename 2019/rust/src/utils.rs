use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::result::Result;
use std::str::FromStr;

pub fn get_input() -> Result<String, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut filename: Option<String> = None;
    if args.len() == 2 && args[1] != "-" {
        filename = Some(args[1].clone());
    }
    let mut reader: Box<dyn io::Read> = match &filename {
        Some(filename) => Box::new(io::BufReader::new(File::open(&filename)?)),
        None => Box::new(io::stdin()),
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
