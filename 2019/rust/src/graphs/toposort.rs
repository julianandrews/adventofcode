use super::graph::Graph;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

pub fn toposort<'a, G: Graph<'a>>(graph: &'a G) -> Option<Vec<G::Item>>
where
    G::Item: Eq + Hash + Ord + Clone,
{
    let mut num_nodes = 0;
    let mut indegrees = HashMap::new();
    let mut queue = BinaryHeap::new();
    for node in graph.nodes() {
        indegrees.entry(node.clone()).or_insert(0);
        for neighbor in graph.neighbors(&node) {
            *indegrees.entry(neighbor).or_insert(0) += 1;
        }
        num_nodes += 1;
    }
    for node in graph.nodes() {
        if indegrees[&node] == 0 {
            queue.push(node);
        }
    }

    let mut result = Vec::with_capacity(num_nodes);
    while queue.len() > 0 {
        let node = queue.pop().unwrap();
        for neighbor in graph.neighbors(&node) {
            // TODO: Don't just unwrap here, this represents a real logic error where there's a
            // neighbor that wasn't in the graph's `nodes`.
            *indegrees.get_mut(&neighbor).unwrap() -= 1;
            if indegrees[&neighbor] == 0 {
                queue.push(neighbor);
            }
        }
        result.push(node);
    }

    if result.len() == num_nodes {
        Some(result)
    } else {
        None
    }
}
