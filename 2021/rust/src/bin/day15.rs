use std::collections::BinaryHeap;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: CaveMap = input.trim().parse()?;

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
    Ok(())
}

fn part1(map: &CaveMap) -> usize {
    min_risk(map)
}

fn part2(map: &CaveMap) -> usize {
    min_risk(&map.five_x_map())
}

fn min_risk(map: &CaveMap) -> usize {
    let goal = (map.width() - 1, map.height() - 1);
    let mut total_risk = vec![vec![usize::MAX; map.width()]; map.height()];

    let mut heap = BinaryHeap::new();
    heap.push(SearchState {
        risk: 0,
        position: (0, 0),
    });

    while let Some(SearchState { risk, position }) = heap.pop() {
        if position == goal {
            return risk;
        } else if risk > total_risk[position.1][position.0] {
            continue;
        }

        for neighbor in map.neighbors(position) {
            let neighbor_risk = risk + map.risk(neighbor);
            if neighbor_risk < total_risk[neighbor.1][neighbor.0] {
                heap.push(SearchState {
                    risk: neighbor_risk,
                    position: neighbor,
                });
                total_risk[neighbor.1][neighbor.0] = neighbor_risk;
            }
        }
    }
    unreachable!();
}

struct CaveMap {
    risk_levels: Vec<Vec<usize>>,
}

impl CaveMap {
    fn height(&self) -> usize {
        self.risk_levels.len()
    }

    fn width(&self) -> usize {
        self.risk_levels.get(0).map(|row| row.len()).unwrap_or(0)
    }

    fn risk(&self, (x, y): (usize, usize)) -> usize {
        self.risk_levels[y][x]
    }

    fn contains(&self, (x, y): (i64, i64)) -> bool {
        (0..self.width() as i64).contains(&x) && (0..self.height() as i64).contains(&y)
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let (x, y) = (x as i64, y as i64);
        static OFFSETS: [(i64, i64); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
        let neighbors: Vec<_> = OFFSETS
            .iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(|&p| self.contains(p))
            .map(|(x, y)| (x as usize, y as usize))
            .collect();
        neighbors.into_iter()
    }

    fn five_x_map(&self) -> CaveMap {
        let mut risk_levels = vec![];
        for row in &self.risk_levels {
            let mut new_row = vec![];
            for i in 0..5 {
                new_row.extend(row.iter().map(|x| (x + i - 1) % 9 + 1));
            }
            risk_levels.push(new_row);
        }
        let mut new_rows = vec![];
        for i in 1..5 {
            for row in &risk_levels[..self.height()] {
                let new_row = row.iter().map(|x| (x + i - 1) % 9 + 1).collect();
                new_rows.push(new_row);
            }
        }
        risk_levels.extend(new_rows);

        CaveMap { risk_levels }
    }
}

impl std::str::FromStr for CaveMap {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let risk_levels: Vec<Vec<usize>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).map(|x| x as usize))
                    .collect()
            })
            .collect::<Option<_>>()
            .ok_or(AOCError::new("Unrecognized character"))?;
        let width = risk_levels.get(0).map(|row| row.len()).unwrap_or(0);
        if !risk_levels.iter().all(|row| row.len() == width) {
            return Err(Box::new(AOCError::new("Non-rectangular map")));
        }
        Ok(CaveMap { risk_levels })
    }
}

impl std::fmt::Display for CaveMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = vec![];
        for row in &self.risk_levels {
            lines.push(row.iter().map(|x| x.to_string()).collect());
        }
        write!(f, "{}", lines.join("\n"))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchState {
    risk: usize,
    position: (usize, usize),
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = "1163751742\
                            \n1381373672\
                            \n2136511328\
                            \n3694931569\
                            \n7463417111\
                            \n1319128137\
                            \n1359912421\
                            \n3125421639\
                            \n1293138521\
                            \n2311944581";

    #[test]
    fn test_min_risk() {
        let map: CaveMap = TEST_DATA.parse().unwrap();
        assert_eq!(min_risk(&map), 40);
    }

    #[test]
    fn test_five_x_map() {
        let map: CaveMap = TEST_DATA.parse().unwrap();
        let big_map = map.five_x_map();
        println!("{}", big_map);
        assert_eq!(min_risk(&big_map), 315);
    }
}
