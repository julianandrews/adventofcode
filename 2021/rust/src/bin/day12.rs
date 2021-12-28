use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: CaveMap<2> = input.trim().parse()?;

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
    Ok(())
}

fn part1(map: &CaveMap<2>) -> u64 {
    map.path_count(false)
}

fn part2(map: &CaveMap<2>) -> u64 {
    map.path_count(true)
}

#[derive(Debug)]
struct CaveMap<const N: usize> {
    routes: HashMap<Cave<N>, Vec<Cave<N>>>,
}

impl<const N: usize> CaveMap<N> {
    fn path_count(&self, allow_revisit: bool) -> u64 {
        let mut path_count = 0;
        let mut to_visit = vec![(Cave::Start, HashSet::new(), !allow_revisit)];
        while let Some((cave, visited, revisit_used)) = to_visit.pop() {
            match cave {
                Cave::End => path_count += 1,
                _ => {
                    let mut visited = visited.clone();
                    if let Cave::SmallCave(_) = cave {
                        visited.insert(cave);
                    }
                    for neighbor in self.neighbors(cave) {
                        if !revisit_used {
                            let uses_revisit = revisit_used || visited.contains(&neighbor);
                            to_visit.push((neighbor, visited.clone(), uses_revisit));
                        } else {
                            if !matches!(neighbor, Cave::SmallCave(_))
                                || !visited.contains(&neighbor)
                            {
                                to_visit.push((neighbor, visited.clone(), revisit_used));
                            }
                        }
                    }
                }
            }
        }
        path_count
    }

    fn neighbors(&self, cave: Cave<N>) -> impl Iterator<Item = Cave<N>> {
        let neighbors = self.routes.get(&cave).cloned().unwrap_or(vec![]);
        neighbors.into_iter()
    }
}

impl<const N: usize> std::str::FromStr for CaveMap<N> {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut routes = HashMap::new();
        for line in s.lines() {
            let (a, b) = line
                .split_once('-')
                .ok_or(AOCError::new("Failed to parse line"))?;
            let (a, b): (Cave<N>, Cave<N>) = (a.parse()?, b.parse()?);
            if let (Cave::BigCave(_), Cave::BigCave(_)) = (a, b) {
                return Err(Box::new(AOCError::new("Invalid route between big caves")));
            }
            for (from, to) in [(a, b), (b, a)] {
                if !matches!(to, Cave::Start) && !matches!(from, Cave::End) {
                    routes.entry(from).or_insert_with(Vec::new).push(to);
                }
            }
        }
        Ok(Self { routes })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cave<const N: usize> {
    Start,
    End,
    SmallCave([u8; N]),
    BigCave([u8; N]),
}

impl<const N: usize> std::str::FromStr for Cave<N> {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            _ => {
                let bytes: [u8; N] = s.as_bytes().try_into()?;
                if bytes.iter().all(|b| b.is_ascii_uppercase()) {
                    Ok(Cave::BigCave(bytes))
                } else {
                    Ok(Cave::SmallCave(bytes))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static SMALL_EXAMPLE: &str = "start-A\
                                \nstart-b\
                                \nA-c\
                                \nA-b\
                                \nb-d\
                                \nA-end\
                                \nb-end";
    static MEDIUM_EXAMPLE: &str = "dc-end\
                                 \nHN-start\
                                 \nstart-kj\
                                 \ndc-start\
                                 \ndc-HN\
                                 \nLN-dc\
                                 \nHN-end\
                                 \nkj-sa\
                                 \nkj-HN\
                                 \nkj-dc";
    static LARGE_EXAMPLE: &str = "fs-end\
                                \nhe-DX\
                                \nfs-he\
                                \nstart-DX\
                                \npj-DX\
                                \nend-zg\
                                \nzg-sl\
                                \nzg-pj\
                                \npj-he\
                                \nRW-he\
                                \nfs-DX\
                                \npj-RW\
                                \nzg-RW\
                                \nstart-pj\
                                \nhe-WI\
                                \nzg-he\
                                \npj-fs\
                                \nstart-RW";

    #[test]
    fn path_count_1() {
        let map: CaveMap<1> = SMALL_EXAMPLE.parse().unwrap();
        assert_eq!(map.path_count(false), 10);
    }

    #[test]
    fn path_count_2() {
        let map: CaveMap<2> = MEDIUM_EXAMPLE.parse().unwrap();
        assert_eq!(map.path_count(false), 19);
    }

    #[test]
    fn path_count_3() {
        let map: CaveMap<2> = LARGE_EXAMPLE.parse().unwrap();
        assert_eq!(map.path_count(false), 226);
    }

    #[test]
    fn path_count_with_revist_1() {
        let map: CaveMap<1> = SMALL_EXAMPLE.parse().unwrap();
        assert_eq!(map.path_count(true), 36);
    }

    #[test]
    fn path_count_with_revist_2() {
        let map: CaveMap<2> = MEDIUM_EXAMPLE.parse().unwrap();
        assert_eq!(map.path_count(true), 103);
    }

    #[test]
    fn path_count_with_revist_3() {
        let map: CaveMap<2> = LARGE_EXAMPLE.parse().unwrap();
        assert_eq!(map.path_count(true), 3509);
    }
}
