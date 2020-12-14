use std::str::FromStr;

use aoc::aoc_error::AOCError;
use aoc::math::{egcd, mod_add, mod_mul};
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let mut lines = input.trim().lines();
    let timestamp = lines
        .next()
        .ok_or(AOCError::new("Invalid input"))?
        .parse()?;
    let buses = parse_fields(lines.next().ok_or(AOCError::new("Invalid input"))?, ',')?;

    println!("Part 1: {}", part1(timestamp, &buses)?);
    println!("Part 2: {}", part2(&buses)?);
    Ok(())
}

fn part1(timestamp: u64, buses: &[Bus]) -> Result<u64> {
    let bus_id = next_bus_id(timestamp, buses).ok_or(AOCError::new("No bus found"))?;

    Ok(bus_id * wait_time(bus_id, timestamp))
}

fn part2(buses: &[Bus]) -> Result<u64> {
    departure_pattern_start(buses)
}

fn next_bus_id(timestamp: u64, buses: &[Bus]) -> Option<u64> {
    buses
        .iter()
        .filter_map(|bus| bus.id)
        .min_by_key(|&bus_id| wait_time(bus_id, timestamp))
}

fn departure_pattern_start(buses: &[Bus]) -> Result<u64> {
    // Build an iterator over constraints implied by the bus schedule
    let mut constraints = buses.iter().enumerate().filter_map(|(i, bus)| {
        bus.id.map(|id| Constraint {
            m: id,
            a: wait_time(id, i as u64),
        })
    });
    // Fold the constraints together into a single constraint
    let constraint = constraints.try_fold(Constraint { a: 0, m: 1 }, |a, b| a.combine(b))?;

    Ok(constraint.solutions().next().unwrap())
}

fn wait_time(bus_id: u64, timestamp: u64) -> u64 {
    (bus_id - timestamp % bus_id) % bus_id
}

struct Bus {
    id: Option<u64>,
}

impl FromStr for Bus {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let id = if s == "x" { None } else { Some(s.parse()?) };

        Ok(Bus { id })
    }
}

/// Representation of the constraint x = a % m
#[derive(Debug, Clone, Copy)]
struct Constraint {
    a: u64,
    m: u64,
}

impl Constraint {
    /// Combine two constraints into a single constraint.
    ///
    /// See: https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Case_of_two_moduli
    fn combine(self, other: Constraint) -> Result<Self> {
        let (g, x, y) = egcd(self.m as i64, other.m as i64);
        if g != 1 {
            return Err(AOCError::new("Moduli must be co-prime").into());
        }
        let m = self.m * other.m;
        // Get x, y in 0..m
        let (x, y) = (x.rem_euclid(m as i64) as u64, y.rem_euclid(m as i64) as u64);
        // a = self.a * other.m * y + other.a * self.m * x (mod m)
        // We need to be careful about overflow.
        let a = mod_add(
            mod_mul(mod_mul(self.a, other.m, m), y, m),
            mod_mul(mod_mul(other.a, self.m, m), x, m),
            m,
        );

        Ok(Constraint { a, m })
    }

    /// Solutions to x = a % m
    fn solutions(&self) -> impl Iterator<Item = u64> {
        let (a, m) = (self.a, self.m);
        (0..).map(move |n| a + m * n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_next_bus() {
        let timestamp = 939;
        let buses = parse_fields("7,13,x,x,59,x,31,19", ',').unwrap();

        assert_eq!(next_bus_id(timestamp, &buses), Some(59));
    }

    #[test]
    fn departure_pattern_1() {
        let buses = parse_fields("7,13,x,x,59,x,31,19", ',').unwrap();

        assert_eq!(departure_pattern_start(&buses).unwrap(), 1068781);
    }

    #[test]
    fn departure_pattern_2() {
        let buses = parse_fields("17,x,13,19", ',').unwrap();

        assert_eq!(departure_pattern_start(&buses).unwrap(), 3417);
    }

    #[test]
    fn departure_pattern_3() {
        let buses = parse_fields("67,7,59,61", ',').unwrap();

        assert_eq!(departure_pattern_start(&buses).unwrap(), 754018);
    }

    #[test]
    fn departure_pattern_4() {
        let buses = parse_fields("67,x,7,59,61", ',').unwrap();

        assert_eq!(departure_pattern_start(&buses).unwrap(), 779210);
    }

    #[test]
    fn departure_pattern_5() {
        let buses = parse_fields("67,7,x,59,61", ',').unwrap();

        assert_eq!(departure_pattern_start(&buses).unwrap(), 1261476);
    }

    #[test]
    fn departure_pattern_6() {
        let buses = parse_fields("1789,37,47,1889", ',').unwrap();

        assert_eq!(departure_pattern_start(&buses).unwrap(), 1202161486);
    }
}
