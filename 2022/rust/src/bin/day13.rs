use std::cmp::Ordering;

use anyhow::{bail, Result};

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let packets = parse_packets(&input)?;

    println!("Part 1: {}", part1(&packets)?);
    println!("Part 2: {}", part2(&packets));

    Ok(())
}

fn part1(packets: &[Packet]) -> Result<usize> {
    if packets.len() % 2 != 0 {
        bail!("Can't compare odd number of packets");
    }
    Ok(packets
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, s)| s[0] < s[1])
        .map(|(i, _)| i + 1)
        .sum())
}

fn part2(packets: &[Packet]) -> usize {
    let left_div: Packet = "[[2]]".parse().expect("Parse failed");
    let left = packets.iter().filter(|&p| p < &left_div).count() + 1;
    let right_div = "[[6]]".parse().expect("Parse failed");
    let right = packets.iter().filter(|&p| p < &right_div).count() + 2;
    left * right
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
            (Self::Integer(_), Self::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Self::List(_), Self::Integer(_)) => self.cmp(&Self::List(vec![other.clone()])),
            (Self::List(l), Self::List(m)) => {
                for (a, b) in l.iter().zip(m) {
                    if a.cmp(b) != Ordering::Equal {
                        return a.cmp(b);
                    }
                }
                l.len().cmp(&m.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::str::FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (packet, rem) = parse_packet(s)?;
        if !rem.is_empty() {
            bail!("Failed to parse whole input");
        }
        Ok(packet)
    }
}

fn parse_packet(s: &str) -> Result<(Packet, &str)> {
    if let Some(mut rem) = s.strip_prefix('[') {
        let mut packets = vec![];
        loop {
            match rem.chars().next() {
                Some(',') => rem = &rem[1..],
                Some(']') => return Ok((Packet::List(packets), &rem[1..])),
                _ => {
                    let value;
                    (value, rem) = parse_packet(rem)?;
                    packets.push(value);
                }
            }
        }
    } else {
        let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
        Ok((Packet::Integer(digits.parse()?), &s[digits.len()..]))
    }
}

fn parse_packets(s: &str) -> Result<Vec<Packet>> {
    s.trim()
        .split('\n')
        .filter(|&line| !line.is_empty())
        .map(|line| line.parse())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn ordering() {
        let packets = parse_packets(TEST_DATA).unwrap();
        let results: Vec<_> = packets.chunks(2).map(|s| s[0].cmp(&s[1])).collect();
        let expected = vec![
            Ordering::Less,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Less,
            Ordering::Greater,
            Ordering::Greater,
        ];
        assert_eq!(results, expected);
    }

    #[test]
    fn tricky_packets() {
        let a: Packet = "[[3,[1,8],0]]".parse().unwrap();
        let b: Packet = "[[[3],[[6,9,9]],4,[[],8],9],[[[1,0,7],[2,1,4],[0,9,4,10,2]],[[]],1,1],[[8,10,7],[6],5],[4]]".parse().unwrap();
        assert!(a < b)
    }

    #[test]
    fn count_indices() {
        let packets = parse_packets(TEST_DATA).unwrap();
        assert_eq!(part1(&packets).unwrap(), 13);
    }

    #[test]
    fn decoder_key() {
        let packets = parse_packets(TEST_DATA).unwrap();
        assert_eq!(part2(&packets), 140);
    }
}
