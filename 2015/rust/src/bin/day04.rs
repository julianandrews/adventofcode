use anyhow::Result;
use md5::{Digest, Md5};

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let secret = input.trim();

    println!("Part 1: {}", part1(secret));
    println!("Part 2: {}", part2(secret));

    Ok(())
}

fn part1(secret: &str) -> usize {
    find_hash(secret, ends_in_five_zeroes)
}

fn part2(secret: &str) -> usize {
    find_hash(secret, ends_in_siz_zeros)
}

fn find_hash(secret: &str, predicate: fn(&[u8]) -> bool) -> usize {
    let base_hasher = Md5::new_with_prefix(secret);
    for i in 0.. {
        let mut hasher = base_hasher.clone();
        hasher.update(i.to_string());
        if predicate(&hasher.finalize()) {
            return i;
        }
    }
    unreachable!()
}

fn ends_in_five_zeroes(hash: &[u8]) -> bool {
    hash[0] | hash[1] | (hash[2] >> 4) == 0
}

fn ends_in_siz_zeros(hash: &[u8]) -> bool {
    hash[0] | hash[1] | hash[2] == 0
}

#[cfg(test)]
mod tests {
    use super::{ends_in_five_zeroes, find_hash};

    #[test]
    fn five_zeros_1() {
        assert_eq!(find_hash("abcdef", ends_in_five_zeroes), 609043);
    }

    #[test]
    fn five_zeros_2() {
        assert_eq!(find_hash("pqrstuv", ends_in_five_zeroes), 1048970);
    }
}
