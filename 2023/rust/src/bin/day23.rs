use anyhow::{bail, Result};
use rustc_hash::{FxHashMap, FxHashSet};

use aoc::planar::{Direction, TileMap};
use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let map = input.trim().parse()?;

    println!("Part 1: {}", part1(&map)?);
    println!("Part 2: {}", part2(&map)?);

    Ok(())
}

fn part1(map: &ForestMap) -> Result<u64> {
    let graph = ForestGraph::new(map, SlopeType::Unpassable);
    graph.longest_path()
}

fn part2(map: &ForestMap) -> Result<u64> {
    let graph = ForestGraph::new(map, SlopeType::Passable);
    graph.longest_path()
}

#[derive(Debug, Clone)]
struct ForestMap {
    map: TileMap<ForestTile>,
    entrance: (usize, usize),
    exit: (usize, usize),
}

impl ForestMap {
    fn step(
        &self,
        x: usize,
        y: usize,
        direction: Direction,
        slope_type: SlopeType,
    ) -> Option<(usize, usize)> {
        if let Some((new_x, new_y)) = self.map.step(x, y, direction) {
            let &tile = self.map.get(new_x, new_y).unwrap();
            match ForestMap::can_move(direction, tile, slope_type) {
                true => Some((new_x, new_y)),
                false => None,
            }
        } else {
            None
        }
    }

    fn advance(
        &self,
        point: (usize, usize),
        direction: Direction,
        slope_type: SlopeType,
    ) -> Option<((usize, usize), u64)> {
        let (mut x, mut y) = point;
        let mut direction = direction;
        for distance in 1.. {
            let mut options = Direction::iterator().filter_map(|d| {
                if d == direction.reverse() {
                    return None;
                }
                Some((self.step(x, y, d, slope_type)?, d))
            });
            let next = options.next()?;
            if next.0 == self.exit || next.0 == self.entrance {
                return Some((next.0, distance));
            } else if options.next().is_some() {
                return Some(((x, y), distance));
            }
            ((x, y), direction) = next;
        }
        unreachable!();
    }

    fn can_move(direction: Direction, tile: ForestTile, slope_type: SlopeType) -> bool {
        match slope_type {
            SlopeType::Unpassable => match direction {
                Direction::North => matches!(tile, ForestTile::Path | ForestTile::NorthSlope),
                Direction::East => matches!(tile, ForestTile::Path | ForestTile::EastSlope),
                Direction::South => matches!(tile, ForestTile::Path | ForestTile::SouthSlope),
                Direction::West => matches!(tile, ForestTile::Path | ForestTile::WestSlope),
            },
            SlopeType::Passable => !matches!(tile, ForestTile::Forest),
        }
    }

