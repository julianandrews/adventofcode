use std::result::Result;
use std::str::FromStr;

// TODO: See if I can use `std::str::Pattern` instead of `char`
pub fn parse_fields<T: FromStr>(
    data: &str,
    delimiter: char,
) -> Result<Vec<T>, <T as FromStr>::Err> {
    data.split(delimiter).map(&str::parse).collect()
}
