fn main() -> anyhow::Result<()> {
    let input = aoc_2025::utils::get_input()?;
    let mut map: WarehouseMap = input.trim().parse()?;

    println!("{}", part1(&map));
    println!("{}", part2(&mut map));

    Ok(())
}

fn part1(map: &WarehouseMap) -> usize {
    map.removable().count()
}

fn part2(map: &mut WarehouseMap) -> usize {
    let mut count = 0;
    loop {
        let to_remove = map.removable().collect::<Vec<_>>();
        if to_remove.is_empty() {
            break;
        }
        count += to_remove.len();
        for (x, y) in to_remove {
            map.remove(x, y);
        }
    }
    count
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WarehouseMap {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl WarehouseMap {
    fn removable(&self) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        (0..self.width).flat_map(move |x| {
            (0..self.height)
                .filter(move |&y| {
                    self.get(x as isize, y as isize) == 1 && self.neighbor_count(x, y) < 4
                })
                .map(move |y| (x, y))
        })
    }

    fn neighbor_count(&self, x: usize, y: usize) -> u8 {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let result = OFFSETS
            .iter()
            .map(|&(dx, dy)| self.get(x as isize + dx, y as isize + dy))
            .sum();
        result
    }

    fn get(&self, x: isize, y: isize) -> u8 {
        if x < 0 || y < 0 {
            return 0;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= self.width || y >= self.height {
            return 0;
        }
        self.data[y * self.width + x]
    }

    fn remove(&mut self, x: usize, y: usize) {
        self.data[y * self.width + x] = 0;
    }
}

mod parsing {
    use anyhow::bail;

    use crate::WarehouseMap;

    impl std::str::FromStr for WarehouseMap {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut data = Vec::with_capacity(s.len());
            let width = s.lines().next().map(|line| line.len()).unwrap_or(0);
            let height = s.lines().count();
            for line in s.lines() {
                if line.len() != width {
                    bail!("Uneven map. Line '{}' expected width {}", line, width);
                }
                for c in line.chars() {
                    match c {
                        '.' => data.push(0),
                        '@' => data.push(1),
                        _ => bail!("Unexpected character '{}' in '{}'", c, line),
                    }
                }
            }
            Ok(WarehouseMap {
                data,
                width,
                height,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, WarehouseMap};

    static TEST_DATA: &str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.";

    #[test]
    fn parsing() {
        let map: WarehouseMap = TEST_DATA.parse().unwrap();
        let expected = WarehouseMap {
            data: vec![
                0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0,
                1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1,
                1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1,
                1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0,
            ],
            width: 10,
            height: 10,
        };

        assert_eq!(map, expected);
    }

    #[test]
    fn removable() {
        let map: WarehouseMap = TEST_DATA.parse().unwrap();
        let result: Vec<_> = map.removable().collect();

        assert_eq!(
            result,
            vec![
                (0, 1),
                (0, 4),
                (0, 7),
                (0, 9),
                (2, 0),
                (2, 9),
                (3, 0),
                (5, 0),
                (6, 0),
                (6, 2),
                (8, 0),
                (8, 9),
                (9, 4)
            ]
        );
    }

    #[test]
    fn remove() {
        let mut map: WarehouseMap = TEST_DATA.parse().unwrap();
        let result = part2(&mut map);

        assert_eq!(result, 43);
    }
}
