#![feature(try_blocks)]

use aoc::utils::{get_input, parse_fields};

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let reindeer: Vec<Reindeer> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&reindeer));
    println!("Part 2: {}", part2(&reindeer));

    Ok(())
}

fn part1(reindeer: &[Reindeer]) -> u64 {
    reindeer.iter().map(|r| r.position(2503)).max().unwrap_or(0)
}

fn part2(reindeer: &[Reindeer]) -> u64 {
    points(reindeer, 2503).into_iter().max().unwrap_or(0)
}

fn points(reindeer: &[Reindeer], time: u64) -> Vec<u64> {
    let mut points = vec![0; reindeer.len()];
    for t in 1..time + 1 {
        let distances: Vec<_> = reindeer.iter().map(|r| r.position(t)).collect();
        let best = *distances.iter().max().unwrap_or(&0);
        for (score, distance) in points.iter_mut().zip(distances) {
            if distance == best {
                *score += 1;
            }
        }
    }
    points
}

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    speed: u64,
    fly_time: u64,
    rest_time: u64,
}

impl Reindeer {
    fn position(&self, time: u64) -> u64 {
        let cycle_time = self.fly_time + self.rest_time;
        let (cycles, remainder) = (time / cycle_time, time % cycle_time);
        self.speed * (self.fly_time * cycles + remainder.min(self.fly_time))
    }
}

mod parsing {
    use super::Reindeer;

    use anyhow::anyhow;

    impl std::str::FromStr for Reindeer {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Option<_> = try {
                let (_name, rest) = s.split_once(" can fly ")?;
                let (speed_part, rest) = rest.split_once(" km/s for ")?;
                let (fly_time_part, rest) = rest.split_once(" seconds, but then must rest for ")?;
                let rest_time_part = rest.strip_suffix(" seconds.")?;
                (speed_part, fly_time_part, rest_time_part)
            };
            let (speed_part, fly_time_part, rest_time_part) =
                parts.ok_or_else(|| anyhow!("Invalid reindeer {}.", s))?;
            Ok(Reindeer {
                speed: speed_part.parse()?,
                fly_time: fly_time_part.parse()?,
                rest_time: rest_time_part.parse()?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{points, Reindeer};

    static COMET: &str =
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
    static DANCER: &str =
        "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn position_1() {
        let comet: Reindeer = COMET.parse().unwrap();
        assert_eq!(comet.position(1000), 1120);
    }

    #[test]
    fn position_2() {
        let dancer: Reindeer = DANCER.parse().unwrap();
        assert_eq!(dancer.position(1000), 1056);
    }

    #[test]
    fn race() {
        let reindeer = vec![COMET.parse().unwrap(), DANCER.parse().unwrap()];
        assert_eq!(points(&reindeer, 1000), vec![312, 689]);
    }
}
