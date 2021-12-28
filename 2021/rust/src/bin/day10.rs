use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let lines: Vec<_> = input.trim().lines().collect();

    println!("Part 1: {}", part1(&lines)?);
    println!("Part 2: {}", part2(&lines)?);
    Ok(())
}

fn part1(lines: &[&str]) -> Result<u64> {
    let mut score = 0;
    for line in lines {
        score += error_score(line)?;
    }
    Ok(score)
}

fn part2(lines: &[&str]) -> Result<u64> {
    let mut scores = vec![];
    for line in lines {
        let score = completion_score(line)?;
        if score != 0 {
            scores.push(score);
        }
    }
    scores.sort_unstable();
    Ok(scores[scores.len() / 2])
}
fn error_score(line: &str) -> Result<u64> {
    let mut stack = vec![];
    for b in line.bytes() {
        match b {
            b'(' | b'{' | b'[' | b'<' => stack.push(b),
            b')' | b'}' | b']' | b'>' => {
                if stack.pop() != Some(matching_brace(b)) {
                    return match b {
                        b')' => Ok(3),
                        b']' => Ok(57),
                        b'}' => Ok(1197),
                        b'>' => Ok(25137),
                        _ => unreachable!(),
                    };
                }
            }
            _ => return Err(Box::new(AOCError::new("Unexpected character"))),
        }
    }
    Ok(0)
}

fn completion_score(line: &str) -> Result<u64> {
    let mut stack = vec![];
    for b in line.bytes() {
        match b {
            b'(' | b'{' | b'[' | b'<' => stack.push(b),
            b')' | b'}' | b']' | b'>' => {
                if stack.pop() != Some(matching_brace(b)) {
                    return Ok(0);
                }
            }
            _ => return Err(Box::new(AOCError::new("Unexpected character"))),
        }
    }
    let mut score = 0;
    for b in stack.into_iter().rev() {
        score *= 5;
        score += match b {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => unreachable!(),
        }
    }
    Ok(score)
}

fn matching_brace(b: u8) -> u8 {
    match b {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => panic!("Called with non-brace!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: [&str; 10] = [
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn test_part_1() {
        let result = part1(&TEST_DATA).unwrap();
        assert_eq!(result, 26397);
    }

    #[test]
    fn completion_score_1() {
        let line = "[({(<(())[]>[[{[]{<()<>>";
        let result = completion_score(line).unwrap();
        assert_eq!(result, 288957);
    }

    #[test]
    fn completion_score_2() {
        let line = "[(()[<>])]({[<{<<[]>>(";
        let result = completion_score(line).unwrap();
        assert_eq!(result, 5566);
    }

    #[test]
    fn completion_score_3() {
        let line = "(((({<>}<{<{<>}{[]{[]{}";
        let result = completion_score(line).unwrap();
        assert_eq!(result, 1480781);
    }

    #[test]
    fn completion_score_4() {
        let line = "{<[[]]>}<{[{[{[]{()[[[]";
        let result = completion_score(line).unwrap();
        assert_eq!(result, 995444);
    }

    #[test]
    fn completion_score_5() {
        let line = "<{([{{}}[<[[[<>{}]]]>[]]";
        let result = completion_score(line).unwrap();
        assert_eq!(result, 294);
    }

    #[test]
    fn test_part_2() {
        let result = part2(&TEST_DATA).unwrap();
        assert_eq!(result, 288957);
    }
}
