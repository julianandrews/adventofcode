use std::collections::BTreeMap;

use anyhow::{anyhow, Result};

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let data = input.trim().as_bytes();

    println!("Part 1: {}", part1(data)?);
    println!("Part 2: {}", part2(data)?);

    Ok(())
}

fn part1(data: &[u8]) -> Result<usize> {
    find_marker(data, 4).ok_or_else(|| anyhow!("Marker not found"))
}

fn part2(data: &[u8]) -> Result<usize> {
    find_marker(data, 14).ok_or_else(|| anyhow!("Marker not found"))
}

fn find_marker(data: &[u8], n: usize) -> Option<usize> {
    let mut counts = BTreeMap::new();

    // Add the first n-1 characters
    for b in data.get(0..n - 1)? {
        *counts.entry(b).or_insert(0) += 1
    }
    for (i, window) in data.windows(n).enumerate() {
        // Add the new character
        *counts.entry(window.last()?).or_insert(0) += 1;

        // Check if we've got n distinct characters
        if counts.len() == n {
            return Some(i + n);
        }

        // Remove the old character
        let old_byte = &window.first()?;
        let old_count = counts.remove(old_byte).expect("Old byte not found");
        if old_count > 1 {
            counts.insert(old_byte, old_count - 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_marker_1() {
        let data = b"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_marker(data, 4).unwrap(), 7);
    }

    #[test]
    fn packet_marker_2() {
        let data = b"bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_marker(data, 4).unwrap(), 5);
    }

    #[test]
    fn packet_marker_3() {
        let data = b"nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_marker(data, 4).unwrap(), 6);
    }

    #[test]
    fn packet_marker_4() {
        let data = b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_marker(data, 4).unwrap(), 10);
    }

    #[test]
    fn packet_marker_5() {
        let data = b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_marker(data, 4).unwrap(), 11);
    }

    #[test]
    fn message_marker_1() {
        let data = b"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_marker(data, 14).unwrap(), 19);
    }

    #[test]
    fn message_marker_2() {
        let data = b"bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_marker(data, 14).unwrap(), 23);
    }

    #[test]
    fn message_marker_3() {
        let data = b"nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_marker(data, 14).unwrap(), 23);
    }

    #[test]
    fn message_marker_4() {
        let data = b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_marker(data, 14).unwrap(), 29);
    }

    #[test]
    fn message_marker_5() {
        let data = b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_marker(data, 14).unwrap(), 26);
    }

    #[test]
    fn short_input() {
        let data = b"mjq";
        assert_eq!(find_marker(data, 4), None);
    }

    #[test]
    fn input_equals_len_no_match() {
        let data = b"mjqj";
        assert_eq!(find_marker(data, 4), None);
    }

    #[test]
    fn input_equals_len_match() {
        let data = b"mjqx";
        assert_eq!(find_marker(data, 4).unwrap(), 4);
    }

    #[test]
    fn match_at_end() {
        let data = b"mjmjqx";
        assert_eq!(find_marker(data, 4).unwrap(), 6);
    }

    #[test]
    fn n_zero() {
        let data = b"mjmj";
        assert_eq!(find_marker(data, 4), None);
    }

    #[test]
    fn n_one() {
        let data = b"mjmj";
        assert_eq!(find_marker(data, 1).unwrap(), 1);
    }
}
