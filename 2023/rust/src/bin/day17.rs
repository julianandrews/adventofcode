use std::collections::BinaryHeap;

use anyhow::{anyhow, Result};
use rustc_hash::FxHashMap;

use aoc::planar::{Direction, TileMap, Turn};
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map = HeatMap(input.trim().parse()?);

    println!("Part 1: {}", part1(&map)?);
    println!("Part 2: {}", part2(&map)?);

    Ok(())
}

fn part1(map: &HeatMap) -> Result<u64> {
    map.minimum_loss(crucible_neighbors)
        .ok_or_else(|| anyhow!("Failed to find path"))
}

fn part2(map: &HeatMap) -> Result<u64> {
    map.minimum_loss(ultra_crucible_neighbors)
        .ok_or_else(|| anyhow!("Failed to find path"))
}

#[derive(Debug, Clone)]
struct HeatMap(TileMap<HeatLoss>);

impl HeatMap {
    fn minimum_loss<F>(&self, neighbors: F) -> Option<u64>
    where
        F: Fn(&TileMap<HeatLoss>, &State) -> Vec<(State, u64)>,
    {
        // Djikstra's algorithm
        let mut costs = FxHashMap::default();
        let mut queue = BinaryHeap::new();
        queue.push(SearchNode::default());
        costs.insert(State::default(), self.0.get(0, 0)?.0);
        #[cfg(feature = "verbose")]
        let mut parents: FxHashMap<State, SearchNode> = FxHashMap::default();

        while let Some(SearchNode { cost, state }) = queue.pop() {
            if state.x == self.0.width() - 1 && state.y == self.0.height() - 1 {
                #[cfg(feature = "verbose")]
                print_chain(&SearchNode { cost, state }, parents);
                return Some(cost);
            } else if cost > *costs.get(&state).unwrap_or(&u64::MAX) {
                continue;
            }

            for (neighbor, neighbor_cost) in neighbors(&self.0, &state) {
                let old_cost = costs.entry(neighbor.clone()).or_insert(u64::MAX);
                let new_cost = cost + neighbor_cost;
                if new_cost < *old_cost {
                    *old_cost = new_cost;
                    #[cfg(feature = "verbose")]
                    parents.insert(
                        neighbor.clone(),
                        SearchNode {
                            cost,
                            state: state.clone(),
                        },
                    );
                    queue.push(SearchNode {
                        cost: new_cost,
                        state: neighbor,
                    });
                }
            }
        }
        None
    }
}

fn crucible_neighbors(map: &TileMap<HeatLoss>, state: &State) -> Vec<(State, u64)> {
    let mut neighbors = vec![];
    for turn in [Turn::Clockwise, Turn::CounterClockwise] {
        let direction = state.direction.turn(turn);
        if let Some((x, y)) = map.step(state.x, state.y, direction) {
            let neighbor = State {
                x,
                y,
                momentum: 1,
                direction,
            };
            neighbors.push((neighbor, map.get(x, y).unwrap().0));
        }
    }
    if state.momentum < 3 {
        if let Some((x, y)) = map.step(state.x, state.y, state.direction) {
            let neighbor = State {
                x,
                y,
                momentum: state.momentum + 1,
                direction: state.direction,
            };
            neighbors.push((neighbor, map.get(x, y).unwrap().0));
        }
    }
    neighbors
}

fn ultra_crucible_neighbors(map: &TileMap<HeatLoss>, state: &State) -> Vec<(State, u64)> {
    let mut neighbors = vec![];
    if state.momentum == 0 || state.momentum >= 4 {
        for turn in [Turn::Clockwise, Turn::CounterClockwise] {
            let direction = state.direction.turn(turn);
            if let Some((x, y)) = map.step(state.x, state.y, direction) {
                let neighbor = State {
                    x,
                    y,
                    momentum: 1,
                    direction,
                };
                neighbors.push((neighbor, map.get(x, y).unwrap().0));
            }
        }
    }
    if state.momentum < 10 {
        if let Some((x, y)) = map.step(state.x, state.y, state.direction) {
            let neighbor = State {
                x,
                y,
                momentum: state.momentum + 1,
                direction: state.direction,
            };
            neighbors.push((neighbor, map.get(x, y).unwrap().0));
        }
    }
    neighbors
}

#[cfg(feature = "verbose")]
fn print_chain(node: &SearchNode, mut parents: FxHashMap<State, SearchNode>) {
    let mut chain = vec![];
    chain.push(node.clone());
    while let Some(node) = parents.remove(&chain.last().unwrap().state) {
        chain.push(node);
    }
    for node in chain.iter().rev() {
        println!("{}\n{:?}\n", node.cost, node.state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct HeatLoss(u64);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct SearchNode {
    cost: u64,
    state: State,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    momentum: usize,
    direction: Direction,
}

impl Default for State {
    fn default() -> Self {
        State {
            x: 0,
            y: 0,
            momentum: 0,
            direction: Direction::East,
        }
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Note that this is reverse ordering since we want a min-heap.
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<char> for HeatLoss {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        c.to_digit(10)
            .map(|d| HeatLoss(d as u64))
            .ok_or_else(|| anyhow!("Failed to parse heat loss: {}", c))
    }
}

impl From<&HeatLoss> for char {
    fn from(h: &HeatLoss) -> Self {
        char::from_digit(h.0 as u32, 10).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        2413432311323\n\
        3215453535623\n\
        3255245654254\n\
        3446585845452\n\
        4546657867536\n\
        1438598798454\n\
        4457876987766\n\
        3637877979653\n\
        4654967986887\n\
        4564679986453\n\
        1224686865563\n\
        2546548887735\n\
        4322674655533";

    #[test]
    fn normal_crucible() {
        let map = HeatMap(TEST_DATA.parse().unwrap());
        assert_eq!(map.minimum_loss(crucible_neighbors), Some(102));
    }

    #[test]
    fn ultra_crucible() {
        let map = HeatMap(TEST_DATA.parse().unwrap());
        assert_eq!(map.minimum_loss(ultra_crucible_neighbors), Some(94));
    }
}
