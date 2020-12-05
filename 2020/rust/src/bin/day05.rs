use aoc::aoc_error::AOCError;
use aoc::utils::get_input;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let seats: Vec<_> = input.lines().map(&str::parse).collect::<Result<_>>()?;

    println!("Part 1: {}", part1(&seats)?);
    println!("Part 2: {}", part2(&seats)?);
    Ok(())
}

fn part1(seats: &[Seat]) -> Result<u64> {
    Ok(seats
        .iter()
        .map(|seat| seat.id())
        .max()
        .ok_or(AOCError::new("No seats"))?)
}

fn part2(seats: &[Seat]) -> Result<u64> {
    let mut seat_ids: Vec<_> = seats.iter().map(|seat| seat.id()).collect();
    seat_ids.sort_unstable();
    Ok(seat_ids
        .windows(2)
        .find(|&pair| pair[1] - pair[0] > 1) // First empty seat is fine by me!
        .map(|pair| pair[0] + 1)
        .ok_or(AOCError::new("Seat not found"))?)
}

struct Seat {
    row: u64,
    column: u64,
}

impl Seat {
    fn id(&self) -> u64 {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 10 || !s.is_ascii() {
            Err(AOCError::new("Invalid seat"))?;
        }
        let row = u64::from_str_radix(&s[..7].replace('F', "0").replace('B', "1"), 2)?;
        let column = u64::from_str_radix(&s[7..].replace('L', "0").replace('R', "1"), 2)?;

        Ok(Seat { row, column })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_seat_1() {
        let seat: Seat = "FBFBBFFRLR".parse().unwrap();
        assert_eq!(seat.row, 44);
        assert_eq!(seat.column, 5);
        assert_eq!(seat.id(), 357);
    }

    #[test]
    fn parse_seat_2() {
        let seat: Seat = "BFFFBBFRRR".parse().unwrap();
        assert_eq!(seat.row, 70);
        assert_eq!(seat.column, 7);
        assert_eq!(seat.id(), 567);
    }

    #[test]
    fn parse_seat_3() {
        let seat: Seat = "FFFBBBFRRR".parse().unwrap();
        assert_eq!(seat.row, 14);
        assert_eq!(seat.column, 7);
        assert_eq!(seat.id(), 119);
    }

    #[test]
    fn parse_seat_4() {
        let seat: Seat = "BBFFBBFRLL".parse().unwrap();
        assert_eq!(seat.row, 102);
        assert_eq!(seat.column, 4);
        assert_eq!(seat.id(), 820);
    }
}
