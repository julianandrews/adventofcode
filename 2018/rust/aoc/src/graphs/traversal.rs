use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

pub type NeighborFunc<T> = Fn(&T) -> std::vec::Vec<T>;

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

pub struct BFSTraversal<'a, T> {
    index: u64,
    neighbors: &'a NeighborFunc<T>,
    queue: VecDeque<TraversalNode<T>>,
    seen: HashSet<T>,
}

impl<'a, T: Clone + Eq + Hash> BFSTraversal<'a, T> {
    pub fn new(start: T, neighbors: &NeighborFunc<T>) -> BFSTraversal<T> {
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
            neighbors: neighbors,
            queue: queue,
            seen: seen,
        }
    }
}

impl<'a, T: Clone + Eq + Hash> Iterator for BFSTraversal<'a, T> {
    type Item = TraversalNode<T>;

    fn next(&mut self) -> Option<TraversalNode<T>> {
        let node = match self.queue.pop_front() {
            Some(node) => node,
            None => return None,
        };

        for neighbor in (self.neighbors)(&node.value) {
            self.index += 1;
            if !self.seen.contains(&neighbor) {
                self.seen.insert(neighbor.clone());
                self.queue.push_back(TraversalNode {
                    value: neighbor,
                    index: self.index,
                    depth: node.depth + 1,
                    parent: Some(Box::new(node.clone())),
                });
            }
        }

        Some(node)
    }
}
