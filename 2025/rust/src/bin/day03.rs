fn main() -> anyhow::Result<()> {
    let input = aoc_2025::utils::get_input()?;
    let power_banks = parsing::parse_input(input.trim())?;

    println!("{}", part1(&power_banks));
    println!("{}", part2(&power_banks));

    Ok(())
}

fn part1(power_banks: &[PowerBank]) -> u64 {
    power_banks.iter().map(|p| p.max_joltage(2)).sum()
}

fn part2(power_banks: &[PowerBank]) -> u64 {
    power_banks.iter().map(|p| p.max_joltage(12)).sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PowerBank {
    batteries: Vec<u8>,
}

impl PowerBank {
    fn max_joltage(&self, battery_count: usize) -> u64 {
        let mut stack = Vec::with_capacity(battery_count);

        for (i, &digit) in self.batteries.iter().enumerate() {
            // When we find a stronger battery, remove the last one as long as we have enough left.
            let stack_size_floor = (battery_count + i).saturating_sub(self.batteries.len());
            while stack.len() > stack_size_floor && digit > *stack.last().unwrap() {
                stack.pop();
            }
            if stack.len() < battery_count {
                stack.push(digit);
            }
        }

        stack
            .iter()
            .fold(0u64, |acc, &digit| acc * 10 + digit as u64)
    }
}

mod parsing {
    use super::PowerBank;

    impl std::str::FromStr for PowerBank {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let batteries = s
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|n| n as u8)
                        .ok_or_else(|| anyhow::anyhow!("Failed to parse {} in bank {}", c, s))
                })
                .collect::<anyhow::Result<_>>()?;

            Ok(PowerBank { batteries })
        }
    }

    pub fn parse_input(s: &str) -> anyhow::Result<Vec<PowerBank>> {
        aoc_2025::utils::parse_fields(s.trim(), '\n')
    }
}

#[cfg(test)]
mod tests {
    use crate::{parsing, PowerBank};

    static TEST_DATA: &str = "\n\
        987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111";

    #[test]
    fn parsing() {
        let result = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(
            result,
            vec![
                PowerBank {
                    batteries: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
                },
                PowerBank {
                    batteries: vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
                },
                PowerBank {
                    batteries: vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]
                },
                PowerBank {
                    batteries: vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
                },
            ]
        );
    }

    #[test]
    fn max_joltage_2() {
        let batteries = parsing::parse_input(TEST_DATA).unwrap();
        let result: Vec<_> = batteries.iter().map(|b| b.max_joltage(2)).collect();
        assert_eq!(result, vec![98, 89, 78, 92]);
    }

    #[test]
    fn max_joltage_12() {
        let batteries = parsing::parse_input(TEST_DATA).unwrap();
        let result: Vec<_> = batteries.iter().map(|b| b.max_joltage(12)).collect();
        assert_eq!(
            result,
            vec![987654321111, 811111111119, 434234234278, 888911112111]
        );
    }
}
