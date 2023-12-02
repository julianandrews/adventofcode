use bitvec::vec::BitVec;

use aoc::utils::get_input;

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = get_input()?;
    let map: CucumberMap = input.parse()?;

    println!("Part 1: {}", part1(map));

    Ok(())
}

fn part1(mut map: CucumberMap) -> u64 {
    map.advance_until_stopped()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CucumberMap {
    eastbound: Vec<BitVec>,
    southbound: Vec<BitVec>,
}

impl CucumberMap {
    pub fn advance_until_stopped(&mut self) -> u64 {
        let mut i = 1;
        while self.step() {
            i += 1;
        }
        i
    }

    pub fn step(&mut self) -> bool {
        let mut changed = false;

        // move east
        let occupied = self.occupied();
        for (y, row) in self.eastbound.clone().into_iter().enumerate() {
            let mut blocked = occupied[y].clone();
            blocked.rotate_left(1);
            let moved = row & !blocked;
            changed |= moved.any();
            let mut new = moved.clone();
            new.rotate_right(1);
            self.eastbound[y] |= new;
            self.eastbound[y] &= !moved;
        }

        // move south
        let occupied = self.occupied();
        for (y, row) in self.southbound.clone().into_iter().enumerate() {
            let moved = row & !occupied[(y + 1) % occupied.len()].clone();
            changed |= moved.any();
            self.southbound[(y + 1) % occupied.len()] |= moved.clone();
            self.southbound[y] &= !moved;
        }

        changed
    }

    fn occupied(&self) -> Vec<BitVec> {
        self.eastbound
            .iter()
            .zip(self.southbound.iter())
            .map(|(a, b)| a.clone() | b)
            .collect()
    }
}

impl std::str::FromStr for CucumberMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(|c| !matches!(c, 'v' | '>' | '.' | '\n')) {
            bail!("Unrecognized character in map");
        }
        let width = s.lines().next().map(|line| line.len()).unwrap_or(0);
        if s.lines().any(|line| line.len() != width) {
            bail!("Non rectangular map detected");
        }

        let eastbound = s
            .lines()
            .map(|line| line.bytes().map(|b| b == b'>').collect())
            .collect::<Vec<_>>();
        let southbound = s
            .lines()
            .map(|line| line.bytes().map(|b| b == b'v').collect())
            .collect();
        Ok(CucumberMap {
            eastbound,
            southbound,
        })
    }
}

impl std::fmt::Display for CucumberMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![];
        for (eastbound_row, southbound_row) in self.eastbound.iter().zip(&self.southbound) {
            lines.push(
                (0..eastbound_row.len())
                    .map(|x| {
                        if eastbound_row[x] {
                            '>'
                        } else if southbound_row[x] {
                            'v'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>(),
            )
        }
        write!(f, "{}", lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &'static str = "\
        v...>>.vv>\n\
        .vv>>.vv..\n\
        >>.>v>...v\n\
        >>v>>.>.v.\n\
        v>v.vv.v..\n\
        >.>>..v...\n\
        .vv..>.>v.\n\
        v.v..>>v.v\n\
        ....v..v.>";

    #[test]
    fn single_step() {
        let mut map: CucumberMap = TEST_DATA.parse().unwrap();
        let expected: CucumberMap = "\
            ....>.>v.>\n\
            v.v>.>v.v.\n\
            >v>>..>v..\n\
            >>v>v>.>.v\n\
            .>v.v...v.\n\
            v>>.>vvv..\n\
            ..v...>>..\n\
            vv...>>vv.\n\
            >.v.v..v.v"
            .parse()
            .unwrap();
        map.step();

        assert_eq!(map, expected);
    }

    #[test]
    fn count_steps() {
        let mut map: CucumberMap = TEST_DATA.parse().unwrap();
        let result = map.advance_until_stopped();

        assert_eq!(result, 58);
    }
}
