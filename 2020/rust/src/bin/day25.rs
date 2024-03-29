#![feature(str_split_once)]

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Modulus to use for keys. Code below assumes MODULUS ** 2 < std::u64::max.
static MODULUS: u64 = 20201227;

fn main() -> Result<()> {
    let input = get_input()?;
    let (a, b) = input
        .trim()
        .split_once('\n')
        .ok_or(AOCError::new("Invalid input"))?;
    let card_pubkey: u64 = a.parse::<u64>()?;
    let door_pubkey: u64 = b.parse::<u64>()?;

    println!("Part 1: {}", part1(card_pubkey, door_pubkey)?);

    Ok(())
}

fn part1(card_pubkey: u64, door_pubkey: u64) -> Result<u64> {
    let card_loop_size = find_loop_size(card_pubkey).ok_or(AOCError::new("Loop size not found"))?;
    Ok(transform(door_pubkey, card_loop_size))
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    // Protect against multiplication overflow.
    let subject_number = subject_number % MODULUS;
    let loop_size = loop_size % MODULUS;
    (0..loop_size).fold(1, |value, _| value * subject_number % MODULUS)
}

fn find_loop_size(pubkey: u64) -> Option<u64> {
    let mut value = 1;
    // If a value ever repeats, there will be a cycle, so if we don't find
    // our key after MODULUS checks, we never will.
    for loop_size in 0..MODULUS {
        if value == pubkey {
            return Some(loop_size);
        }
        value = value * 7 % MODULUS;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_size() {
        assert_eq!(find_loop_size(17807724), Some(11));
        assert_eq!(find_loop_size(5764801), Some(8));
    }

    #[test]
    fn transform_works() {
        assert_eq!(transform(17807724, 8), 14897079);
    }
}
