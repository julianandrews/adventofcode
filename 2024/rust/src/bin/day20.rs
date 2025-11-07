use anyhow::{anyhow, bail, Result};

use aoc::planar::TileMap;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let finder = CheatFinder::new(&input)?;

    println!("Part 1: {}", part1(&finder));
    println!("Part 2: {}", part2(&finder));

    Ok(())
}

fn part1(finder: &CheatFinder) -> usize {
    finder.num_cheats(100, 2)
}

fn part2(finder: &CheatFinder) -> usize {
    finder.num_cheats(100, 20)
}

static CHUNK_SIZE: usize = 5;

#[derive(Debug)]
struct CheatFinder {
    /// Sequence of all visited points in order from start to end.
    /// The index in this vector is the distance from the start.
    points: Vec<(usize, usize)>,

    /// Spatial partitioning of path positions for efficient range queries.
    /// The flattened grid is divided into CHUNK_SIZE × CHUNK_SIZE chunks.
    /// Each chunk contains a list of distances (indices into `distances`) for all path positions
    /// that fall within that chunk sorted largest to smallest.
    chunks: Vec<Vec<usize>>,

    // Number of rows in the spatial grid.
    chunk_rows: usize,

    // Number of columns in the spatial grid.
    chunk_cols: usize,
}

impl CheatFinder {
    fn new(s: &str) -> Result<Self> {
        let track: TileMap<Tile> = s.parse()?;
        let start = track
            .find(Tile::Start)
            .ok_or_else(|| anyhow!("Failed to find track start"))?;

        // Initialize data structures
        let mut points = vec![start];
        let mut visited = vec![false; track.width() * track.height()];
        let chunk_rows = track.height().div_ceil(CHUNK_SIZE);
        let chunk_cols = track.width().div_ceil(CHUNK_SIZE);
        let mut chunks = vec![vec![]; chunk_rows * chunk_cols];
        chunks[(start.1 / CHUNK_SIZE) * chunk_cols + start.0 / CHUNK_SIZE].push(0);

        loop {
            let &(x, y) = points.last().unwrap();
            visited[y * track.width() + x] = true;
            let mut neighbor_count = 0;
            for (nx, ny) in track.manhattan_neighbors(x, y) {
                if visited[ny * track.width() + nx] {
                    continue;
                }
                let tile = *track.get(nx, ny).unwrap();
                if tile == Tile::Wall {
                    continue;
                }
                neighbor_count += 1;
                if neighbor_count > 1 {
                    bail!("Multiple paths detected. Expected unbranched path.");
                }
                chunks[ny / CHUNK_SIZE * chunk_cols + nx / CHUNK_SIZE].push(points.len());
                points.push((nx, ny));
                if tile == Tile::End {
                    let chunks = chunks
                        .into_iter()
                        .map(|chunk| chunk.into_iter().rev().collect())
                        .collect();
                    return Ok(CheatFinder {
                        points,
                        chunks,
                        chunk_rows,
                        chunk_cols,
                    });
                };
            }
        }
    }

    fn num_cheats(&self, min_saved: usize, max_cheat_length: usize) -> usize {
        let mut count = 0;

        // Iterate over every position A in the path
        for (dist_a, &(x_a, y_a)) in self.points.iter().enumerate() {
            // B must be at least this far along the path to potentially save enough steps
            let min_dist_b = dist_a + min_saved + 2;

            // Find all chunks within cheat range of position A
            let chunk_min_x = x_a.saturating_sub(max_cheat_length) / CHUNK_SIZE;
            let chunk_max_x = ((x_a + max_cheat_length) / CHUNK_SIZE).min(self.chunk_cols - 1);
            let chunk_min_y = y_a.saturating_sub(max_cheat_length) / CHUNK_SIZE;
            let chunk_max_y = ((y_a + max_cheat_length) / CHUNK_SIZE).min(self.chunk_rows - 1);

            for chunk_x in chunk_min_x..=chunk_max_x {
                for chunk_y in chunk_min_y..=chunk_max_y {
                    let chunk = &self.chunks[chunk_y * self.chunk_cols + chunk_x];

                    // Check each point in the chunk
                    for &dist_b in chunk.iter().take_while(|&d| *d >= min_dist_b) {
                        let (x_b, y_b) = self.points[dist_b];
                        let cheat_length = x_a.abs_diff(x_b) + y_a.abs_diff(y_b);
                        let saved = dist_b.saturating_sub(dist_a + cheat_length);
                        if cheat_length <= max_cheat_length && saved >= min_saved {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Space,
    Wall,
    Start,
    End,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Space),
            '#' => Ok(Tile::Wall),
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            _ => bail!("Unrecognized tile {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CheatFinder;

    static EXAMPLE: &str = "\
        ###############\n\
        #...#...#.....#\n\
        #.#.#.#.#.###.#\n\
        #S#...#.#.#...#\n\
        #######.#.#.###\n\
        #######.#.#...#\n\
        #######.#.###.#\n\
        ###..E#...#...#\n\
        ###.#######.###\n\
        #...###...#...#\n\
        #.#####.#.###.#\n\
        #.#...#.#.#...#\n\
        #.#.#.#.#.#.###\n\
        #...#...#...###\n\
        ###############";

    #[test]
    fn short_cheat() {
        let finder = CheatFinder::new(EXAMPLE).unwrap();
        let result = finder.num_cheats(10, 2);

        assert_eq!(result, 10);
    }

    #[test]
    fn real_cheat() {
        let finder = CheatFinder::new(EXAMPLE).unwrap();
        let result = finder.num_cheats(50, 20);

        assert_eq!(result, 285);
    }
}
