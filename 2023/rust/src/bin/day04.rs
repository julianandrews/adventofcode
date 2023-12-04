use anyhow::{anyhow, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let cards: Vec<_> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&cards));
    println!("Part 2: {}", part2(&cards));

    Ok(())
}

fn part1(cards: &[Card]) -> u64 {
    cards.iter().map(|card| card.points()).sum()
}

fn part2(cards: &[Card]) -> usize {
    count_cards(cards).iter().sum()
}

fn count_cards(cards: &[Card]) -> Vec<usize> {
    let mut counts = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let card_count = counts[i];
        let score = card.score();
        for count in &mut counts[i + 1..i + 1 + score] {
            *count += card_count;
        }
    }
    counts
}

struct Card {
    _id: usize,
    winning_numbers: u128,
    numbers: u128,
}

impl Card {
    fn score(&self) -> usize {
        (self.numbers & self.winning_numbers).count_ones() as usize
    }

    fn points(&self) -> u64 {
        match self.score() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

impl std::str::FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn split_parts(s: &str) -> Option<(&str, &str, &str)> {
            let (id_part, numbers_part) = s.split_once(": ")?;
            let id = id_part.strip_prefix("Card ")?.trim();
            let (winning, numbers) = numbers_part.split_once(" | ")?;
            Some((id, winning, numbers))
        }

        fn parse_bitvec(s: &str) -> Result<u128> {
            s.split_whitespace().try_fold(0, |n, s| {
                Ok(n | 1u128
                    .checked_shl(s.parse::<u32>()?)
                    .ok_or_else(|| anyhow!("Invalid numbers: {}", s))?)
            })
        }

        let (id_part, winning_part, numbers_part) =
            split_parts(s).ok_or_else(|| anyhow!("Invalid card: {}", s))?;
        let _id = id_part.parse()?;
        let winning_numbers = parse_bitvec(winning_part)?;
        let numbers = parse_bitvec(numbers_part)?;

        Ok(Card {
            _id,
            winning_numbers,
            numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn points() {
        let cards: Vec<Card> = parse_fields(TEST_DATA, '\n').unwrap();
        let result: Vec<_> = cards.iter().map(|card| card.points()).collect();
        let expected = vec![8, 2, 2, 1, 0, 0];

        assert_eq!(result, expected);
    }

    #[test]
    fn counts() {
        let cards: Vec<Card> = parse_fields(TEST_DATA, '\n').unwrap();
        let result = count_cards(&cards);
        let expected = vec![1, 2, 4, 8, 14, 1];

        assert_eq!(result, expected);
    }
}
