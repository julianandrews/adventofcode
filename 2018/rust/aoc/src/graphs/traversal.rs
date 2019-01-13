use super::graph::Graph;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

#[derive(Clone)]
pub struct TraversalNode<T> {
    pub value: T,
    pub index: u64,
    pub depth: u64,
    pub parent: Option<Box<TraversalNode<T>>>,
}

impl<T> TraversalNode<T> {
    pub fn path(self) -> TraversalPathIterator<T> {
        TraversalPathIterator {
            node: Some(Box::new(self)),
        }
    }
}

pub struct TraversalPathIterator<T> {
    node: Option<Box<TraversalNode<T>>>,
}

impl<T: Clone> Iterator for TraversalPathIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match &self.node {
            Some(node) => {
                let value = node.value.clone();
                self.node = node.parent.clone();

                Some(value)
            }
            None => return None,
        }
    }
}

pub struct BFSTraversal<T, G: Graph<T>> {
    index: u64,
    graph: G,
    queue: VecDeque<TraversalNode<T>>,
    seen: HashSet<T>,
}

pub fn bfs<T: Clone + Eq + Hash, G: Graph<T>>(graph: G, start: T) -> BFSTraversal<T, G> {
    let mut seen = HashSet::new();
    seen.insert(start.clone());
    let mut queue = VecDeque::new();
    queue.push_back(TraversalNode {
        value: start,
        index: 0,
        depth: 0,
        parent: None,
    });

    BFSTraversal {
        index: 0,
        graph: graph,
        queue: queue,
        seen: seen,
    }
}

impl<T: Clone + Eq + Hash, G: Graph<T>> Iterator for BFSTraversal<T, G> {
    type Item = TraversalNode<T>;

    fn next(&mut self) -> Option<TraversalNode<T>> {
        let node = match self.queue.pop_front() {
            Some(node) => node,
            None => return None,
        };

        for neighbor in self.graph.neighbors(&node.value) {
            self.index += 1;
            if !self.seen.contains(&neighbor) {
                self.seen.insert(neighbor.clone());
                self.queue.push_back(TraversalNode {
                    value: neighbor.clone(),
                    index: self.index,
                    depth: node.depth + 1,
                    parent: Some(Box::new(node.clone())),
                });
            }
        }

        Some(node)
    }
}
