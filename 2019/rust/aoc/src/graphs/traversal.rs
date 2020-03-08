use super::graph::Graph;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::rc::Rc;

pub struct TraversalNode<'a, T> {
    pub value: &'a T,
    pub index: u64,
    pub depth: u64,
    pub parent: Option<Rc<TraversalNode<'a, T>>>,
}

impl<'a, T> TraversalNode<'a, T> {
    pub fn path(self) -> TraversalPathIterator<'a, T> {
        TraversalPathIterator {
            node: Some(Rc::new(self)),
        }
    }
}

pub struct TraversalPathIterator<'a, T> {
    node: Option<Rc<TraversalNode<'a, T>>>,
}

impl<'a, T> Iterator for TraversalPathIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match &self.node {
            Some(node) => {
                let value = node.value;
                self.node = node.parent.clone();

                Some(value)
            }
            None => return None,
        }
    }
}

pub struct BFSTraversal<'a, T, G: Graph<T>> {
    index: u64,
    graph: &'a G,
    queue: VecDeque<TraversalNode<'a, T>>,
    seen: HashSet<&'a T>,
}

pub fn bfs<'a, T: Eq + Hash, G: Graph<T>>(graph: &'a G, start: &'a T) -> BFSTraversal<'a, T, G> {
    let mut seen = HashSet::new();
    seen.insert(start);
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

impl<'a, T: Eq + Hash, G: Graph<T>> Iterator for BFSTraversal<'a, T, G> {
    type Item = Rc<TraversalNode<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = match self.queue.pop_front() {
            Some(node) => Rc::new(node),
            None => return None,
        };

        for neighbor in self.graph.neighbors(node.value) {
            self.index += 1;
            if !self.seen.contains(neighbor) {
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
