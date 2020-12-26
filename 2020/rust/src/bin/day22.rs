use std::collections::{HashSet, VecDeque};

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let (deck1, deck2) = parse_input(&input)?;

    println!("Part 1 {}", part1(deck1.clone(), deck2.clone()));
    println!("Part 2 {}", part2(deck1.clone(), deck2.clone()));

    Ok(())
}

fn part1(mut deck1: Deck, mut deck2: Deck) -> usize {
    match play_simple_game(&mut deck1, &mut deck2) {
        Winner::Player1 => deck1.score(),
        Winner::Player2 => deck2.score(),
    }
}

fn part2(mut deck1: Deck, mut deck2: Deck) -> usize {
    match play_game(&mut deck1, &mut deck2) {
        Winner::Player1 => deck1.score(),
        Winner::Player2 => deck2.score(),
    }
}

fn play_simple_game(deck1: &mut Deck, deck2: &mut Deck) -> Winner {
    loop {
        if deck2.is_empty() {
            return Winner::Player1;
        } else if deck1.is_empty() {
            return Winner::Player2;
        }
        let (card1, card2) = (deck1.draw().unwrap(), deck2.draw().unwrap());
        if card1 > card2 {
            deck1.collect(card1, card2);
        } else {
            deck2.collect(card2, card1);
        }
    }
}

fn play_game(deck1: &mut Deck, deck2: &mut Deck) -> Winner {
    let mut seen_decks = HashSet::new();
    loop {
        if deck2.is_empty() || seen_decks.contains(deck1) {
            return Winner::Player1;
        } else if deck1.is_empty() {
            return Winner::Player2;
        }
        seen_decks.insert(deck1.clone());
        let (card1, card2) = (deck1.draw().unwrap(), deck2.draw().unwrap());
        let round_winner = if card1 <= deck1.len() && card2 <= deck2.len() {
            play_game(&mut deck1.subdeck(card1), &mut deck2.subdeck(card2))
        } else if card1 > card2 {
            Winner::Player1
        } else {
            Winner::Player2
        };
        match round_winner {
            Winner::Player1 => deck1.collect(card1, card2),
            Winner::Player2 => deck2.collect(card2, card1),
        }
    }
}

fn parse_input(input: &str) -> Result<(Deck, Deck)> {
    let decks: Vec<_> = input
        .split("\n\n")
        .map(|s| s.lines().skip(1).map(|line| line.parse()).collect())
        .collect::<std::result::Result<Vec<_>, _>>()?;
    if decks.len() != 2 {
        return Err(AOCError::new("Invalid input").into());
    }
    let mut decks = decks.into_iter();

    Ok((
        Deck {
            cards: decks.next().unwrap(),
        },
        Deck {
            cards: decks.next().unwrap(),
        },
    ))
}

#[derive(Debug, Clone, PartialEq, Eq, std::hash::Hash)]
struct Deck {
    cards: VecDeque<usize>,
}

impl Deck {
    fn score(&self) -> usize {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &c)| (i + 1) * c as usize)
            .sum()
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn draw(&mut self) -> Option<usize> {
        self.cards.pop_front()
    }

    fn collect(&mut self, card1: usize, card2: usize) {
        self.cards.push_back(card1);
        self.cards.push_back(card2);
    }

    fn subdeck(&self, length: usize) -> Deck {
        let cards = self.cards.iter().take(length).cloned().collect();
        Deck { cards }
    }
}

#[derive(Debug, PartialEq)]
enum Winner {
    Player1,
    Player2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_game() {
        let mut deck1 = Deck {
            cards: vec![9, 2, 6, 3, 1].into_iter().collect(),
        };
        let mut deck2 = Deck {
            cards: vec![5, 8, 4, 7, 10].into_iter().collect(),
        };
        let winner = play_simple_game(&mut deck1, &mut deck2);
        assert_eq!(winner, Winner::Player2);
        assert_eq!(deck2.score(), 306);
    }

    #[test]
    fn recursive_game() {
        let mut deck1 = Deck {
            cards: vec![9, 2, 6, 3, 1].into_iter().collect(),
        };
        let mut deck2 = Deck {
            cards: vec![5, 8, 4, 7, 10].into_iter().collect(),
        };
        let winner = play_game(&mut deck1, &mut deck2);
        assert_eq!(winner, Winner::Player2);
        assert_eq!(deck2.score(), 291);
    }
}
