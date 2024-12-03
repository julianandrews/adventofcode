use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(memory: &str) -> u64 {
    simple_muls(memory).iter().map(|(a, b)| a * b).sum()
}

fn part2(memory: &str) -> u64 {
    real_muls(memory).iter().map(|(a, b)| a * b).sum()
}

fn simple_muls(memory: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"mul\((?<left>[1-9][0-9]*),(?<right>[1-9][0-9]*)\)").unwrap();
    re.captures_iter(memory)
        .map(|caps| {
            (
                caps["left"].parse().unwrap(),
                caps["right"].parse().unwrap(),
            )
        })
        .collect()
}

fn real_muls(memory: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"(mul\((?<left>[1-9][0-9]*),(?<right>[1-9][0-9]*)\)|don't\(\)|do\(\))")
        .unwrap();
    let mut enabled = true;
    re.captures_iter(memory)
        .filter_map(|caps| {
            match &caps[0] {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        return Some((
                            caps["left"].parse().unwrap(),
                            caps["right"].parse().unwrap(),
                        ));
                    }
                }
            }
            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn muls() {
        let result =
            simple_muls("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(result, expected);
    }

    #[test]
    fn real() {
        let result =
            real_muls("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        let expected = vec![(2, 4), (8, 5)];
        assert_eq!(result, expected);
    }
}
