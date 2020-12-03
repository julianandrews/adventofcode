use aoc::aoc_error::AOCError;
use aoc::utils::get_input;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let tree_map = input.trim().parse()?;

    println!("Part 1: {}", part1(&tree_map));
    println!("Part 2: {}", part2(&tree_map));
    Ok(())
}

fn part1(tree_map: &TreeMap) -> usize {
    tree_map.count_trees(3, 1)
}

fn part2(tree_map: &TreeMap) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .iter()
        .map(|&(dx, dy)| tree_map.count_trees(dx, dy))
        .product()
}

struct TreeMap {
    rows: Vec<Vec<bool>>,
}

impl TreeMap {
    fn has_tree_at(&self, x: usize, y: usize) -> bool {
        self.rows
            .get(y)
            .map(|row| *row.get(x % row.len()).unwrap_or(&false))
            .unwrap_or(false)
    }

    fn count_trees(&self, dx: usize, dy: usize) -> usize {
        (0..self.rows.len())
            .step_by(dy)
            .enumerate()
            .filter(|&(i, y)| self.has_tree_at(i * dx, y))
            .count()
    }
}

impl FromStr for TreeMap {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().any(|c| c != '.' && c != '#' && c != '\n') {
            return Err(AOCError::new("Invalid characters in map"))?;
        }
        let rows: Vec<Vec<bool>> = s
            .lines()
            .map(|line| line.as_bytes().iter().map(|&b| b == b'#').collect())
            .collect();
        let row_length = rows.get(0).map(|row| row.len()).unwrap_or(0);
        if rows.iter().any(|row| row.len() != row_length) {
            return Err(AOCError::new("Inconsistent row lengths in map"))?;
        }

        Ok(TreeMap { rows })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = "..##.......\
                                     \n#...#...#..\
                                     \n.#....#..#.\
                                     \n..#.#...#.#\
                                     \n.#...##..#.\
                                     \n..#.##.....\
                                     \n.#.#.#....#\
                                     \n.#........#\
                                     \n#.##...#...\
                                     \n#...##....#\
                                     \n.#..#...#.#";

    #[test]
    fn test_count_trees() {
        let tree_map: TreeMap = TEST_INPUT.parse().unwrap();
        assert_eq!(tree_map.count_trees(1, 1), 2);
        assert_eq!(tree_map.count_trees(3, 1), 7);
        assert_eq!(tree_map.count_trees(5, 1), 3);
        assert_eq!(tree_map.count_trees(7, 1), 4);
        assert_eq!(tree_map.count_trees(1, 2), 2);
    }
}
