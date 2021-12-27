use aoc::utils::{get_input, parse_fields};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let timers: Vec<u64> = parse_fields(input.trim(), ',')?;

    println!("Part 1: {}", part1(&timers));
    println!("Part 2: {}", part2(&timers));
    Ok(())
}

fn part1(timers: &[u64]) -> u64 {
    propagate_lanternfish(timers, 80)
}

fn part2(timers: &[u64]) -> u64 {
    propagate_lanternfish(timers, 256)
}

fn propagate_lanternfish(timers: &[u64], days: u64) -> u64 {
    let mut spawning_groups = [0; 7];
    let mut fry = 0;
    let mut juveniles = 0;
    for n in timers {
        spawning_groups[*n as usize] += 1;
    }
    for day in 0..days as usize {
        // Figure out how many new fish we'll have
        let new_fish = spawning_groups[day % 7];
        // Add the juveniles to the spawning group that just had children
        spawning_groups[day % 7] += juveniles;
        // Update the juvenile count to the old fry count
        juveniles = fry;
        // Update the fry count with the newly spawned fish
        fry = new_fish;
    }
    spawning_groups.iter().sum::<u64>() + juveniles + fry
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: [u64; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn test_case_1() {
        let result = propagate_lanternfish(&TEST_DATA, 18);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_case_2() {
        let result = propagate_lanternfish(&TEST_DATA, 80);
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_case_3() {
        let result = propagate_lanternfish(&TEST_DATA, 256);
        assert_eq!(result, 26984457539);
    }
}
