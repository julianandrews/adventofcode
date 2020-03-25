use std::error;
use std::fmt;

pub fn digits(s: &str) -> std::result::Result<Vec<u32>, ToDigitsError> {
    s.chars()
        .map(|c| c.to_digit(10).ok_or(ToDigitsError))
        .collect()
}

#[derive(Debug, Clone)]
pub struct ToDigitsError;

impl fmt::Display for ToDigitsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid string for digits")
    }
}

impl error::Error for ToDigitsError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
