use super::graph::Graph;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone)]
pub struct TraversalNode<T> {
    pub value: T,
    pub index: u64,
    pub depth: u64,
    pub parent: Option<Rc<TraversalNode<T>>>,
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

impl<T: Clone> IntoIterator for TraversalNode<T> {
    type Item = T;
    type IntoIter = TraversalPathIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        TraversalPathIterator {
            node: Some(Rc::new(self)),
        }
    }
}

pub struct BFSTraversal<'a, G: Graph<'a>> {
    index: u64,
    graph: &'a G,
    queue: VecDeque<TraversalNode<G::Item>>,
    seen: HashSet<G::Item>,
}

pub fn bfs<'a, G: Graph<'a>>(graph: &'a G, start: G::Item) -> BFSTraversal<'a, G>
where
    G::Item: Eq + Hash + Clone,
{
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

impl<'a, G: Graph<'a>> Iterator for BFSTraversal<'a, G>
where
    G::Item: Eq + Hash + Clone,
{
    type Item = Rc<TraversalNode<G::Item>>;

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
