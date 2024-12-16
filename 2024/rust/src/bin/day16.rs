use std::collections::BinaryHeap;

use anyhow::{anyhow, bail, Result};

use aoc::planar::{CardinalDirection, Direction, TileMap, Turn};
use rustc_hash::{FxHashMap, FxHashSet};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let maze = Maze(input.trim().parse()?);

    println!("Part 1: {}", part1(&maze)?);
    // println!("Part 2: {}", part2(&maze));

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
        // TODO: Cheaper data structure for costs!
        let mut costs: FxHashMap<State, u64> = FxHashMap::default();
        let mut queue = BinaryHeap::new();
        let start = SearchNode {
            steps: 0,
            turns: 0,
            state: State {
                position: self.0.find(Tile::Start)?,
                orientation: CardinalDirection::East,
            },
        };
        costs.insert(start.state, 0);
        queue.push(start);
        while let Some(node) = queue.pop() {
            let state = node.state;
            let cost = node.cost();
            if self.0.get(state.position.0, state.position.1) == Some(&Tile::End) {
                return Some(cost);
            } else if cost > *costs.get(&state).unwrap_or(&0) {
                continue;
            }
            for neighbor in self.neighbors(node) {
                let old_cost = costs.entry(neighbor.state).or_insert(u64::MAX);
                let new_cost = cost + neighbor.cost();
                if new_cost < *old_cost {
                    *old_cost = new_cost;
                    // TODO: This is wrong for paths over 1000 steps
                    let new_steps = new_cost % 1000;
                    let new_turns = new_cost / 1000;
                    queue.push(SearchNode {
                        steps: new_steps,
                        turns: new_turns,
                        state: neighbor.state,
                    });
                }
            }
        }
        None
    }

    fn best_path_tiles(&self) -> usize {
        todo!();
    }

    fn neighbors(&self, node: SearchNode) -> impl Iterator<Item = SearchNode> + '_ {
        self.immediate_neighbors(node.state).filter_map(|neighbor| {
            let new_node = self.advance(neighbor.state)?;
            Some(SearchNode::new(
                new_node.state,
                new_node.steps + neighbor.steps,
                neighbor.turns + new_node.turns,
            ))
        })
    }

    fn immediate_neighbors(&self, state: State) -> impl Iterator<Item = SearchNode> + '_ {
        let options = [
            (state.orientation, 0),
            (state.orientation.turn(Turn::Clockwise), 1),
            (state.orientation.turn(Turn::CounterClockwise), 1),
        ];
        options.into_iter().filter_map(move |(orientation, turns)| {
            let position = self
                .0
                .step(state.position.0, state.position.1, orientation)?;
            if let Some(Tile::Wall) = self.0.get(position.0, position.1) {
                return None;
            }
            let new_state = State {
                position,
                orientation,
            };
            Some(SearchNode {
                steps: 1,
                turns,
                state: new_state,
            })
        })
    }

    fn advance(&self, mut state: State) -> Option<SearchNode> {
        let mut steps = 0;
        let mut turns = 0;
        loop {
            if let Some(&Tile::End) = self.0.get(state.position.0, state.position.1) {
                return Some(SearchNode::new(state, steps, turns));
            }
            let options: Vec<_> = self.immediate_neighbors(state).collect();
            match &options[..] {
                [] => return None,
                [node] => {
                    state = node.state;
                    turns += node.turns;
                    steps += node.steps;
                }
                _ => return Some(SearchNode::new(state, steps, turns)),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchNode {
    state: State,
    steps: u64,
    turns: u64,
}

impl SearchNode {
    fn new(state: State, steps: u64, turns: u64) -> Self {
        Self {
            state,
            steps,
            turns,
        }
    }

    fn cost(&self) -> u64 {
        self.steps + 1000 * self.turns
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Note that this is reverse ordering since we want a min-heap.
        other.cost().cmp(&self.cost())
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

    // #[test]
    fn best_path_tiles_1() {
        let maze = Maze(EXAMPLE_1.parse().unwrap());
        assert_eq!(maze.best_path_tiles(), 45);
    }

    // #[test]
    fn best_path_tiles_2() {
        let maze = Maze(EXAMPLE_2.parse().unwrap());
        assert_eq!(maze.best_path_tiles(), 64);
    }
}
