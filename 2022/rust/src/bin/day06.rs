use anyhow::{anyhow, Result};

use aoc::utils::get_input;

fn main() -> Result<()> {
    let data = get_input()?;

    println!("Part 1: {}", part1(&data)?);
    println!("Part 2: {}", part2(&data)?);

    Ok(())
}

fn part1(data: &str) -> Result<usize> {
    find_marker(data, 4).ok_or_else(|| anyhow!("Marker not found"))
}

fn part2(data: &str) -> Result<usize> {
    find_marker(data, 14).ok_or_else(|| anyhow!("Marker not found"))
}

fn find_marker(data: &str, n: usize) -> Option<usize> {
    let masks: Vec<_> = data.trim().bytes().map(|b| 1u32 << (b - b'a')).collect();
    let mut seen = 0u32;
    for (i, mask) in masks.iter().enumerate() {
        seen ^= mask;
        if i >= n {
            seen ^= masks[i - n];
        }
        if seen.count_ones() as usize == n {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_marker_1() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_marker(data, 4).unwrap(), 7);
    }

    #[test]
    fn packet_marker_2() {
        let data = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_marker(data, 4).unwrap(), 5);
    }

    #[test]
    fn packet_marker_3() {
        let data = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_marker(data, 4).unwrap(), 6);
    }

    #[test]
    fn packet_marker_4() {
        let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_marker(data, 4).unwrap(), 10);
    }

    #[test]
    fn packet_marker_5() {
        let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_marker(data, 4).unwrap(), 11);
    }

    #[test]
    fn message_marker_1() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_marker(data, 14).unwrap(), 19);
    }

    #[test]
    fn message_marker_2() {
        let data = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_marker(data, 14).unwrap(), 23);
    }

    #[test]
    fn message_marker_3() {
        let data = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_marker(data, 14).unwrap(), 23);
    }

    #[test]
    fn message_marker_4() {
        let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_marker(data, 14).unwrap(), 29);
    }

    #[test]
    fn message_marker_5() {
        let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_marker(data, 14).unwrap(), 26);
    }

    #[test]
    fn short_input() {
        let data = "mjq";
        assert_eq!(find_marker(data, 4), None);
    }

    #[test]
    fn input_equals_len_no_match() {
        let data = "mjqj";
        assert_eq!(find_marker(data, 4), None);
    }

    #[test]
    fn input_equals_len_match() {
        let data = "mjqx";
        assert_eq!(find_marker(data, 4).unwrap(), 4);
    }

    #[test]
    fn match_at_end() {
        let data = "mjmjqx";
        assert_eq!(find_marker(data, 4).unwrap(), 6);
    }

    #[test]
    fn n_zero() {
        let data = "mjmj";
        assert_eq!(find_marker(data, 4), None);
    }

    #[test]
    fn n_one() {
        let data = "mjmj";
        assert_eq!(find_marker(data, 1).unwrap(), 1);
    }
}
