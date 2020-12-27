use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let digits: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).map(|x| x as usize))
        .collect::<Option<_>>()
        .ok_or(AOCError::new("Invalid input"))?;

    println!("Part 1 {}", part1(Cups::new(&digits, digits.len())?));
    println!("Part 2 {}", part2(Cups::new(&digits, 1_000_000)?));

    Ok(())
}

fn part1(mut cups: Cups) -> String {
    (0..100).for_each(|_| cups.step());
    cups.starting_from(1)
        .skip(1)
        .map(|x| x.to_string())
        .collect()
}

fn part2(mut cups: Cups) -> usize {
    (0..10_000_000).for_each(|_| cups.step());
    cups.starting_from(1).skip(1).take(2).product()
}

struct Cups {
    next: Vec<usize>,
    current: usize,
}

impl Cups {
    fn new(seed: &[usize], length: usize) -> Result<Self> {
        if length < seed.len() || length < 3 {
            return Err(AOCError::new("Invalid length").into());
        }
        let max_digit = *seed.iter().max().unwrap();
        let cups: Vec<_> = seed
            .iter()
            .copied()
            .chain((max_digit + 1..).take(length - seed.len()))
            .collect();
        let mut next: Vec<_> = vec![0; length + 1].into_iter().collect();
        for (i, &cup) in cups.iter().enumerate() {
            next[cup] = cups[(i + 1) % length];
        }
        let current = seed[0];
        Ok(Cups { next, current })
    }

    fn step(&mut self) {
        let a = self.next[self.current];
        let b = self.next[a];
        let c = self.next[b];

        let mut destination = self.current - 1;
        while destination == a || destination == b || destination == c || destination == 0 {
            destination = (destination + self.next.len() - 1) % self.next.len();
        }

        let next_current = self.next[c];
        self.next[c] = self.next[destination];
        self.next[destination] = a;
        self.next[self.current] = next_current;
        self.current = next_current;
    }

    fn starting_from<'a>(&'a self, start: usize) -> impl Iterator<Item = usize> + 'a {
        (1..self.next.len()).scan(start, move |last, _| {
            let value = *last;
            *last = self.next[*last];
            Some(value)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let mut cups: Cups = Cups::new(&vec![3, 8, 9, 1, 2, 5, 4, 6, 7], 9).unwrap();
        (0..100).for_each(|_| cups.step());
        let result: Vec<_> = cups.starting_from(1).skip(1).collect();
        assert_eq!(result, vec![6, 7, 3, 8, 4, 5, 2, 9]);
    }

    #[test]
    fn full_case() {
        let mut cups: Cups = Cups::new(&vec![3, 8, 9, 1, 2, 5, 4, 6, 7], 1_000_000).unwrap();
        (0..10_000_000).for_each(|_| cups.step());
        let result: usize = cups.starting_from(1).skip(1).take(2).product();
        assert_eq!(result, 149245887792);
    }
}
