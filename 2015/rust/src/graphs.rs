mod graph;

pub use self::graph::Graph;

use std::collections::BinaryHeap;

use rustc_hash::FxHashMap;

pub trait GraphExt<'a>: Graph<'a> {
    /// Return nodes in a topologically sorted order. Returns `None` if no valid sort exists.
    fn toposort(&'a self) -> Option<Vec<Self::Item>>
    where
        Self: Sized,
        <Self as Graph<'a>>::Item: Eq + Ord + Clone + std::hash::Hash,
    {
        let mut num_nodes = 0;
        let mut indegrees = FxHashMap::default();
        let mut queue = BinaryHeap::new();
        for node in self.nodes() {
            indegrees.entry(node.clone()).or_insert(0);
            for neighbor in self.neighbors(&node) {
                *indegrees.entry(neighbor).or_insert(0) += 1;
            }
            num_nodes += 1;
        }
        for node in self.nodes() {
            if indegrees[&node] == 0 {
                queue.push(node);
            }
        }

        let mut result = Vec::with_capacity(num_nodes);
        while let Some(node) = queue.pop() {
            for neighbor in self.neighbors(&node) {
                if let Some(n) = indegrees.get_mut(&neighbor) {
                    *n -= 1;
                    if *n == 0 {
                        queue.push(neighbor);
                    }
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
}

impl<'a, T, G: Graph<'a, Item = T>> GraphExt<'a> for G {}
