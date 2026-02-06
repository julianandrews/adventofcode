use anyhow::{bail, Result};

use aoc_2025::byte_grid::ByteGrid;

fn main() -> Result<()> {
    let input = aoc_2025::utils::get_input()?;
    let grid = TachyonGrid::new(input.trim())?;

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));

    Ok(())
}

fn part1(grid: &TachyonGrid) -> u64 {
    grid.count_splitters()
}

fn part2(grid: &TachyonGrid) -> u64 {
    grid.count_paths()
}

struct TachyonGrid<'a> {
    byte_grid: ByteGrid<'a>,
    starts: Vec<usize>,
}

impl<'a> TachyonGrid<'a> {
    fn count_splitters(&self) -> u64 {
        let mut count = 0;
        let mut beams: Vec<bool> = self.byte_grid.row(0).iter().map(|&b| b == b'S').collect();
        for y in 1..self.byte_grid.height {
            for x in self.byte_grid.find_in_row(b'^', y) {
                if beams[x] {
                    count += 1;
                    beams.splice(x - 1..=x + 1, [true, false, true]);
                }
            }
        }
        count
    }

    fn count_paths(&self) -> u64 {
        let mut path_counts = vec![1; self.byte_grid.width];
        for y in (1..self.byte_grid.height).rev() {
            for x in self.byte_grid.find_in_row(b'^', y) {
                path_counts[x] = path_counts[x + 1] + path_counts[x - 1];
            }
        }
        self.starts.iter().map(|&x| path_counts[x]).sum()
    }

    pub fn new(s: &'a str) -> Result<Self> {
        let byte_grid = ByteGrid::new(s)?;
        let starts: Vec<usize> = byte_grid.find_in_row(b'S', 0).collect();

        // Validate assumptions:
        // - Only 'S', '^', and '.' appear in the diagram
        // - 'S' only appears in the first row
        // - 'S' and '^' are never on the sides
        // - '^^' never appears (neccessary for single-pass beams splicing)
        if byte_grid.row(0).iter().any(|&b| !matches!(b, b'S' | b'.')) {
            bail!("Unexpected byte in first row.");
        }
        if starts.first() == Some(&0) || starts.last() == Some(&byte_grid.width.saturating_sub(1)) {
            bail!("Beam start at grid edge.");
        }
        for row in byte_grid.rows().skip(1) {
            if row.windows(2).any(|pair| matches!(pair, [b'^', b'^'])) {
                bail!("Adjacent splitters");
            }
            if row.iter().any(|&b| !matches!(b, b'^' | b'.')) {
                bail!("Unexpected byte in grid");
            }
            if row.last() == Some(&b'^') || row.first() == Some(&b'^') {
                bail!("Splitter at grid edge");
            }
        }

        Ok(TachyonGrid { byte_grid, starts })
    }
}

#[cfg(test)]
mod tests {
    use crate::TachyonGrid;

    static TEST_DATA: &str = concat!(
        ".......S.......\n",
        "...............\n",
        ".......^.......\n",
        "...............\n",
        "......^.^......\n",
        "...............\n",
        ".....^.^.^.....\n",
        "...............\n",
        "....^.^...^....\n",
        "...............\n",
        "...^.^...^.^...\n",
        "...............\n",
        "..^...^.....^..\n",
        "...............\n",
        ".^.^.^.^.^...^.\n",
        "..............."
    );

    #[test]
    fn count_splitters() {
        let grid = TachyonGrid::new(TEST_DATA).unwrap();
        let result = grid.count_splitters();

        assert_eq!(result, 21);
    }

    #[test]
    fn count_paths() {
        let grid = TachyonGrid::new(TEST_DATA).unwrap();
        let result = grid.count_paths();

        assert_eq!(result, 40);
    }
}
