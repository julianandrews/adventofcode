#![feature(iterator_try_collect)]

fn main() -> anyhow::Result<()> {
    let input = aoc_2025::utils::get_input()?;
    let rotations = parsing::parse_input(&input)?;

    println!("{}", part1(&rotations));
    println!("{}", part2(&rotations));

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Rotation(i64);

impl Rotation {
    fn zero_visits(&self, start: i64) -> i64 {
        let mut count = self.0.abs() / 100;
        let end = start + self.0 % 100;
        if end >= 100 || start > 0 && end <= 0 {
            count += 1;
        }
        count
    }
}

fn part1(rotations: &[Rotation]) -> u64 {
    let mut dial = 50;
    let mut count = 0;
    for rotation in rotations {
        dial = (dial + rotation.0).rem_euclid(100);
        if dial == 0 {
            count += 1;
        }
    }
    count
}

fn part2(rotations: &[Rotation]) -> i64 {
    let mut dial = 50;
    let mut count = 0;
    for rotation in rotations {
        count += rotation.zero_visits(dial);
        dial = (dial + rotation.0).rem_euclid(100);
    }
    count
}

mod parsing {
    use super::Rotation;

    impl std::str::FromStr for Rotation {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            let (dir_part, num_part) = (&s[..1], &s[1..]);
            let clicks: i64 = num_part.parse()?;
            match dir_part {
                "L" => Ok(Rotation(-clicks)),
                "R" => Ok(Rotation(clicks)),
                _ => anyhow::bail!("Failed to parse rotation '{}'", s),
            }
        }
    }

    pub fn parse_input(s: &str) -> anyhow::Result<Vec<Rotation>> {
        s.lines().map(|line| line.parse()).try_collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        L68\n\
        L30\n\
        R48\n\
        L5\n\
        R60\n\
        L55\n\
        L1\n\
        L99\n\
        R14\n\
        L82";

    #[test]
    fn parse() {
        let rotations = parsing::parse_input(TEST_DATA).unwrap();
        let expected = vec![
            Rotation(-68),
            Rotation(-30),
            Rotation(48),
            Rotation(-5),
            Rotation(60),
            Rotation(-55),
            Rotation(-1),
            Rotation(-99),
            Rotation(14),
            Rotation(-82),
        ];

        assert_eq!(rotations, expected);
    }

    #[test]
    fn simple_password() {
        let rotations = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(part1(&rotations), 3);
    }

    #[test]
    fn zero_visits() {
        // Provided example
        assert_eq!(Rotation(-68).zero_visits(50), 1); // 50 → -18 (0)
        assert_eq!(Rotation(-30).zero_visits(82), 0); // 82 → 52
        assert_eq!(Rotation(48).zero_visits(52), 1); // 52 → 100 (100)
        assert_eq!(Rotation(-5).zero_visits(0), 0); // 0 → -5
        assert_eq!(Rotation(60).zero_visits(95), 1); // 95 → 155 (100)
        assert_eq!(Rotation(-55).zero_visits(55), 1); // 55 → 0 (0)
        assert_eq!(Rotation(-1).zero_visits(0), 0); // 0 → -1
        assert_eq!(Rotation(-99).zero_visits(99), 1); // -1 → -100 (-100)
        assert_eq!(Rotation(14).zero_visits(0), 0); // 0 → 14
        assert_eq!(Rotation(-82).zero_visits(14), 1); // 14 → -68 (0)

        // One rotation
        assert_eq!(Rotation(100).zero_visits(0), 1); // 0 → 100 (100)
        assert_eq!(Rotation(-100).zero_visits(0), 1); // 50 → -100 (-100)
        assert_eq!(Rotation(100).zero_visits(50), 1); // 50 → 150 (100)
        assert_eq!(Rotation(-100).zero_visits(50), 1); // 50 → -50 (0)

        // Multiple rotations
        assert_eq!(Rotation(1000).zero_visits(50), 10); // 50 → 1050 (100, 200, 300, 400, 500, 600, 700, 800, 900, 1000)
        assert_eq!(Rotation(-1000).zero_visits(50), 10); // 50 → -950 (0, -100, -200, -300, -400, -500, -600, -700, -800, -900)
        assert_eq!(Rotation(250).zero_visits(50), 3); // 50 → 300 (100, 200, 300)
        assert_eq!(Rotation(-250).zero_visits(50), 3); // 50 → -200 (0, -100, -200)

        // Edge of range
        assert_eq!(Rotation(1).zero_visits(99), 1); // 99 → 100 (100)
        assert_eq!(Rotation(-1).zero_visits(1), 1); // 1 → 0 (0)
        assert_eq!(Rotation(-2).zero_visits(1), 1); // 1 → -1 (0)

        // Zero rotation
        assert_eq!(Rotation(0).zero_visits(0), 0); // 0 → 0
        assert_eq!(Rotation(0).zero_visits(50), 0); // 50 → 50

        // Exact multiples ending at boundaries
        assert_eq!(Rotation(200).zero_visits(0), 2); // 0 → 200 (100, 200)
        assert_eq!(Rotation(-200).zero_visits(0), 2); // 0 → -200 (-100, -200)
        assert_eq!(Rotation(200).zero_visits(50), 2); // 50 → 250 (100, 200)
        assert_eq!(Rotation(-200).zero_visits(50), 2); // 50 → -150 (0, -100)
    }

    #[test]
    fn click_password() {
        let rotations = parsing::parse_input(TEST_DATA).unwrap();
        assert_eq!(part2(&rotations), 6);
    }
}
