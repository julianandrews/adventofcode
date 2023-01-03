use anyhow::{anyhow, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let numbers: Vec<i64> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&numbers)?);
    println!("Part 2: {}", part2(&numbers)?);

    Ok(())
}

fn part1(numbers: &[i64]) -> Result<i64> {
    let mixed = mix(numbers, 1);
    let (x, y, z) = grove_coordinates(&mixed).ok_or_else(|| anyhow!("No coordinates found"))?;
    Ok(x + y + z)
}

fn part2(numbers: &[i64]) -> Result<i64> {
    let decrypted = decrypt(numbers);
    let mixed = mix(&decrypted, 10);
    let (x, y, z) = grove_coordinates(&mixed).ok_or_else(|| anyhow!("No coordinates found"))?;
    Ok(x + y + z)
}

fn mix(numbers: &[i64], n: usize) -> Vec<i64> {
    let mut indices: Vec<usize> = (0..numbers.len()).collect();
    for _ in 0..n {
        for i in 0..numbers.len() {
            let j = indices.iter().position(|&x| x == i).unwrap();
            indices.remove(j);
            let new_index = (j as i64 + numbers[i]).rem_euclid(numbers.len() as i64 - 1) as usize;
            indices.insert(new_index, i);
        }
    }
    indices.iter().map(|&i| numbers[i]).collect()
}

fn grove_coordinates(numbers: &[i64]) -> Option<(i64, i64, i64)> {
    let zero_ix = numbers.iter().position(|&x| x == 0)?;
    Some((
        numbers[(zero_ix + 1000) % numbers.len()],
        numbers[(zero_ix + 2000) % numbers.len()],
        numbers[(zero_ix + 3000) % numbers.len()],
    ))
}

fn decrypt(numbers: &[i64]) -> Vec<i64> {
    static DECRYPTION_KEY: i64 = 811589153;
    numbers.iter().map(|x| x * DECRYPTION_KEY).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &[i64] = &[1, 2, -3, 3, -2, 0, 4];

    #[test]
    fn simple_mix() {
        let mixed = mix(TEST_DATA, 1);
        assert_eq!(grove_coordinates(&mixed), Some((4, -3, 2)));
    }

    #[test]
    fn decrypted_10x_mix() {
        let mixed = mix(&decrypt(TEST_DATA), 10);
        assert_eq!(
            grove_coordinates(&mixed),
            Some((811589153, 2434767459, -1623178306))
        );
    }
}
