use anyhow::{anyhow, Result};
use std::{convert::TryFrom, str::FromStr};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let hands: Vec<HandAndBid<Card>> = parse_fields(input.trim(), '\n')?;
    let joker_hands: Vec<HandAndBid<CardWithJokers>> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", total_winnings(hands));
    println!("Part 2: {}", total_winnings(joker_hands));

    Ok(())
}

fn total_winnings<CardType: std::cmp::Ord>(mut hands: Vec<HandAndBid<CardType>>) -> u64 {
    hands.sort_unstable();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct HandAndBid<CardType> {
    hand: Hand<CardType>,
    bid: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<CardType> {
    hand_type: HandType,
    cards: [CardType; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardWithJokers {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[Card; 5]> for HandType {
    fn from(hand: &[Card; 5]) -> HandType {
        let mut counts = [0; 13];
        for card in hand {
            counts[*card as usize] += 1;
        }
        counts.sort_unstable();
        HandType::from_card_counts(counts[12], counts[11]).expect("Impossible card counts")
    }
}

impl From<&[CardWithJokers; 5]> for HandType {
    fn from(hand: &[CardWithJokers; 5]) -> HandType {
        let mut counts = [0; 13];
        let mut joker_count = 0;
        for card in hand {
            match card {
                CardWithJokers::Joker => joker_count += 1,
                _ => counts[*card as usize] += 1,
            }
        }
        counts.sort_unstable();
        HandType::from_card_counts(counts[12] + joker_count, counts[11])
            .expect("Impossible card counts")
    }
}

impl HandType {
    fn from_card_counts(first: u64, second: u64) -> Option<HandType> {
        match (first, second) {
            (1, 1) => Some(HandType::HighCard),
            (2, 1) => Some(HandType::OnePair),
            (2, 2) => Some(HandType::TwoPair),
            (3, 1) => Some(HandType::ThreeOfAKind),
            (3, 2) => Some(HandType::FullHouse),
            (4, 1) => Some(HandType::FourOfAKind),
            (5, 0) => Some(HandType::FiveOfAKind),
            _ => None,
        }
    }
}

impl<CardType> FromStr for HandAndBid<CardType>
where
    Hand<CardType>: FromStr,
    anyhow::Error: From<<Hand<CardType> as FromStr>::Err>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_part, bid_part) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid input {}", s))?;
        Ok(HandAndBid {
            hand: hand_part.parse()?,
            bid: bid_part.parse().map_err(anyhow::Error::new)?,
        })
    }
}

impl<CardType> FromStr for Hand<CardType>
where
    CardType: TryFrom<char>,
    HandType: for<'a> From<&'a [CardType; 5]>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<CardType> = s
            .chars()
            .map(|c| c.try_into().map_err(|_| anyhow!("Invalid card {}", c)))
            .collect::<Result<_>>()?;
        let cards: [CardType; 5] = cards
            .try_into()
            .map_err(|_| anyhow!("Wrong number of cards in {}", s))?;
        let hand_type = HandType::from(&cards);
        Ok(Hand { hand_type, cards })
    }
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(anyhow!("Unrecognized card: {}", value)),
        }
    }
}

impl TryFrom<char> for CardWithJokers {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'J' => Ok(CardWithJokers::Joker),
            '2' => Ok(CardWithJokers::Two),
            '3' => Ok(CardWithJokers::Three),
            '4' => Ok(CardWithJokers::Four),
            '5' => Ok(CardWithJokers::Five),
            '6' => Ok(CardWithJokers::Six),
            '7' => Ok(CardWithJokers::Seven),
            '8' => Ok(CardWithJokers::Eight),
            '9' => Ok(CardWithJokers::Nine),
            'T' => Ok(CardWithJokers::Ten),
            'Q' => Ok(CardWithJokers::Queen),
            'K' => Ok(CardWithJokers::King),
            'A' => Ok(CardWithJokers::Ace),
            _ => Err(anyhow!("Unrecognized card: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483";

    #[test]
    fn regular_hands() {
        let hands: Vec<HandAndBid<Card>> = parse_fields(TEST_DATA, '\n').unwrap();
        let result = total_winnings(hands);

        assert_eq!(result, 6440);
    }

    #[test]
    fn joker_hands() {
        let hands: Vec<HandAndBid<CardWithJokers>> = parse_fields(TEST_DATA, '\n').unwrap();
        let result = total_winnings(hands);

        assert_eq!(result, 5905);
    }
}
