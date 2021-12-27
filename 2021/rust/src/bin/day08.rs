use std::convert::TryInto;

use aoc::aoc_error::AOCError;
use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let entries: Vec<_> = input
        .trim()
        .lines()
        .map(parse_line)
        .collect::<Result<_>>()?;

    println!("Part 1: {}", part1(&entries)?);
    println!("Part 2: {}", part2(&entries)?);
    Ok(())
}

fn parse_line(line: &str) -> Result<([Signal; 10], [Signal; 4])> {
    let (inputs_part, outputs_part) = line
        .split_once(" | ")
        .ok_or(AOCError::new("Failed to parse line"))?;
    let inputs: [Signal; 10] = parse_fields(inputs_part, ' ')?.as_slice().try_into()?;
    let outputs: [Signal; 4] = parse_fields(outputs_part, ' ')?.as_slice().try_into()?;
    Ok((inputs, outputs))
}

fn part1(entries: &[([Signal; 10], [Signal; 4])]) -> Result<usize> {
    let decoded_digits: Vec<_> = entries
        .iter()
        .map(|(inputs, outputs)| decode(inputs, outputs))
        .collect::<Result<_>>()?;
    Ok(decoded_digits
        .into_iter()
        .flatten()
        .filter(|&digit| digit == 1 || digit == 4 || digit == 7 || digit == 8)
        .count())
}

fn part2(entries: &[([Signal; 10], [Signal; 4])]) -> Result<u64> {
    let decoded_digits: Vec<_> = entries
        .iter()
        .map(|(inputs, outputs)| decode(inputs, outputs))
        .collect::<Result<_>>()?;
    Ok(decoded_digits.into_iter().map(digits_value).sum())
}

fn decode(inputs: &[Signal; 10], outputs: &[Signal; 4]) -> Result<[u64; 4]> {
    let mut digits: [Option<Signal>; 10] = [None; 10];
    // First identify 1, 4, 7, and 8 based on number of segments
    for &input in inputs {
        match input.segments() {
            2 => digits[1] = Some(input),
            3 => digits[7] = Some(input),
            4 => digits[4] = Some(input),
            7 => digits[8] = Some(input),
            _ => {}
        }
    }

    // Identify the rest based on number of segments and size of overlap with known digits.
    for &input in inputs {
        let segments = input.segments();
        if segments == 5 {
            if input.overlap_count(&digits[4].ok_or(AOCError::new("Digit missing"))?) == 2 {
                digits[2] = Some(input);
            } else if input.overlap_count(&digits[1].ok_or(AOCError::new("Digit missing"))?) == 2 {
                digits[3] = Some(input);
            } else {
                digits[5] = Some(input);
            }
        }
        if segments == 6 {
            if input.overlap_count(&digits[1].ok_or(AOCError::new("Digit missing"))?) == 1 {
                digits[6] = Some(input);
            } else if input.overlap_count(&digits[4].ok_or(AOCError::new("Digit missing"))?) == 4 {
                digits[9] = Some(input);
            } else {
                digits[0] = Some(input);
            }
        }
    }
    if digits.iter().any(|x| x.is_none()) {
        return Err(Box::new(AOCError::new("Failed to match all digits")));
    }
    let mut result = [0; 4];
    for i in 0..4 {
        for (x, &digit) in digits.iter().enumerate() {
            if Some(outputs[i]) == digit {
                result[i] = x as u64;
            }
        }
    }
    Ok(result)
}

fn digits_value(digits: [u64; 4]) -> u64 {
    digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3] * 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Signal {
    inputs: u8,
}

impl Signal {
    fn overlap_count(&self, other: &Self) -> u32 {
        (self.inputs & other.inputs).count_ones()
    }

    fn segments(&self) -> u32 {
        self.inputs.count_ones()
    }
}

impl std::str::FromStr for Signal {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut inputs = 0;
        for b in s.bytes() {
            let i = match b {
                b'a'..=b'g' => (b - b'a') as usize,
                _ => return Err(Box::new(AOCError::new("Unrecognized input"))),
            };
            inputs |= 1 << i;
        }
        Ok(Signal { inputs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: [&str; 10] = [
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];
    static TEST_RESULTS: [u64; 10] = [8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];

    #[test]
    fn test_part1() {
        let entries: Vec<_> = TEST_DATA
            .iter()
            .map(|s| parse_line(s))
            .collect::<Result<_>>()
            .unwrap();
        let result = part1(&entries).unwrap();
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part2_example() {
        let data =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let entries = [parse_line(data).unwrap()];
        let result = part2(&entries).unwrap();
        assert_eq!(result, 5353);
    }

    macro_rules! entry_tests {
        ($($name:ident: $index:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (inputs, outputs) = parse_line(TEST_DATA[$index]).unwrap();
                let result = digits_value(decode(&inputs, &outputs).unwrap());
                assert_eq!(result, TEST_RESULTS[$index]);
            }
        )*
        }
    }

    entry_tests! {
        decode_0: 0,
        decode_1: 1,
        decode_2: 2,
        decode_3: 3,
        decode_4: 4,
        decode_5: 5,
        decode_6: 6,
        decode_7: 7,
        decode_8: 8,
        decode_9: 9,
    }

    #[test]
    fn test_part_2_full() {
        let entries: Vec<_> = TEST_DATA
            .iter()
            .map(|s| parse_line(s))
            .collect::<Result<_>>()
            .unwrap();
        let result = part2(&entries).unwrap();
        assert_eq!(result, 61229);
    }
}
