extern crate aoc;

use aoc::aoc_error::AOCError;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Deck {
    size: u64,
    stride: u64,
    offset: u64,
}

impl std::ops::Add for Deck {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut deck = self;
        deck += other;
        deck
    }
}
impl std::ops::AddAssign for Deck {
    fn add_assign(&mut self, other: Self) {
        if self.size != other.size {
            panic!("Tried to add decks of different sizes")
        }
        self.cut(other.offset);
        self.stride = mod_mul(self.stride, other.stride, self.size);
    }
}

impl std::ops::Mul<u64> for Deck {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        let mut deck = Deck {
            size: self.size,
            stride: 1,
            offset: 0,
        };
        let mut n = rhs;
        while n != 0 {
            let mut count = 1;
            let mut new_deck = self;
            while count < n / 2 {
                new_deck = new_deck + new_deck;
                count *= 2;
            }
            n -= count;
            deck += new_deck;
        }

        deck
    }
}

impl Deck {
    fn from_moves(size: u64, moves: &str) -> Result<Deck> {
        let mut deck = Deck {
            size,
            stride: 1,
            offset: 0,
        };
        for line in moves.lines() {
            if line.starts_with("deal with increment") {
                let n: u64 = line
                    .rsplit(' ')
                    .next()
                    .ok_or(AOCError::new(&format!("Couldn't parse '{}'", line)))?
                    .parse()?;
                deck.increment(n)?;
            } else if line == "deal into new stack" {
                deck.deal_new_stack()?;
            } else if line.starts_with("cut") {
                let mut n: i64 = line
                    .rsplit(' ')
                    .next()
                    .ok_or(AOCError::new(&format!("Couldn't parse '{}'", line)))?
                    .parse()?;
                while n < 0 {
                    n += deck.size as i64;
                }
                deck.cut(n as u64);
            } else {
                return Err(AOCError::new(&format!("Couldn't parse '{}'", line)))?;
            }
        }

        Ok(deck)
    }

    fn cut(&mut self, n: u64) {
        self.offset = mod_add(self.offset, mod_mul(self.stride, n, self.size), self.size);
    }

    fn increment(&mut self, n: u64) -> Result<()> {
        self.stride = mod_mul(
            self.stride,
            mod_inverse(n, self.size).ok_or(AOCError::new(&format!(
                "No inverse found for {} mod {}",
                n, self.size
            )))?,
            self.size,
        );
        Ok(())
    }

    fn deal_new_stack(&mut self) -> Result<()> {
        self.increment(self.size - 1)?;
        self.cut(1);
        Ok(())
    }

    fn iter(&self) -> impl Iterator<Item = u64> {
        let cloned = *self;
        (0..self.size).map(move |n| {
            mod_add(
                cloned.offset,
                mod_mul(cloned.stride, n, cloned.size),
                cloned.size,
            )
        })
    }
}

fn mod_add(mut a: u64, mut b: u64, m: u64) -> u64 {
    a %= m;
    b %= m;
    if b >= m.wrapping_sub(a) {
        b = b.wrapping_sub(m);
    }
    a.wrapping_add(b)
}

fn mod_mul(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut result = 0;

    while a != 0 {
        if a & 1 != 0 {
            result = mod_add(result, b, m);
        }
        a >>= 1;
        b = mod_add(b, b, m);
    }

    result
}

fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    struct U64WithSign {
        value: u64,
        is_negative: bool,
    }

    let mut ab = (a, m);
    let mut xs = (
        U64WithSign {
            value: 0,
            is_negative: false,
        },
        U64WithSign {
            value: 1,
            is_negative: false,
        },
    );

    while ab.0 > 1 {
        if ab.1 == 0 {
            return None;
        }

        let mut new_x0 = U64WithSign {
            value: (ab.0 / ab.1) * xs.0.value,
            is_negative: false,
        };
        ab = (ab.1, ab.0 % ab.1);

        if xs.0.is_negative != xs.1.is_negative {
            new_x0.value += xs.1.value;
            new_x0.is_negative = xs.1.is_negative;
        } else if xs.1.value > new_x0.value {
            new_x0.value = xs.1.value - new_x0.value;
            new_x0.is_negative = xs.1.is_negative;
        } else {
            new_x0.value -= xs.1.value;
            new_x0.is_negative = !xs.0.is_negative;
        }
        xs = (new_x0, xs.0);
    }

    if xs.1.is_negative {
        Some(m.wrapping_sub(xs.1.value))
    } else {
        Some(xs.1.value)
    }
}

fn part1(moves: &str) -> Result<usize> {
    let deck = Deck::from_moves(10007, moves)?;
    for (i, card) in deck.iter().enumerate() {
        if card == 2019 {
            return Ok(i);
        }
    }
    Err(AOCError::new("Card 2019 not found"))?
}

