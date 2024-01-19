use aoc::utils::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    let grid: LightGrid = input.trim().parse()?;

    println!("Part 1: {}", part1(grid.clone()));
    println!("Part 2: {}", part2(grid.clone()));

    Ok(())
}

fn part1(mut grid: LightGrid) -> usize {
    grid.run(100);
    grid.light_count()
}

fn part2(mut grid: LightGrid) -> usize {
    grid.run_fixed(100);
    grid.light_count()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LightGrid {
    rows: Vec<Vec<bool>>,
}

impl LightGrid {
    fn run(&mut self, time: usize) {
        for _ in 0..time {
            self.tick();
        }
    }

    fn run_fixed(&mut self, time: usize) {
        self.fix();
        for _ in 0..time {
            self.tick();
            self.fix();
        }
    }

    fn tick(&mut self) {
        self.rows = self
            .rows
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, _)| self.should_light_up(x, y))
                    .collect()
            })
            .collect();
    }

    fn fix(&mut self) {
        let x_max = self.width().saturating_sub(1);
        let y_max = self.height().saturating_sub(1);
        for (x, y) in [(0, 0), (0, y_max), (x_max, 0), (x_max, y_max)] {
            if let Some(b) = self.get_mut(x, y) {
                *b = true;
            }
        }
    }

    fn should_light_up(&self, x: usize, y: usize) -> bool {
        let mut count = 0;
        for y in y.saturating_sub(1)..=y + 1 {
            for x in x.saturating_sub(1)..=x + 1 {
                if let Some(true) = self.get(x, y) {
                    count += 1;
                }
            }
        }
        count == 3 || (count == 4 && self.get(x, y).unwrap_or(false))
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.rows.get(y)?.get(x).copied()
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut bool> {
        self.rows.get_mut(y)?.get_mut(x)
    }

    fn width(&self) -> usize {
        self.rows.first().map(|row| row.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn light_count(&self) -> usize {
        self.rows.iter().flatten().filter(|&b| *b).count()
    }
}

mod parsing {
    use super::LightGrid;

    use anyhow::bail;

    impl std::str::FromStr for LightGrid {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let rows: Vec<Vec<bool>> = s
                .lines()
                .map(|line| {
                    line.bytes()
                        .map(|b| match b {
                            b'#' => Ok(true),
                            b'.' => Ok(false),
                            _ => bail!("Unrecognized character {} in map", b as char),
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<_, _>>()?;
            Ok(LightGrid { rows })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LightGrid;

    static TEST_DATA: &str = "\
        .#.#.#\n\
        ...##.\n\
        #....#\n\
        ..#...\n\
        #.#..#\n\
        ####..";

    #[test]
    fn run() {
        let mut grid: LightGrid = TEST_DATA.parse().unwrap();
        grid.run(5);
        let expected: LightGrid = "\
            ......\n\
            ......\n\
            ..##..\n\
            ..##..\n\
            ......\n\
            ......"
            .parse()
            .unwrap();

        assert_eq!(grid, expected);
    }

    #[test]
    fn run_fixed() {
        let mut grid: LightGrid = TEST_DATA.parse().unwrap();
        grid.run_fixed(5);
        let expected: LightGrid = "\
            ##.###\n\
            .##..#\n\
            .##...\n\
            .##...\n\
            #.#...\n\
            ##...#"
            .parse()
            .unwrap();

        assert_eq!(grid, expected);
    }
}
