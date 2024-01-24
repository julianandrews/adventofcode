fn main() -> anyhow::Result<()> {
    let input = aoc::utils::get_input()?;
    let (row, column) = parsing::parse_input(input.trim())?;

    println!("Part 1: {}", part1(row, column));

    Ok(())
}

fn part1(row: u64, column: u64) -> u64 {
    code(row, column)
}

fn code(row: u64, column: u64) -> u64 {
    let mut code = 20151125;
    for _ in 0..index(row, column) - 1 {
        code = next_code(code)
    }
    code
}

fn index(row: u64, column: u64) -> u64 {
    fn arithmetic_sum(start: u64, end: u64) -> u64 {
        let n = end + 1 - start;
        let a = start;
        n * (2 * a + n - 1) / 2
    }

    arithmetic_sum(0, column) + arithmetic_sum(column, column + row - 2)
}

fn next_code(code: u64) -> u64 {
    (code * 252533) % 33554393
}

mod parsing {
    use anyhow::{anyhow, Result};

    pub fn parse_input(s: &str) -> Result<(u64, u64)> {
        let (row, column) = s
            .strip_prefix(
                "To continue, please consult the code grid in the manual.  Enter the code at row ",
            )
            .and_then(|s| s.strip_suffix('.'))
            .and_then(|s| s.split_once(", column "))
            .ok_or_else(|| anyhow!("Invalid input {}.", s))?;
        Ok((row.parse()?, column.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_1() {
        assert_eq!(index(4, 3), 18);
    }

    #[test]
    fn index_2() {
        assert_eq!(index(4, 1), 7);
    }

    #[test]
    fn index_3() {
        assert_eq!(index(2, 5), 20);
    }

    #[test]
    fn next_code_1() {
        assert_eq!(next_code(20151125), 31916031);
    }

    #[test]
    fn code_1() {
        assert_eq!(code(1, 1), 20151125);
    }

    #[test]
    fn code_2() {
        assert_eq!(code(5, 2), 17552253);
    }

    #[test]
    fn code_3() {
        assert_eq!(code(2, 6), 4041754);
    }
}
