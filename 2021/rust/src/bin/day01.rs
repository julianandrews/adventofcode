use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let depths = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&depths));
    println!("Part 2: {}", part2(&depths));
    Ok(())
}

fn part1(depths: &[u64]) -> usize {
    count_increases(depths)
}

fn part2(depths: &[u64]) -> usize {
    let sums: Vec<_> = depths.windows(3).map(|x| x.iter().sum()).collect();
    count_increases(&sums)
}

fn count_increases(items: &[u64]) -> usize {
    items.windows(2).filter(|pair| pair[1] > pair[0]).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &[u64] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn count_simple_increases() {
        let result = count_increases(TEST_DATA);
        assert_eq!(result, 7);
    }

    #[test]
    fn count_window_increases() {
        let result = part2(TEST_DATA);
        assert_eq!(result, 5);
    }
}
