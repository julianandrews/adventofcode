#![feature(try_blocks)]

use aoc::combinatorics::permute;
use aoc::utils::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let mut map: HappinessMap = input.trim().parse().unwrap();

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&mut map));

    Ok(())
}

fn part1(map: &HappinessMap) -> i64 {
    map.max_happiness()
}

fn part2(map: &mut HappinessMap) -> i64 {
    map.add_indifferent_person("Me");
    map.max_happiness()
}

#[derive(Debug, Clone)]
struct HappinessMap {
    names: Vec<String>,
    map: Vec<Vec<i64>>,
}

impl HappinessMap {
    fn max_happiness(&self) -> i64 {
        let mut best = i64::MIN;
        // Always start with the first person, and permute the remaining people.
        let mut permutations = permute((1..self.names.len()).collect::<Vec<_>>());
        while let Some(indices) = permutations.next_perm() {
            let mut total = 0;
            let mut a = 0;
            for &b in indices {
                total += self.map[a][b] + self.map[b][a];
                a = b;
            }
            total += self.map[a][0] + self.map[0][a];
            best = best.max(total);
        }
        best
    }

    fn add_indifferent_person(&mut self, name: &str) {
        self.names.push(name.to_string());
        for vec in &mut self.map {
            vec.push(0);
        }
        self.map.push(vec![0; self.names.len()]);
    }
}

mod parsing {
    use super::HappinessMap;

    use anyhow::anyhow;
    use rustc_hash::FxHashMap;

    impl std::str::FromStr for HappinessMap {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut map = FxHashMap::default();
            for line in s.lines() {
                let parts: Option<(&str, &str, i64)> = try {
                    let (a, rest) = line.split_once(" would ")?;
                    let (happiness_part, b) = rest
                        .strip_suffix('.')?
                        .split_once(" happiness units by sitting next to ")?;
                    let (sign_part, num_part) = happiness_part.split_once(' ')?;
                    let happiness = match sign_part {
                        "gain" => num_part.parse::<i64>().ok()?,
                        "lose" => -num_part.parse::<i64>().ok()?,
                        _ => None?,
                    };
                    (a, b, happiness)
                };
                let (a, b, happiness) = parts.ok_or_else(|| anyhow!("Invalid line {}.", line))?;
                map.entry(a.to_string())
                    .or_insert_with(FxHashMap::default)
                    .insert(b.to_string(), happiness);
            }
            let (names, map) = aoc::utils::build_index_map(map, || 0);
            Ok(HappinessMap { names, map })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HappinessMap;

    static TEST_DATA: &str = "\
        Alice would gain 54 happiness units by sitting next to Bob.\n\
        Alice would lose 79 happiness units by sitting next to Carol.\n\
        Alice would lose 2 happiness units by sitting next to David.\n\
        Bob would gain 83 happiness units by sitting next to Alice.\n\
        Bob would lose 7 happiness units by sitting next to Carol.\n\
        Bob would lose 63 happiness units by sitting next to David.\n\
        Carol would lose 62 happiness units by sitting next to Alice.\n\
        Carol would gain 60 happiness units by sitting next to Bob.\n\
        Carol would gain 55 happiness units by sitting next to David.\n\
        David would gain 46 happiness units by sitting next to Alice.\n\
        David would lose 7 happiness units by sitting next to Bob.\n\
        David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn max_happiness() {
        let map: HappinessMap = TEST_DATA.parse().unwrap();
        assert_eq!(map.max_happiness(), 330);
    }
}
