extern crate aoc;

use aoc::aoc_error::AOCError;
use aoc::nums::digits;
use std::convert::TryFrom;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn fft(input_list: &[u8], num_phases: usize) -> Vec<u8> {
    static BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];
    let pattern_iterator = |n: usize| {
        (0..4)
            .cycle()
            .flat_map(move |i| std::iter::repeat(BASE_PATTERN[i]).take(n + 1))
            .skip(1)
    };
    let mut input = input_list.to_vec();
    let mut output = vec![0; input.len()];
    for _phase in 0..num_phases {
        for (n, o) in output.iter_mut().enumerate().take(input.len()) {
            let total = input
                .iter()
                .zip(pattern_iterator(n))
                .fold(0, |total, (a, b)| total + *a as i64 * b);
            *o = (total.abs() % 10) as u8;
        }
        std::mem::swap(&mut input, &mut output);
    }

    input
}

fn fft_last_half_digits(mut last_half: Vec<u8>, num_phases: usize) -> Vec<u8> {
    for _ in 0..num_phases {
        for i in (0..last_half.len() - 1).rev() {
            last_half[i] = (last_half[i + 1] + last_half[i]) % 10;
        }
    }

    last_half
}

fn parse_digits(s: &str) -> Result<Vec<u8>> {
    Ok(digits(s)?
        .into_iter()
        .map(u8::try_from)
        .collect::<std::result::Result<Vec<_>, _>>()?)
}

fn part1(input_list: &[u8]) -> String {
    fft(input_list, 100)[..8]
        .iter()
        .map(|d| std::char::from_digit(*d as u32, 10).unwrap())
        .collect()
}

fn part2(input_list: &[u8]) -> Result<String> {
    let new_input_list_length = input_list.len() * 10_000;
    let message_offset: usize = input_list
        .iter()
        .take(7)
        .map(|d| d.to_string())
        .collect::<String>()
        .parse()?;
    if message_offset <= new_input_list_length / 2 {
        Err(AOCError::new("Message offset too close to start of list"))?;
    }
    let trailing_digits = input_list
        .iter()
        .cycle()
        .skip(message_offset % input_list.len())
        .take(new_input_list_length - message_offset)
        .cloned()
        .collect::<Vec<u8>>();
    let digits = fft_last_half_digits(trailing_digits, 100);

    Ok(digits.iter().take(8).map(u8::to_string).collect())
}

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let input_list = parse_digits(input.trim())?;

    println!("Part 1: {}", part1(&input_list));
    println!("Part 2: {}", part2(&input_list)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{fft, fft_last_half_digits, parse_digits};

    #[test]
    fn test_fft_simple() {
        let input_list = parse_digits("12345678").unwrap();
        let output = fft(&input_list, 4);
        assert_eq!(output, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_fft_case_1() {
        let input_list = parse_digits("80871224585914546619083218645595").unwrap();
        let output = fft(&input_list, 100);
        assert_eq!(output[..8], vec![2, 4, 1, 7, 6, 1, 7, 6][..8]);
    }

    #[test]
    fn test_fft_case_2() {
        let input_list = parse_digits("19617804207202209144916044189917").unwrap();
        let output = fft(&input_list, 100);
        assert_eq!(output[..8], vec![7, 3, 7, 4, 5, 4, 1, 8][..8]);
    }

    #[test]
    fn test_fft_case_3() {
        let input_list = parse_digits("69317163492948606335995924319873").unwrap();
        let output = fft(&input_list, 100);
        assert_eq!(output[..8], vec![5, 2, 4, 3, 2, 1, 3, 3][..8]);
    }

    #[test]
    fn test_fft_last_half_digits_case_1() {
        let input_list = parse_digits("80871224585914546619083218645595").unwrap();
        let fft_output = fft(&input_list, 100);
        let fft_last_half_digits_output =
            fft_last_half_digits(input_list[input_list.len() - 10..].to_vec(), 100);
        assert_eq!(
            fft_output[fft_output.len() - 10..],
            fft_last_half_digits_output[..]
        );
    }

    #[test]
    fn test_fft_last_half_digits_case_2() {
        let input_list = parse_digits("19617804207202209144916044189917").unwrap();
        let fft_output = fft(&input_list, 100);
        let fft_last_half_digits_output =
            fft_last_half_digits(input_list[input_list.len() - 10..].to_vec(), 100);
        assert_eq!(
            fft_output[fft_output.len() - 10..],
            fft_last_half_digits_output[..]
        );
    }

    #[test]
    fn test_fft_last_half_digits_case_3() {
        let input_list = parse_digits("69317163492948606335995924319873").unwrap();
        let fft_output = fft(&input_list, 100);
        let fft_last_half_digits_output =
            fft_last_half_digits(input_list[input_list.len() - 10..].to_vec(), 100);
        assert_eq!(
            fft_output[fft_output.len() - 10..],
            fft_last_half_digits_output[..]
        );
    }
}
