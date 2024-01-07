use anyhow::Result;
use rustc_hash::FxHashMap;

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;

    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
    Ok(())
}

fn part1(s: &str) -> usize {
    s.lines().filter(|line| is_stupid_nice(line)).count()
}

fn part2(s: &str) -> usize {
    s.lines().filter(|line| is_nice(line)).count()
}

fn is_stupid_nice(s: &str) -> bool {
    static VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
    static FORBIDDEN_PAIRS: [[u8; 2]; 4] = [[b'a', b'b'], [b'c', b'd'], [b'p', b'q'], [b'x', b'y']];
    let vowel_count = s.bytes().filter(|b| VOWELS.contains(b)).count();
    let has_pair = s.as_bytes().windows(2).any(|pair| pair[0] == pair[1]);
    let has_forbidden_pair = s
        .as_bytes()
        .windows(2)
        .any(|pair| FORBIDDEN_PAIRS.contains(pair.try_into().unwrap()));

    vowel_count >= 3 && has_pair && !has_forbidden_pair
}

fn is_nice(s: &str) -> bool {
    let has_double_pair = 'has_double_pair: {
        let mut pairs = FxHashMap::default();
        for (i, pair) in s.as_bytes().windows(2).enumerate() {
            let j = *pairs.entry(pair).or_insert(i);
            if i > j + 1 {
                break 'has_double_pair true;
            }
        }
        false
    };
    let has_skip_pair = s.as_bytes().windows(3).any(|triple| triple[0] == triple[2]);

    has_double_pair && has_skip_pair
}

#[cfg(test)]
mod tests {
    use super::{is_nice, is_stupid_nice};

    #[test]
    fn is_stupid_nice_1() {
        assert!(is_stupid_nice("ugknbfddgicrmopn"));
    }

    #[test]
    fn is_stupid_nice_2() {
        assert!(is_stupid_nice("aaa"));
    }

    #[test]
    fn is_stupid_nice_3() {
        assert!(!is_stupid_nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn is_stupid_nice_4() {
        assert!(!is_stupid_nice("haegwjzuvuyypxyu"));
    }

    #[test]
    fn is_stupid_nice_5() {
        assert!(!is_stupid_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn is_nice_1() {
        assert!(is_nice("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn is_nice_2() {
        assert!(is_nice("xxyxx"));
    }

    #[test]
    fn is_nice_3() {
        assert!(!is_nice("uurcxstgmygtbstg"));
    }

    #[test]
    fn is_nice_4() {
        assert!(!is_nice("ieodomkazucvgmuy"));
    }
}
