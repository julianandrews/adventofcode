use std::collections::VecDeque;

use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let graph: ComponentGraph = input.trim().parse()?;

    println!("Part 1: {}", part1(&graph)?);

    Ok(())
}

fn part1(graph: &ComponentGraph) -> Result<usize> {
    let (a, b) = graph.find_cut(3).ok_or(anyhow!("Failed to find cut"))?;
    Ok(a * b)
}

#[derive(Debug, Clone)]
struct ComponentGraph {
    edges: Vec<Vec<bool>>,
}

impl ComponentGraph {
    /// Try to cut the graph into two components and return their sizes.
    fn find_cut(&self, max_cuts: usize) -> Option<(usize, usize)> {
        // Pick a random node, and then try all other nodes in random order.
        let mut nodes: Vec<_> = (0..self.edges.len()).collect();
        nodes.shuffle(&mut rand::thread_rng());
        let i = *nodes.first()?;
        for &j in &nodes[1..] {
            if let Some(result) = self.component_sizes(i, j, max_cuts) {
                return Some(result);
            }
        }
        None
    }

    /// Try to cut a graph into two components with node `i` in one component and `j` in the
    /// other using at most `max_cuts` cuts. Return the component sizes if a cut exists.
    fn component_sizes(&self, i: usize, j: usize, max_cuts: usize) -> Option<(usize, usize)> {
        let mut graph = self.clone();
        for _ in 0..max_cuts + 1 {
            if let Some(component_size) = graph.find_component_size_or_remove_path(i, j) {
                return Some((component_size, self.edges.len() - component_size));
            }
        }
        None
    }

    /// If `start` is not connected to `end` return the size of the component containing `start`.
    /// Otherwise find a path between `start` and `end` and remove all its edges.
    fn find_component_size_or_remove_path(&mut self, start: usize, end: usize) -> Option<usize> {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut parents = vec![None; self.edges.len()];
        while let Some(i) = queue.pop_front() {
            if i == end {
                let mut child = end;
                while child != start {
                    let parent: usize = parents[child].unwrap();
                    self.edges[child][parent] = false;
                    self.edges[parent][child] = false;
                    child = parent;
                }
                return None;
            }
            for (j, &is_edge) in self.edges[i].iter().enumerate() {
                if is_edge && parents[j].is_none() {
                    parents[j] = Some(i);
                    queue.push_back(j);
                }
            }
        }
        Some(parents.into_iter().filter(|o| o.is_some()).count())
    }
}

mod parsing {
    use super::ComponentGraph;

    use anyhow::anyhow;
    use rustc_hash::{FxHashMap, FxHashSet};

    impl std::str::FromStr for ComponentGraph {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut nodes_by_str: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();
            for line in s.lines() {
                let (label, neighbor_part) = line
                    .split_once(": ")
                    .ok_or_else(|| anyhow!("Invalid line {}", line))?;
                for neighbor in neighbor_part.split(' ') {
                    nodes_by_str.entry(label).or_default().insert(neighbor);
                    nodes_by_str.entry(neighbor).or_default().insert(label);
                }
            }
            let node_indices: FxHashMap<&str, usize> = nodes_by_str
                .keys()
                .enumerate()
                .map(|(i, &s)| (s, i))
                .collect();

            let mut edges = vec![vec![false; nodes_by_str.len()]; nodes_by_str.len()];
            for (label, neighbors) in nodes_by_str.into_iter() {
                let node_ix = *node_indices.get(label).unwrap();
                for neighbor in neighbors {
                    let neighbor_ix = *node_indices.get(neighbor).unwrap();
                    edges[node_ix][neighbor_ix] = true;
                    edges[neighbor_ix][node_ix] = true;
                }
            }
            Ok(ComponentGraph { edges })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentGraph;

    static TEST_DATA: &str = "\
        jqt: rhn xhk nvd\n\
        rsh: frs pzl lsr\n\
        xhk: hfx\n\
        cmg: qnr nvd lhk bvb\n\
        rhn: xhk bvb hfx\n\
        bvb: xhk hfx\n\
        pzl: lsr hfx nvd\n\
        qnr: nvd\n\
        ntq: jqt hfx bvb xhk\n\
        nvd: lhk\n\
        lsr: lhk\n\
        rzs: qnr cmg lsr rsh\n\
        frs: qnr lhk lsr";

    #[test]
    fn cut() {
        let graph: ComponentGraph = TEST_DATA.parse().unwrap();
        let (a, b) = graph.find_cut(3).unwrap();
        let (a, b) = (a.min(b), a.max(b));

        assert_eq!((a, b), (6, 9));
    }
}
