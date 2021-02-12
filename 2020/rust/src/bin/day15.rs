use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let numbers = parse_fields(input.trim(), ',')?;

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
    Ok(())
}

fn part1(numbers: &[usize]) -> usize {
    nth_number(numbers, 2020)
}

fn part2(numbers: &[usize]) -> usize {
    nth_number(numbers, 30_000_000)
}

fn nth_number(numbers: &[usize], n: usize) -> usize {
    NumberIterator::new(numbers.to_vec(), n)
        .nth(n - 1)
        .expect("Iterator ended unexpectedly")
}

struct NumberIterator {
    starting_numbers: Vec<usize>,
    i: usize,
    last_number: usize,
    last_seen: Vec<Option<usize>>,
}

impl NumberIterator {
    // Using ceiling lets us perform just a single allocation, which saves a ton of time.
    fn new(starting_numbers: Vec<usize>, ceiling: usize) -> Self {
        NumberIterator {
            starting_numbers,
            i: 0,
            last_number: 0,
            last_seen: vec![None; ceiling],
        }
    }
}

impl Iterator for NumberIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let n = match self.starting_numbers.get(self.i) {
            Some(n) => *n,
            None => match self.last_seen.get(self.last_number) {
                Some(&Some(x)) => self.i - 1 - x,
                Some(None) => 0,
                None => return None,
            },
        };
        if self.i > 0 {
            self.last_seen[self.last_number] = Some(self.i - 1);
        }
        self.last_number = n;
        self.i += 1;
        Some(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_iterator() {
        let mut iterator = NumberIterator::new(vec![0, 3, 6], 10);
        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(6));
        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), Some(0));
    }

    #[test]
    fn nth_fast() {
        assert_eq!(nth_number(&vec![1, 3, 2], 2020), 1);
        assert_eq!(nth_number(&vec![2, 1, 3], 2020), 10);
        assert_eq!(nth_number(&vec![1, 2, 3], 2020), 27);
        assert_eq!(nth_number(&vec![2, 3, 1], 2020), 78);
        assert_eq!(nth_number(&vec![3, 2, 1], 2020), 438);
        assert_eq!(nth_number(&vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    #[ignore]
    fn nth_slow() {
        assert_eq!(nth_number(&vec![0, 3, 6], 30_000_000), 175594);
        assert_eq!(nth_number(&vec![1, 3, 2], 30_000_000), 2578);
        assert_eq!(nth_number(&vec![2, 1, 3], 30_000_000), 3544142);
        assert_eq!(nth_number(&vec![1, 2, 3], 30_000_000), 261214);
        assert_eq!(nth_number(&vec![2, 3, 1], 30_000_000), 6895259);
        assert_eq!(nth_number(&vec![3, 2, 1], 30_000_000), 18);
        assert_eq!(nth_number(&vec![3, 1, 2], 30_000_000), 362);
    }
}
