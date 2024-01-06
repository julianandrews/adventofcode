use anyhow::{anyhow, Result};
use rand::Rng;
use rustc_hash::FxHashMap;

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
    nodes: Vec<(usize, FxHashMap<usize, usize>)>,
}

impl ComponentGraph {
    fn find_cut(&self, max_edges: usize) -> Option<(usize, usize)> {
        // Based on Karger's aglorithm
        loop {
            let graph = self.random_cut();
            let mut nodes = graph.nodes.iter().filter(|&(count, _)| *count != 0);
            let (a, edges) = nodes.next().unwrap();
            let a = *a;
            let &(b, _) = nodes.next().unwrap();
            let edges = *edges.values().next().unwrap();
            if edges <= max_edges {
                return Some((a.min(b), a.max(b)));
            }
        }
    }

    fn random_edge(&self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let edge_count = self.nodes.iter().map(|(_, edges)| edges.len()).sum();
        let choice = rng.gen_range(0..edge_count);
        self.nodes
            .iter()
            .enumerate()
            .flat_map(|(i, (_, edges))| edges.keys().map(move |&j| (i, j)))
            .nth(choice)
            .unwrap()
    }

    fn random_cut(&self) -> ComponentGraph {
        let mut graph = self.clone();
        while graph.node_count() > 2 {
            let (i, j) = graph.random_edge();
            graph.contract_edge(i, j);
        }
        graph
    }

    fn contract_edge(&mut self, i: usize, j: usize) {
        let (i, j) = (i.min(j), i.max(j));
        self.nodes[i].0 += self.nodes[j].0;
        self.nodes[j].0 = 0;
        self.nodes[i].1.remove(&j);
        self.nodes[j].1.remove(&i);
        let edges = std::mem::take(&mut self.nodes[j].1);
        for (k, count) in edges {
            *self.nodes[i].1.entry(k).or_insert(0) += count;
            self.nodes[k].1.remove(&j);
            *self.nodes[k].1.entry(i).or_insert(0) += count;
        }
    }

    fn node_count(&self) -> usize {
        self.nodes.iter().filter(|(count, _)| count > &0).count()
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
                for neighbor in neighbor_part.split(" ") {
                    nodes_by_str.entry(label).or_default().insert(neighbor);
                    nodes_by_str.entry(neighbor).or_default().insert(label);
                }
            }
            let node_indices: FxHashMap<&str, usize> = nodes_by_str
                .keys()
                .enumerate()
                .map(|(i, &s)| (s, i))
                .collect();
            let mut nodes = vec![(1, FxHashMap::default()); nodes_by_str.len()];
            for (label, neighbors) in nodes_by_str.into_iter() {
                let node_ix = *node_indices.get(label).unwrap();
                for neighbor in neighbors {
                    let neighbor_ix = *node_indices.get(neighbor).unwrap();
                    nodes[node_ix].1.insert(neighbor_ix, 1);
                }
            }
            Ok(ComponentGraph { nodes })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(graph.find_cut(3), Some((6, 9)));
    }
}
