use anyhow::Result;
use std::cmp::Ordering;

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let lines: Vec<_> = input.lines().map(DiagramLine::new).collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &[DiagramLine]) -> u64 {
    part_number_sum(lines)
}

fn part2(lines: &[DiagramLine]) -> u64 {
    gear_ratio_sum(lines)
}

fn part_number_sum(lines: &[DiagramLine]) -> u64 {
    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        let mut neighbors: Vec<_> = (y.saturating_sub(1)..y.saturating_add(2).min(lines.len()))
            .map(|y| lines[y].iter_symbols().peekable())
            .collect();
        for range in line.iter_number_ranges() {
            let mut touches = false;
            for line_symbols in &mut neighbors {
                while let Some(&(x, _)) = line_symbols.peek() {
                    if range.touches(x) {
                        touches = true;
                        break;
                    }
                    match x.cmp(&(range.end() + 1)) {
                        Ordering::Less => {
                            line_symbols.next();
                        }
                        Ordering::Equal => {}
                        Ordering::Greater => break,
                    }
                }
            }
            if touches {
                sum += range.contents.parse::<u64>().unwrap();
            }
        }
    }
    sum
}

fn gear_ratio_sum(lines: &[DiagramLine]) -> u64 {
    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        let mut neighbors: Vec<_> = (y.saturating_sub(1)..y.saturating_add(2).min(lines.len()))
            .map(|y| lines[y].iter_number_ranges().peekable())
            .collect();
        for (x, _) in line.iter_symbols().filter(|&(_, s)| s == b'*') {
            let mut gear_numbers: Vec<u64> = vec![];
            for numbers in &mut neighbors {
                while let Some(range) = numbers.next_if(|range| range.start <= x + 1) {
                    if range.touches(x) {
                        gear_numbers.push(range.contents.parse().unwrap());
                    }
                }
            }
            if gear_numbers.len() == 2 {
                sum += gear_numbers[0] * gear_numbers[1];
            }
        }
    }
    sum
}

struct DiagramLine<'a> {
    line: &'a str,
}

impl<'a> DiagramLine<'a> {
    fn new(line: &'a str) -> DiagramLine<'a> {
        DiagramLine { line }
    }

    fn iter_number_ranges(&self) -> NumberRangeIterator<'a> {
        NumberRangeIterator {
            line: self.line,
            x: 0,
        }
    }

    fn iter_symbols(&self) -> impl Iterator<Item = (usize, u8)> + 'a {
        self.line
            .bytes()
            .enumerate()
            .filter(|(_, b)| *b != b'.' && !b.is_ascii_digit())
    }
}

struct NumberRangeIterator<'a> {
    line: &'a str,
    x: usize,
}

impl<'a> Iterator for NumberRangeIterator<'a> {
    type Item = NumberRange<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.line.as_bytes();
        while !bytes.get(self.x)?.is_ascii_digit() {
            self.x += 1;
        }
        let start = self.x;
        while bytes.get(self.x).is_some_and(|b| b.is_ascii_digit()) {
            self.x += 1;
        }
        Some(NumberRange {
            start,
            contents: &self.line[start..self.x],
        })
    }
}

struct NumberRange<'a> {
    start: usize,
    contents: &'a str,
}

impl<'a> NumberRange<'a> {
    fn touches(&self, x: usize) -> bool {
        (self.start.saturating_sub(1)..(self.end() + 2)).contains(&x)
    }

    fn end(&self) -> usize {
        self.start + self.contents.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";

    #[test]
    fn part_numbers() {
        let lines: Vec<_> = TEST_DATA.lines().map(DiagramLine::new).collect();
        assert_eq!(part_number_sum(&lines), 4361);
    }

    #[test]
    fn on_edge() {
        let lines: Vec<_> = "*7".lines().map(DiagramLine::new).collect();
        assert_eq!(part_number_sum(&lines), 7);
    }

    #[test]
    fn in_sequence() {
        let lines: Vec<_> = "1*3".lines().map(DiagramLine::new).collect();
        assert_eq!(part_number_sum(&lines), 4);
    }

    #[test]
    fn gear_ratios() {
        let lines: Vec<_> = TEST_DATA.lines().map(DiagramLine::new).collect();
        assert_eq!(gear_ratio_sum(&lines), 467835);
    }
}
