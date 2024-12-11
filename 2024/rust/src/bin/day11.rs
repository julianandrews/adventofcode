#![feature(strict_overflow_ops)]
#![feature(iterator_try_collect)]

use std::cell::RefCell;

use anyhow::Result;
use rustc_hash::FxHashMap;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let values: Vec<usize> = aoc::utils::parse_fields(input.trim(), ' ')?;

    println!("Part 1: {}", part1(&values));
    println!("Part 2: {}", part2(&values));

    Ok(())
}

fn part1(values: &[usize]) -> usize {
    values.iter().map(|&value| stone_count(value, 25)).sum()
}

fn part2(values: &[usize]) -> usize {
    values.iter().map(|&value| stone_count(value, 75)).sum()
}

fn blink(value: usize) -> BlinkResult {
    if value == 0 {
        return BlinkResult::Single(1);
    }
    let digits = digit_count(value);
    if digits % 2 == 0 {
        let pow = 10_i32.pow(digits / 2) as usize;
        BlinkResult::Pair(value / pow, value % pow)
    } else {
        BlinkResult::Single(value.strict_mul(2024))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlinkResult {
    Single(usize),
    Pair(usize, usize),
}

fn stone_count(value: usize, blinks: usize) -> usize {
    static CACHE_SIZE: usize = 75;
    thread_local! {
        static CACHE: RefCell<Vec<FxHashMap<usize, usize>>> = RefCell::new(vec![FxHashMap::default(); CACHE_SIZE]);
    }

    if blinks == 0 {
        return 1;
    }
    CACHE.with(|cache| {
        if let Some(map) = cache.borrow().get(blinks) {
            if let Some(count) = map.get(&value) {
                return *count;
            }
        }
        let count = match blink(value) {
            BlinkResult::Single(a) => stone_count(a, blinks - 1),
            BlinkResult::Pair(a, b) => stone_count(a, blinks - 1) + stone_count(b, blinks - 1),
        };
        if let Some(map) = cache.borrow_mut().get_mut(blinks) {
            map.insert(value, count);
        }
        count
    })
}

fn digit_count(mut x: usize) -> u32 {
    let mut count = 0;
    while x > 0 {
        x /= 10;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blinks() {
        assert_eq!(blink(0), BlinkResult::Single(1));
        assert_eq!(blink(1), BlinkResult::Single(2024));
        assert_eq!(blink(10), BlinkResult::Pair(1, 0));
        assert_eq!(blink(99), BlinkResult::Pair(9, 9));
        assert_eq!(blink(999), BlinkResult::Single(2021976));
    }

    #[test]
    fn stone_counts() {
        assert_eq!(stone_count(125, 6), 7);
        assert_eq!(stone_count(17, 6), 15);
    }
}
