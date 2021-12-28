use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let map: OctopusMap = input.trim().parse()?;

    println!("Part 1: {}", part1(map.clone()));
    println!("Part 2: {}", part2(map));
    Ok(())
}

fn part1(mut map: OctopusMap) -> u64 {
    map.flash_count(100)
}

fn part2(mut map: OctopusMap) -> u64 {
    map.sync_time()
}

#[derive(Debug, Clone)]
struct OctopusMap {
    energy_levels: Vec<Vec<u64>>,
}

impl OctopusMap {
    fn height(&self) -> usize {
        self.energy_levels.len()
    }

    fn width(&self) -> usize {
        self.energy_levels.get(0).map(|row| row.len()).unwrap_or(0)
    }

    fn contains(&self, x: i64, y: i64) -> bool {
        (0..self.width() as i64).contains(&x) && (0..self.height() as i64).contains(&y)
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let (x, y) = (x as i64, y as i64);
        static OFFSETS: [(i64, i64); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let neighbors: Vec<_> = OFFSETS
            .iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(|(x, y)| self.contains(*x, *y))
            .map(|(x, y)| (x as usize, y as usize))
            .collect();
        neighbors.into_iter()
    }

    fn flash_count(&mut self, steps: u64) -> u64 {
        let mut flash_count = 0;
        for _ in 0..steps {
            flash_count += self.step();
        }
        flash_count
    }

    fn sync_time(&mut self) -> u64 {
        let octopus_count = (self.width() * self.height()) as u64;
        for step in 1.. {
            if self.step() == octopus_count {
                return step;
            }
        }
        unreachable!()
    }

    fn step(&mut self) -> u64 {
        let mut to_process = vec![];
        for x in 0..self.width() {
            for y in 0..self.height() {
                to_process.push((x, y));
            }
        }

        while let Some((x, y)) = to_process.pop() {
            self.energy_levels[y][x] += 1;
            if self.energy_levels[y][x] == 10 {
                for neighbor in self.neighbors(x, y) {
                    to_process.push(neighbor);
                }
            }
        }

        let mut flash_count = 0;
        for x in 0..self.width() {
            for y in 0..self.height() {
                if self.energy_levels[y][x] > 9 {
                    flash_count += 1;
                    self.energy_levels[y][x] = 0
                }
            }
        }
        flash_count
    }
}

impl std::str::FromStr for OctopusMap {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let energy_levels: Vec<Vec<u64>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).map(|x| x as u64))
                    .collect()
            })
            .collect::<Option<_>>()
            .ok_or(AOCError::new("Unrecognized character"))?;
        let width = energy_levels.get(0).map(|row| row.len()).unwrap_or(0);
        if !energy_levels.iter().all(|row| row.len() == width) {
            return Err(Box::new(AOCError::new("Non-rectangular map")));
        }
        Ok(OctopusMap { energy_levels })
    }
}

impl std::fmt::Display for OctopusMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = vec![];
        for row in &self.energy_levels {
            lines.push(row.iter().map(|x| x.to_string()).collect());
        }
        write!(f, "{}", lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = "5483143223\
                            \n2745854711\
                            \n5264556173\
                            \n6141336146\
                            \n6357385478\
                            \n4167524645\
                            \n2176841721\
                            \n6882881134\
                            \n4846848554\
                            \n5283751526";

    #[test]
    fn flash_counts() {
        let mut map: OctopusMap = TEST_DATA.parse().unwrap();
        assert_eq!(map.flash_count(10), 204);
        assert_eq!(map.flash_count(90), 1656 - 204);
    }

    #[test]
    fn sync_time() {
        let mut map: OctopusMap = TEST_DATA.parse().unwrap();
        assert_eq!(map.sync_time(), 195);
    }
}
