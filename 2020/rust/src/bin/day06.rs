use std::collections::HashSet;
use std::str::FromStr;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let groups: Vec<PassengerGroup> = input
        .split("\n\n")
        .map(&str::parse)
        .collect::<Result<_>>()?;

    println!("Part 1: {}", part1(&groups));
    println!("Part 2: {}", part2(&groups));
    Ok(())
}

fn part1(groups: &[PassengerGroup]) -> usize {
    groups.iter().map(|group| group.union().len()).sum()
}

fn part2(groups: &[PassengerGroup]) -> usize {
    groups.iter().map(|group| group.intersection().len()).sum()
}

struct PassengerGroup {
    answers: Vec<HashSet<char>>,
}

impl PassengerGroup {
    fn union(&self) -> HashSet<char> {
        self.answers.iter().fold(HashSet::new(), |a, b| &a | b)
    }

    fn intersection(&self) -> HashSet<char> {
        self.answers
            .iter()
            .fold(('a'..='z').collect(), |a, b| &a & b)
    }
}

impl FromStr for PassengerGroup {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().any(|c| c != '\n' && !('a'..='z').contains(&c)) {
            return Err(AOCError::new("Invalid input"))?;
        }
        let answers = s.lines().map(|line| line.chars().collect()).collect();

        Ok(PassengerGroup { answers })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_1() {
        let group: PassengerGroup = "abc".parse().unwrap();
        let expected: HashSet<_> = vec!['a', 'b', 'c'].into_iter().collect();
        assert_eq!(group.union(), expected);
    }

    #[test]
    fn union_2() {
        let group: PassengerGroup = "a\nb\nc".parse().unwrap();
        let expected: HashSet<_> = vec!['a', 'b', 'c'].into_iter().collect();
        assert_eq!(group.union(), expected);
    }

    #[test]
    fn union_3() {
        let group: PassengerGroup = "ab\nac\n".parse().unwrap();
        let expected: HashSet<_> = vec!['a', 'b', 'c'].into_iter().collect();
        assert_eq!(group.union(), expected);
    }

    #[test]
    fn union_4() {
        let group: PassengerGroup = "a\na\na\na".parse().unwrap();
        let expected: HashSet<_> = vec!['a'].into_iter().collect();
        assert_eq!(group.union(), expected);
    }

    #[test]
    fn union_5() {
        let group: PassengerGroup = "b".parse().unwrap();
        let expected: HashSet<_> = vec!['b'].into_iter().collect();
        assert_eq!(group.union(), expected);
    }

    #[test]
    fn intersection_1() {
        let group: PassengerGroup = "abc".parse().unwrap();
        let expected: HashSet<_> = vec!['a', 'b', 'c'].into_iter().collect();
        assert_eq!(group.intersection(), expected);
    }

    #[test]
    fn intersection_2() {
        let group: PassengerGroup = "a\nb\nc".parse().unwrap();
        let expected: HashSet<_> = vec![].into_iter().collect();
        assert_eq!(group.intersection(), expected);
    }

    #[test]
    fn intersection_3() {
        let group: PassengerGroup = "ab\nac\n".parse().unwrap();
        let expected: HashSet<_> = vec!['a'].into_iter().collect();
        assert_eq!(group.intersection(), expected);
    }

    #[test]
    fn intersection_4() {
        let group: PassengerGroup = "a\na\na\na".parse().unwrap();
        let expected: HashSet<_> = vec!['a'].into_iter().collect();
        assert_eq!(group.intersection(), expected);
    }

    #[test]
    fn intersection_5() {
        let group: PassengerGroup = "b".parse().unwrap();
        let expected: HashSet<_> = vec!['b'].into_iter().collect();
        assert_eq!(group.intersection(), expected);
    }
}
