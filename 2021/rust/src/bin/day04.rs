use std::collections::HashSet;

use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let numbers: Vec<u64> =
        parse_fields(input.lines().next().ok_or(AOCError::new("No input"))?, ',')?;
    let boards: Vec<BingoBoard> = input
        .trim()
        .split("\n\n")
        .skip(1)
        .map(|block| block.parse())
        .collect::<Result<_>>()?;

    println!("Part 1: {}", part1(&boards, &numbers)?);
    println!("Part 2: {}", part2(&boards, &numbers)?);
    Ok(())
}

fn part1(boards: &[BingoBoard], numbers: &[u64]) -> Result<u64> {
    let mut marked_boards: Vec<MarkedBoard> =
        boards.iter().map(|board| MarkedBoard::new(board)).collect();
    for &number in numbers {
        for marks in &mut marked_boards {
            marks.mark(number);
        }
        let winners: Vec<_> = marked_boards
            .iter()
            .filter(|marks| marks.is_winner())
            .collect();
        match winners[..] {
            [winner] => return Ok(winner.unmarked_total() * number),
            [] => {}
            _ => return Err(Box::new(AOCError::new("Multiple winners found."))),
        }
    }
    Err(Box::new(AOCError::new("No winners found.")))
}

fn part2(boards: &[BingoBoard], numbers: &[u64]) -> Result<u64> {
    let mut marked_boards: Vec<MarkedBoard> =
        boards.iter().map(|board| MarkedBoard::new(board)).collect();
    let mut scores = vec![];
    for &number in numbers {
        for marks in &mut marked_boards {
            marks.mark(number);
        }
        let (winners, new_marked_boards) = marked_boards
            .into_iter()
            .partition(|marks| marks.is_winner());
        marked_boards = new_marked_boards;
        for winner in winners {
            scores.push(winner.unmarked_total() * number);
        }
    }
    scores
        .into_iter()
        .last()
        .ok_or(Box::new(AOCError::new("No winners found.")))
}

#[derive(Debug)]
struct BingoBoard {
    numbers: Vec<Vec<u64>>,
}

impl std::str::FromStr for BingoBoard {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let numbers: Vec<Vec<u64>> = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(u64::from_str)
                    .collect::<std::result::Result<Vec<u64>, _>>()
            })
            .collect::<std::result::Result<_, _>>()
            .map_err(|e| AOCError::new(&format!("Parsing failure: {}", e)))?;
        let height = numbers.len();
        if numbers.iter().any(|row| row.len() != height) {
            return Err(Box::new(AOCError::new("Found non-square bingo board!")));
        }

        Ok(BingoBoard { numbers })
    }
}

#[derive(Debug)]
struct MarkedBoard<'a> {
    board: &'a BingoBoard,
    marks: HashSet<u64>,
}

impl<'a> MarkedBoard<'a> {
    fn new(board: &'a BingoBoard) -> MarkedBoard<'a> {
        MarkedBoard {
            board,
            marks: HashSet::new(),
        }
    }

    fn mark(&mut self, number: u64) {
        self.marks.insert(number);
    }

    fn is_winner(&self) -> bool {
        let size = self.board.numbers.len();
        (0..size).any(|i| {
            self.board
                .numbers
                .iter()
                .all(|row| self.marks.contains(&row[i]))
                || self.board.numbers[i].iter().all(|n| self.marks.contains(n))
        })
    }

    fn unmarked_total(&self) -> u64 {
        self.board
            .numbers
            .iter()
            .flatten()
            .filter(|n| !self.marks.contains(n))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_NUMBERS: [u64; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    static TEST_BOARDS: [&str; 3] = [
        "22 13 17 11  0\
       \n 8  2 23  4 24\
       \n21  9 14 16  7\
       \n 6 10  3 18  5\
       \n 1 12 20 15 19",
        " 3 15  0  2 22\
       \n 9 18 13 17  5\
       \n19  8  7 25 23\
       \n20 11 10 24  4\
       \n14 21 16 12  6",
        "14 21 17 24  4\
       \n10 16 15  9 19\
       \n18  8 23 26 20\
       \n22 11 13  6  5\
       \n 2  0 12  3  7",
    ];

    #[test]
    fn test_part1() {
        let boards: Vec<BingoBoard> = TEST_BOARDS.iter().map(|s| s.parse().unwrap()).collect();
        let result = part1(&boards, &TEST_NUMBERS).unwrap();
        assert_eq!(result, 4512);
    }

    #[test]
    fn test_part2() {
        let boards: Vec<BingoBoard> = TEST_BOARDS.iter().map(|s| s.parse().unwrap()).collect();
        let result = part2(&boards, &TEST_NUMBERS).unwrap();
        assert_eq!(result, 1924);
    }
}
