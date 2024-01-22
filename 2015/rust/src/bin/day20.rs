use aoc::utils::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let num_presents: usize = input.trim().parse()?;

    println!("Part 1: {}", part1(num_presents));
    println!("Part 2: {}", part2(num_presents));

    Ok(())
}

fn part1(n: usize) -> usize {
    first_above(n, 10, None)
}

fn part2(n: usize) -> usize {
    first_above(n, 11, Some(50))
}

fn first_above(total: usize, presents_per_elf: usize, max_deliveries: Option<usize>) -> usize {
    // To avoid wasted calculations only go up to `last_house`. If that fails, try double!
    // Runtime is mostly determined by the final loop, so just start at 1 for elegance.
    let mut last_house = 1;
    loop {
        let mut visits = vec![0; last_house + 1];
        for elf in 1..=last_house {
            let last_delivery = match max_deliveries {
                Some(max) => last_house.min(max * elf),
                None => last_house,
            };
            for value in (elf..last_delivery + 1).step_by(elf) {
                visits[value] += elf
            }
        }
        for (i, &visit_count) in visits.iter().enumerate() {
            if visit_count * presents_per_elf > total {
                return i;
            }
        }
        last_house *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::first_above;

    #[test]
    fn first_above_1() {
        assert_eq!(first_above(110, 10, None), 6);
    }

    #[test]
    fn first_above_2() {
        assert_eq!(first_above(140, 10, None), 8);
    }
}
