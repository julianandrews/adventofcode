use std::cmp::Ordering;

use anyhow::{anyhow, bail, Result};

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
        if !(s.starts_with('[') && s.ends_with(']')) {
            bail!("Invalid root packet");
        }
        let mut packet = Packet::List(vec![]);
        let ptr: *mut Vec<_> = match &mut packet {
            Packet::List(v) => &mut *v,
            _ => unreachable!(),
        };
        let mut ptrs: Vec<*mut Vec<Packet>> = vec![ptr];
        let text = &s[1..s.len() - 1];
        let lexer = Lexer { text, cursor: 0 };
        for token in lexer {
            let v = match ptrs.iter().last() {
                Some(ptr) => unsafe { ptr.as_mut() }.expect("Null pointer!"),
                None => bail!("Invalid packet: too many closing brackets"),
            };
            match token? {
                Token::ListStart => {
                    v.push(Packet::List(vec![]));
                    let i = v.len() - 1;
                    let ptr: *mut Vec<_> = match &mut v[i] {
                        Packet::List(v) => &mut *v,
                        _ => unreachable!(),
                    };
                    ptrs.push(ptr);
                }
                Token::ListEnd => {
                    ptrs.pop();
                }
                Token::Integer(n) => v.push(Packet::Integer(n)),
            }
        }
        Ok(packet)
    }
}

struct Lexer<'a> {
    text: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    fn peek_char(&self) -> Option<char> {
        self.text[self.cursor..].chars().next()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peek_char()? {
            '[' => {
                self.cursor += 1;
                Some(Ok(Token::ListStart))
            }
            ']' => {
                self.cursor += 1;
                if self.peek_char() == Some(',') {
                    self.cursor += 1;
                }
                Some(Ok(Token::ListEnd))
            }
            '0'..='9' => {
                let end = self.text[self.cursor..]
                    .chars()
                    .position(|c| !c.is_ascii_digit())
                    .unwrap_or(self.text.len() - self.cursor)
                    + self.cursor;
                let n = self.text[self.cursor..end]
                    .parse()
                    .expect("Failed to parse digits as number");
                self.cursor = end;
                if self.peek_char() == Some(',') {
                    self.cursor += 1;
                }
                Some(Ok(Token::Integer(n)))
            }
            _ => Some(Err(anyhow!("Unexpected character"))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    ListStart,
    ListEnd,
    Integer(u32),
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
    fn lexer() {
        let text = "[[3,[1,8],0]]";
        let lexer = Lexer { text, cursor: 0 };
        let tokens: Vec<_> = lexer.collect::<Result<_>>().unwrap();
        let expected = vec![
            Token::ListStart,
            Token::ListStart,
            Token::Integer(3),
            Token::ListStart,
            Token::Integer(1),
            Token::Integer(8),
            Token::ListEnd,
            Token::Integer(0),
            Token::ListEnd,
            Token::ListEnd,
        ];
        assert_eq!(tokens, expected);
    }

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
