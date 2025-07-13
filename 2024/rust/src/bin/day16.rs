use std::collections::BinaryHeap;

use anyhow::{anyhow, bail, Result};

use aoc::planar::{CardinalDirection, Direction, TileMap, Turn};
use rustc_hash::{FxHashMap, FxHashSet};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let maze = Maze(input.trim().parse()?);

    println!("Part 1: {}", part1(&maze)?);
    println!("Part 2: {}", part2(&maze));

    Ok(())
}

fn part1(maze: &Maze) -> Result<u64> {
    maze.min_score()
        .ok_or_else(|| anyhow!("Failed to find path through maze"))
}

fn part2(maze: &Maze) -> usize {
    maze.best_path_tiles()
}

#[derive(Debug, Clone)]
struct Maze(TileMap<Tile>);

impl Maze {
    fn min_score(&self) -> Option<u64> {
        let initial_state = State::new(self.0.find(Tile::Start)?, CardinalDirection::East);
        let mut scores = FxHashMap::default();
        scores.insert(initial_state, 0);
        let mut queue = BinaryHeap::new();
        queue.push(SearchNode::new(initial_state, 0));

        while let Some(node) = queue.pop() {
            if self.0.get(node.state.position.0, node.state.position.1) == Some(&Tile::End) {
                return Some(node.score);
            } else if matches!(scores.get(&node.state), Some(&score) if node.score > score) {
                continue;
            }

            for neighbor in self.neighbors(node.state) {
                let score = scores.entry(neighbor).or_insert(u64::MAX);
                let is_turn = neighbor.orientation != node.state.orientation;
                let new_score = node.score.saturating_add(if is_turn { 1001 } else { 1 });
                if new_score < *score {
                    *score = new_score;
                    queue.push(SearchNode::new(neighbor, new_score));
                }
            }
        }
        None
    }

    fn best_path_tiles(&self) -> usize {
        let start_tile = match self.0.find(Tile::Start) {
            Some(position) => position,
            None => return 0,
        };
        let initial_state = State::new(start_tile, CardinalDirection::East);
        let mut scores = FxHashMap::default();
        scores.insert(initial_state, 0);
        let mut queue = BinaryHeap::new();
        queue.push(SearchNode::new(initial_state, 0));
        let mut tiles = FxHashSet::default();
        let mut parents: FxHashMap<State, Vec<State>> = FxHashMap::default();
        let mut best_score = u64::MAX;

        while let Some(node) = queue.pop() {
            if node.score > best_score {
                break;
            }
            if self.0.get(node.state.position.0, node.state.position.1) == Some(&Tile::End) {
                best_score = node.score;
                // Do DFS on parent tree to find all tiles.
                let mut to_visit = vec![&node.state];
                while let Some(state) = to_visit.pop() {
                    tiles.insert(state.position);
                    parents.get(state).map(|states| to_visit.extend(states));
                }
                continue;
            } else if matches!(scores.get(&node.state), Some(&score) if node.score > score) {
                continue;
            }

            for neighbor in self.neighbors(node.state) {
                let score = scores.entry(neighbor).or_insert(u64::MAX);
                let is_turn = neighbor.orientation != node.state.orientation;
                let new_score = node.score.saturating_add(if is_turn { 1001 } else { 1 });
                if new_score < *score {
                    *score = new_score;
                    queue.push(SearchNode::new(neighbor, new_score));
                    parents.insert(neighbor, vec![node.state]);
                } else if new_score == *score {
                    parents.entry(neighbor).or_default().push(node.state);
                }
            }
        }
        tiles.len()
    }

    fn neighbors(&self, state: State) -> impl Iterator<Item = State> + '_ {
        // This assumes we can't reverse on the first turn. This is true for my input and both test
        // cases.
        let options = [
            state.orientation,
            state.orientation.turn(Turn::Clockwise),
            state.orientation.turn(Turn::CounterClockwise),
        ];
        options.into_iter().filter_map(move |orientation| {
            let (x, y) = state.position;
            let position = self.0.step(x, y, orientation)?;
            if let Some(Tile::Wall) = self.0.get(position.0, position.1) {
                return None;
            }
            Some(State::new(position, orientation))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SearchNode {
    state: State,
    score: u64,
}

impl SearchNode {
    fn new(state: State, score: u64) -> Self {
        Self { state, score }
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score) // Note that this is reverse ordering since we want a min-heap.
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    position: (usize, usize),
    orientation: CardinalDirection,
}

impl State {
    fn new(position: (usize, usize), orientation: CardinalDirection) -> Self {
        Self {
            position,
            orientation,
        }
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
    use super::*;

    static EXAMPLE_1: &str = "\
        ###############\n\
        #.......#....E#\n\
        #.#.###.#.###.#\n\
        #.....#.#...#.#\n\
        #.###.#####.#.#\n\
        #.#.#.......#.#\n\
        #.#.#####.###.#\n\
        #...........#.#\n\
        ###.#.#####.#.#\n\
        #...#.....#.#.#\n\
        #.#.#.###.#.#.#\n\
        #.....#...#.#.#\n\
        #.###.#.#.#.#.#\n\
        #S..#.....#...#\n\
        ###############";

    static EXAMPLE_2: &str = "\
        #################\n\
        #...#...#...#..E#\n\
        #.#.#.#.#.#.#.#.#\n\
        #.#.#.#...#...#.#\n\
        #.#.#.#.###.#.#.#\n\
        #...#.#.#.....#.#\n\
        #.#.#.#.#.#####.#\n\
        #.#...#.#.#.....#\n\
        #.#.#####.#.###.#\n\
        #.#.#.......#...#\n\
        #.#.###.#####.###\n\
        #.#.#...#.....#.#\n\
        #.#.#.#####.###.#\n\
        #.#.#.........#.#\n\
        #.#.#.#########.#\n\
        #S#.............#\n\
        #################";

    #[test]
    fn min_score_1() {
        let maze = Maze(EXAMPLE_1.parse().unwrap());
        assert_eq!(maze.min_score(), Some(7036));
    }

    #[test]
    fn min_score_2() {
        let maze = Maze(EXAMPLE_2.parse().unwrap());
        assert_eq!(maze.min_score(), Some(11048));
    }

    #[test]
    fn best_path_tiles_1() {
        let maze = Maze(EXAMPLE_1.parse().unwrap());
        assert_eq!(maze.best_path_tiles(), 45);
    }

    #[test]
    fn best_path_tiles_2() {
        let maze = Maze(EXAMPLE_2.parse().unwrap());
        assert_eq!(maze.best_path_tiles(), 64);
    }
}