fn part2(moves: &str) -> Result<u64> {
    let deck = Deck::from_moves(119315717514047, moves)?;
    let deck = deck * 101741582076661;

    Ok(deck
        .iter()
        .nth(2020)
        .ok_or(AOCError::new("Card at index 2020 not found"))?)
}

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_add() {
        assert_eq!(mod_add(3, 3, 7), 6);
        assert_eq!(mod_add(3, 4, 7), 0);
        assert_eq!(mod_add(15, 4, 7), 5);
        assert_eq!(mod_add(15, 27, 7), 0);
        assert_eq!(mod_add(u64::max_value() - 5, 10, u64::max_value() - 1), 6);
        assert_eq!(mod_add(10, u64::max_value() - 5, u64::max_value() - 1), 6);
    }

    #[test]
    fn test_mod_mul() {
        assert_eq!(mod_mul(3, 3, 7), 2);
        assert_eq!(mod_mul(15, 17, 7), 3);
        assert_eq!(mod_mul(3, 14, 7), 0);

        let big_zero = ((u64::max_value() - 7) / 7) * 7;
        assert_eq!(mod_mul(big_zero + 2, 10, 7), 6);
        assert_eq!(mod_mul(5, big_zero + 3, 7), 1);
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(1, 7), Some(1));
        assert_eq!(mod_inverse(2, 7), Some(4));
        assert_eq!(mod_inverse(3, 7), Some(5));
        assert_eq!(mod_inverse(4, 7), Some(2));
        assert_eq!(mod_inverse(5, 7), Some(3));
        assert_eq!(mod_inverse(6, 7), Some(6));
        assert_eq!(mod_inverse(7, 10), Some(3));
        assert_eq!(mod_inverse(8, 10), None);

        let big_zero = ((u64::max_value() - 7) / 7) * 7;
        assert_eq!(mod_inverse(big_zero + 4, 7), Some(2));
    }

    #[test]
    fn test_deck_1() {
        let deck = Deck::from_moves(10, "deal with increment 9").unwrap();
        let result: Vec<u64> = deck.iter().collect();
        assert_eq!(result, vec![0, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_deck_2() {
        let deck = Deck::from_moves(
            10,
            "deal with increment 7\n\
             deal into new stack\n\
             deal into new stack",
        )
        .unwrap();
        let result: Vec<u64> = deck.iter().collect();
        assert_eq!(result, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_deck_3() {
        let deck = Deck::from_moves(
            10,
            "cut 6\n\
             deal with increment 7\n\
             deal into new stack",
        )
        .unwrap();
        let result: Vec<u64> = deck.iter().collect();
        assert_eq!(result, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_deck_4() {
        let deck = Deck::from_moves(
            10,
            "deal with increment 7\n\
             deal with increment 9\n\
             cut -2",
        )
        .unwrap();
        let result: Vec<u64> = deck.iter().collect();
        assert_eq!(result, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_deck_5() {
        let deck = Deck::from_moves(
            10,
            "deal into new stack\n\
             cut -2\n\
             deal with increment 7\n\
             cut 8\n\
             cut -4\n\
             deal with increment 7\n\
             cut 3\n\
             deal with increment 9\n\
             deal with increment 3\n\
             cut -1",
        )
        .unwrap();
        let result: Vec<u64> = deck.iter().collect();
        assert_eq!(result, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn test_deck_add() {
        let moves_1 = "deal with increment 7\n\
                       deal into new stack\n\
                       cut -2";
        let moves_2 = "cut 8\n\
                       cut 3\n\
                       deal with increment 9";
        let deck_1 = Deck::from_moves(10, moves_1).unwrap();
        let deck_2 = Deck::from_moves(10, moves_2).unwrap();
        assert_eq!(
            deck_1 + deck_2,
            Deck::from_moves(10, &format!("{}\n{}", moves_1, moves_2)).unwrap()
        );
        assert_eq!(
            deck_2 + deck_1,
            Deck::from_moves(10, &format!("{}\n{}", moves_2, moves_1)).unwrap()
        );
    }

    #[test]
    fn test_deck_mul() {
        let moves = "deal into new stack\n\
                     cut -2\n\
                     deal with increment 7\n\
                     cut 8\n\
                     cut -4\n\
                     deal with increment 7\n\
                     cut 3\n\
                     deal with increment 9\n\
                     deal with increment 3\n\
                     cut -1";

        let deck = Deck::from_moves(10, moves).unwrap();
        for i in 0..20 {
            let all_moves = vec![moves; i].join("\n");
            assert_eq!(deck * (i as u64), Deck::from_moves(10, &all_moves).unwrap());
        }
    }
}
