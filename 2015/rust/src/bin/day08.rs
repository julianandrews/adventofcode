use aoc::utils::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;

    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));

    Ok(())
}

fn part1(s: &str) -> usize {
    s.lines().count() * 2 + extra_escape_length(s) // starting & ending quotes + escapes.
}

fn part2(s: &str) -> usize {
    s.lines().count() * 2 + needed_escape_count(s) // starting & ending quotes + escapes.
}

fn extra_escape_length(s: &str) -> usize {
    let mut total = 0;
    let mut i = 0;
    while i < s.len() {
        let extra = match s.as_bytes()[i..] {
            [b'\\', b'\\', ..] | [b'\\', b'"', ..] => 1,
            [b'\\', b'x', a, b, ..] if a.is_ascii_hexdigit() && b.is_ascii_hexdigit() => 3,
            _ => 0,
        };
        i += 1 + extra;
        total += extra;
    }
    total
}

fn needed_escape_count(s: &str) -> usize {
    s.bytes().filter(|&b| b == b'\\' || b == b'"').count()
}

#[cfg(test)]
mod tests {
    use super::{extra_escape_length, needed_escape_count};

    #[test]
    fn extra_escape_length_1() {
        let s = r#""""#;
        assert_eq!(extra_escape_length(s), 0);
    }

    #[test]
    fn extra_escape_length_2() {
        let s = r#""abc""#;
        assert_eq!(extra_escape_length(s), 0);
    }

    #[test]
    fn extra_escape_length_3() {
        let s = r#""aaa\"aaa""#;
        assert_eq!(extra_escape_length(s), 1);
    }

    #[test]
    fn extra_escape_length_4() {
        let s = r#""\x27""#;
        assert_eq!(extra_escape_length(s), 3);
    }

    #[test]
    fn escape_count_1() {
        let s = r#""""#;
        assert_eq!(needed_escape_count(s), 2);
    }

    #[test]
    fn escape_count_2() {
        let s = r#""abc""#;
        assert_eq!(needed_escape_count(s), 2);
    }

    #[test]
    fn escape_count_3() {
        let s = r#""aaa\"aaa""#;
        assert_eq!(needed_escape_count(s), 4);
    }

    #[test]
    fn escape_count_4() {
        let s = r#""\x27""#;
        assert_eq!(needed_escape_count(s), 3);
    }
}
