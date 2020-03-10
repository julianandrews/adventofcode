use super::graph::Graph;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::rc::Rc;

pub struct TraversalNode<T> {
    pub value: T,
    pub index: u64,
    pub depth: u64,
    pub parent: Option<Rc<TraversalNode<T>>>,
}

impl<T> TraversalNode<T> {
    pub fn path(self) -> TraversalPathIterator<T> {
        TraversalPathIterator {
            node: Some(Rc::new(self)),
        }
    }
}

pub struct TraversalPathIterator<T> {
    node: Option<Rc<TraversalNode<T>>>,
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

pub struct BFSTraversal<'a, T, G: Graph<'a, T>> {
    index: u64,
    graph: &'a G,
    queue: VecDeque<TraversalNode<T>>,
    seen: HashSet<T>,
}

pub fn bfs<'a, T: Eq + Hash + Clone, G: Graph<'a, T>>(
    graph: &'a G,
    start: T,
) -> BFSTraversal<'a, T, G> {
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

impl<'a, T: Eq + Hash + Clone, G: Graph<'a, T>> Iterator for BFSTraversal<'a, T, G> {
    type Item = Rc<TraversalNode<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = match self.queue.pop_front() {
            Some(node) => Rc::new(node),
            None => return None,
        };

        for neighbor in self.graph.neighbors(&node.value) {
            self.index += 1;
            if !self.seen.contains(&neighbor) {
                self.seen.insert(neighbor.clone());
                self.queue.push_back(TraversalNode {
                    value: neighbor,
                    index: self.index,
                    depth: node.depth + 1,
                    parent: Some(node.clone()),
                });
            }
        }

        Some(node)
    }
}
