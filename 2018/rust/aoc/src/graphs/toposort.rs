use super::graph::Graph;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

pub type NeighborFunc<T> = Fn(&T) -> std::vec::Vec<T>;

pub fn toposort<T: Eq + Hash + Ord + Clone, G: Graph<T>>(graph: G) -> Option<Vec<T>> {
    let num_values = graph.values().len();
    let mut indegrees = HashMap::new();
    let mut queue = BinaryHeap::new();
    for value in &graph.values() {
        indegrees.entry(value.clone()).or_insert(0);
        for neighbor in graph.neighbors(value) {
            *indegrees.entry(neighbor).or_insert(0) += 1;
        }
    }
    for value in graph.values() {
        if indegrees[&value] == 0 {
            queue.push(value);
        }
    }

    let mut result = Vec::with_capacity(num_values);
    while queue.len() > 0 {
        let value = queue.pop().unwrap();
        for neighbor in graph.neighbors(&value) {
            *indegrees.get_mut(&neighbor).unwrap() -= 1;
            if indegrees[&neighbor] == 0 {
                queue.push(neighbor);
            }
        }
        result.push(value);
    }

    if result.len() == num_values {
        Some(result)
    } else {
        Some(result)
        // None
    }
}