    /// Walk through forest to find which directions are cut off.
    ///
    /// The entrance is always at (1, 0) with the exit at the opposite corner. As a result the outside
    /// edge of the map is divided into two forests - the south-west and north-east forests.
    ///
    /// If you've gone from the entrance to a node next to the south-west forest going west can never
    /// get you to the exit without crossing your path.
    ///
    /// Similarly, if you're at a node next to the north-east forest, going north can never take you
    /// to the exit without crossing your path.
    fn forbidden_dir(&self, x: usize, y: usize, direction: Direction) -> Option<Direction> {
        if self.entrance.0 != 1 || self.exit.0 != self.map.width() - 2 {
            // This optimization assumes the entrance and exit are in specific places.
            // Allow all directions if the assumption doesn't hold.
            return None;
        }
        // Do a DFS for to find the west or east edge.
        let mut stack = vec![(x, y, direction)];
        let mut visited = FxHashSet::default();
        while let Some((x, y, direction)) = stack.pop() {
            visited.insert((x, y));
            if let Some((new_x, new_y)) = self.map.step(x, y, direction) {
                if visited.contains(&(new_x, new_y)) {
                    continue;
                }
                if self.map.get(new_x, new_y) == Some(&ForestTile::Forest) {
                    if new_x == 0 {
                        return Some(Direction::West); // We're in the south-west forest
                    } else if new_x == self.map.width() - 1 {
                        return Some(Direction::North); // We're in the north-east forest
                    }
                    for new_direction in Direction::iterator() {
                        stack.push((new_x, new_y, new_direction));
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Default)]
struct ForestGraph {
    nodes: Vec<(usize, usize)>,
    edges: Vec<Vec<(usize, u64)>>,
    entrance: usize,
    exit: usize,
}

impl ForestGraph {
    fn new(map: &ForestMap, slope_type: SlopeType) -> Self {
        let mut builder = ForestGraphBuilder::new();
        let entrance = builder.add_node(map.entrance);
        let exit = builder.add_node(map.exit);

        let mut stack = vec![(map.entrance, entrance, Direction::South)];
        while let Some((start, start_node, direction)) = stack.pop() {
            let ((x, y), distance) = match map.advance(start, direction, slope_type) {
                Some(v) => v,
                None => continue,
            };
            let end_node = match builder.node_index(x, y) {
                Some(end_node) => end_node,
                None => {
                    let end_node = builder.add_node((x, y));
                    let mut neighbors: Vec<_> = Direction::iterator()
                        .filter_map(|new_dir| {
                            map.step(x, y, new_dir, slope_type)
                                .map(|new_point| (new_point, end_node, new_dir))
                        })
                        .collect();
                    // Since this is a planar graph we can prune some directions when we know we're
                    // at the outside edge of the map. This gives a 4-5x speedup.
                    let blocked_dirs: Vec<_> = Direction::iterator()
                        .filter(|d| neighbors.iter().all(|(_, _, nd)| d != nd))
                        .collect();
                    for missing_dir in blocked_dirs {
                        if let Some(fd) = map.forbidden_dir(x, y, missing_dir) {
                            neighbors =
                                neighbors.into_iter().filter(|&(_, _, d)| d != fd).collect();
                        }
                    }
                    stack.extend(neighbors);
                    end_node
                }
            };
            builder.add_edge(start_node, end_node, distance);
        }
        builder.into_graph(entrance, exit)
    }

    fn longest_path(&self) -> Result<u64> {
        if self.nodes.len() > 64 {
            bail!("Too many nodes.");
        }
        let mut longest = 0;
        let mut stack = vec![(self.entrance, 0, 1u64 << self.entrance)];
        while let Some((i, path_length, visited)) = stack.pop() {
            if i == self.exit {
                longest = path_length.max(longest);
                continue;
            }
            for &(j, distance) in &self.edges[i] {
                if visited & (1 << j) == 0 {
                    stack.push((j, path_length + distance, visited | (1 << j)));
                }
            }
        }
        if longest == 0 && self.entrance != self.exit {
            bail!("No paths found.")
        }
        Ok(longest)
    }
}

#[derive(Debug, Clone)]
struct ForestGraphBuilder {
    nodes: Vec<(usize, usize)>,
    edges: Vec<Vec<Option<u64>>>,
    node_indices: FxHashMap<(usize, usize), usize>,
}

impl ForestGraphBuilder {
    fn new() -> ForestGraphBuilder {
        ForestGraphBuilder {
            nodes: vec![],
            edges: vec![],
            node_indices: FxHashMap::default(),
        }
    }

    fn add_node(&mut self, point: (usize, usize)) -> usize {
        self.node_indices.insert(point, self.nodes.len());
        self.nodes.push(point);
        for row in &mut self.edges {
            row.push(None);
        }
        self.edges.push(vec![None; self.nodes.len()]);
        self.nodes.len() - 1
    }

    fn add_edge(&mut self, i: usize, j: usize, distance: u64) {
        let edge = self.edges[i][j].get_or_insert(0);
        *edge = distance.max(*edge);
    }

    fn node_index(&self, x: usize, y: usize) -> Option<usize> {
        self.node_indices.get(&(x, y)).copied()
    }

    fn into_graph(self, entrance: usize, exit: usize) -> ForestGraph {
        // Convert from adjacency matrix to lists
        let edges = self
            .edges
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter_map(|(i, distance)| distance.as_ref().map(|&distance| (i, distance)))
                    .collect()
            })
            .collect();
        ForestGraph {
            nodes: self.nodes,
            edges,
            entrance,
            exit,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ForestTile {
    Path,
    Forest,
    NorthSlope,
    EastSlope,
    SouthSlope,
    WestSlope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlopeType {
    Unpassable,
    Passable,
}

mod parsing {
    use super::{ForestMap, ForestTile};

    use anyhow::anyhow;
    use aoc::iterators::AocIterators;
    use aoc::planar::TileMap;

    impl std::str::FromStr for ForestMap {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let map: TileMap<ForestTile> = s.parse()?;
            let [entrance] = (0..map.width())
                .filter(|&x| map.get(x, 0) == Some(&ForestTile::Path))
                .exactly_n::<1>()
                .ok_or_else(|| anyhow!("Expected exactly one entrance"))?;
            let entrance = (entrance, 0);
            let [exit] = (0..map.width())
                .filter(|&x| map.get(x, map.height() - 1) == Some(&ForestTile::Path))
                .exactly_n::<1>()
                .ok_or_else(|| anyhow!("Expected exactly one entrance"))?;
            let exit = (exit, map.height() - 1);
            Ok(ForestMap {
                map,
                entrance,
                exit,
            })
        }
    }

    impl TryFrom<char> for ForestTile {
        type Error = anyhow::Error;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '.' => Ok(ForestTile::Path),
                '#' => Ok(ForestTile::Forest),
                '^' => Ok(ForestTile::NorthSlope),
                '>' => Ok(ForestTile::EastSlope),
                'v' => Ok(ForestTile::SouthSlope),
                '<' => Ok(ForestTile::WestSlope),
                _ => Err(anyhow!("Invalid map tile {}", value)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        #.#####################\n\
        #.......#########...###\n\
        #######.#########.#.###\n\
        ###.....#.>.>.###.#.###\n\
        ###v#####.#v#.###.#.###\n\
        ###.>...#.#.#.....#...#\n\
        ###v###.#.#.#########.#\n\
        ###...#.#.#.......#...#\n\
        #####.#.#.#######.#.###\n\
        #.....#.#.#.......#...#\n\
        #.#####.#.#.#########v#\n\
        #.#...#...#...###...>.#\n\
        #.#.#v#######v###.###v#\n\
        #...#.>.#...>.>.#.###.#\n\
        #####v#.#.###v#.#.###.#\n\
        #.....#...#...#.#.#...#\n\
        #.#########.###.#.#.###\n\
        #...###...#...#...#.###\n\
        ###.###.#.###v#####v###\n\
        #...#...#.#.>.>.#.>.###\n\
        #.###.###.#.###.#.#v###\n\
        #.....###...###...#...#\n\
        #####################.#";

    #[test]
    fn longest_path() {
        let map: ForestMap = TEST_DATA.parse().unwrap();
        let graph = ForestGraph::new(&map, SlopeType::Unpassable);

        assert_eq!(graph.longest_path().unwrap(), 94);
    }

    #[test]
    fn longest_path_with_slopes() {
        let map: ForestMap = TEST_DATA.parse().unwrap();
        let graph = ForestGraph::new(&map, SlopeType::Passable);

        assert_eq!(graph.longest_path().unwrap(), 154);
    }
}
