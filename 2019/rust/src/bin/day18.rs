use std::collections::BinaryHeap;
use std::collections::HashMap;

use aoc::aoc_error::AOCError;
use aoc::graphs::{bfs, traversal_path, Graph};
use aoc::simple_bitset::SimpleBitSet;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u64> {
    let maze: KeyMaze = input.parse()?;
    maze.steps()
        .ok_or(AOCError::new("Failed to find path.").into())
}

fn part2(input: &str) -> Result<u64> {
    let input = fix_input(input);
    let maze: KeyMaze = input.parse()?;
    maze.steps()
        .ok_or(AOCError::new("Failed to find path.").into())
}

fn fix_input(input: &str) -> String {
    let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let entry_points: Vec<(usize, usize)> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, c)| *c == '@')
                .map(move |(x, _)| (x, y))
        })
        .collect();
    for (x, y) in entry_points {
        if y > 0 {
            if x > 0 {
                lines[y - 1][x - 1] = '@';
            }
            lines[y - 1][x] = '#';
            if x < lines[y - 1].len() {
                lines[y - 1][x + 1] = '@';
            }
        }
        if x > 0 {
            lines[y][x - 1] = '#';
        }
        lines[y][x] = '#';
        if x < lines[y].len() {
            lines[y][x + 1] = '#';
        }
        if y < lines.len() {
            if x > 0 {
                lines[y + 1][x - 1] = '@';
            }
            lines[y + 1][x] = '#';
            if x < lines[y + 1].len() {
                lines[y + 1][x + 1] = '@';
            }
        }
    }

    let lines: Vec<String> = lines.iter().map(|line| line.iter().collect()).collect();
    lines.join("\n")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum MapTile {
    Path,
    Wall,
    Waypoint(Waypoint),
    Door(Waypoint),
}

/// A simple representation of the maze that tracks the tiles at every point and the locations of
/// waypoints (entrypoints and keys).
#[derive(Debug)]
struct SimpleMaze {
    map: HashMap<Point, MapTile>,
    waypoints: Vec<(Waypoint, Point)>,
}

impl SimpleMaze {
    fn waypoints(&self) -> impl Iterator<Item = &(Waypoint, Point)> {
        self.waypoints.iter()
    }

    fn waypoint_at(&self, point: &Point) -> Option<Waypoint> {
        self.map
            .get(point)
            .and_then(|tile| match tile {
                MapTile::Waypoint(w) => Some(w),
                _ => None,
            })
            .cloned()
    }

    fn door_at(&self, point: &Point) -> Option<Waypoint> {
        self.map
            .get(point)
            .and_then(|tile| match tile {
                MapTile::Door(w) => Some(w),
                _ => None,
            })
            .cloned()
    }
}

impl<'a> Graph<'a> for SimpleMaze {
    type Item = Point;

    fn nodes(&'a self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        unimplemented!();
    }

    fn neighbors(&'a self, value: &Self::Item) -> impl Iterator<Item = Self::Item> + 'a {
        let x = value.x;
        let y = value.y;
        [(1, 0), (0, 1), (0, -1), (-1, 0)]
            .iter()
            .map(move |(dx, dy)| Point {
                x: x + dx,
                y: y + dy,
            })
            .filter(move |p| !matches!(self.map.get(p), Some(MapTile::Wall) | None))
    }
}

impl std::str::FromStr for SimpleMaze {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut map = HashMap::new();
        let mut waypoints = vec![];
        let mut entry_point_count = 0;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = Point {
                    x: x as i64,
                    y: y as i64,
                };
                match c {
                    '@' => {
                        let waypoint = Waypoint::for_entry_point(entry_point_count)?;
                        map.insert(p, MapTile::Waypoint(waypoint));
                        waypoints.push((waypoint, p));
                        entry_point_count += 1;
                    }
                    'a'..='z' => {
                        let waypoint = Waypoint::for_key(c)?;
                        map.insert(p, MapTile::Waypoint(waypoint));
                        waypoints.push((waypoint, p));
                    }
                    'A'..='Z' => {
                        map.insert(p, MapTile::Door(Waypoint::for_door(c)?));
                    }
                    '.' => {
                        map.insert(p, MapTile::Path);
                    }
                    '#' => {
                        map.insert(p, MapTile::Wall);
                    }
                    _ => return Err(AOCError::new("Unexpected map tile").into()),
                }
            }
        }

        Ok(SimpleMaze { map, waypoints })
    }
}

/// An abstraction over the maze that only knows about the distances between waypoints, and the
/// doors between them.
#[derive(Debug)]
struct KeyMaze {
    num_robots: u8,
    distances: HashMap<(Waypoint, Waypoint), u64>,
    doors: HashMap<(Waypoint, Waypoint), SimpleBitSet>,
}

impl KeyMaze {
    fn steps(&self) -> Option<u64> {
        let starting_state = MazeState::new(self.num_robots).unwrap();
        // This is a little inefficient, but we only do it once.
        let goal = self
            .distances
            .keys()
            .fold(SimpleBitSet::new(), |result, &(from, _)| result | from);

        let mut distances: HashMap<MazeState, u64> = HashMap::new();
        distances.insert(starting_state.clone(), 0);

        let mut to_process = BinaryHeap::new();
        to_process.push(DistanceSortable::new(starting_state, 0));
        while let Some(entry) = to_process.pop() {
            let DistanceSortable {
                value: state,
                distance,
            } = entry;
            let best_distance = *distances.get(&state).unwrap_or(&u64::MAX);
            if state.visited_waypoints == goal {
                return Some(distance);
            } else if distance <= best_distance {
                for (from, to, new_state) in self.state_neighbors(&state) {
                    let new_distance = distance
                        + self
                            .distances
                            .get(&(from, to))
                            .expect("All waypoints must have distances");
                    if new_distance < *distances.get(&new_state).unwrap_or(&u64::MAX) {
                        to_process.push(DistanceSortable::new(new_state.clone(), new_distance));
                        distances.insert(new_state, new_distance);
                    }
                }
            }
        }

        None
    }

    fn state_neighbors<'a>(
        &'a self,
        state: &'a MazeState,
    ) -> impl Iterator<Item = (Waypoint, Waypoint, MazeState)> + 'a {
        self.distances
            .keys()
            .filter(move |&(from, to)| {
                if state.visited_waypoints.contains(*to) || !state.robot_locations.contains(*from) {
                    return false;
                }
                let locked_doors = self.doors.get(&(*from, *to)).unwrap_or(&SimpleBitSet(0));
                state.visited_waypoints.contains_set(locked_doors)
            })
            .map(move |&(from, to)| {
                let new_state = MazeState {
                    robot_locations: (state.robot_locations - from) | to,
                    visited_waypoints: state.visited_waypoints | to,
                };

                (from, to, new_state)
            })
    }
}

impl std::str::FromStr for KeyMaze {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let simple_maze: SimpleMaze = s.parse()?;
        let mut distances = HashMap::new();
        let mut doors = HashMap::new();
        let mut num_robots = 0;

        for &(from, position) in simple_maze.waypoints() {
            if from.is_entrypoint() {
                num_robots += 1;
            }
            // This isn't a particularly efficient way to find waypoint distances, but
            // it gets the job done fast enough without much code.
            for node in bfs(&simple_maze, position) {
                if let Some(to) = simple_maze.waypoint_at(&node.value) {
                    distances.insert((from, to), node.depth);
                    for p in traversal_path(node) {
                        if let Some(door) = simple_maze.door_at(&p) {
                            *doors.entry((from, to)).or_insert_with(SimpleBitSet::new) |= door;
                        }
                    }
                }
            }
        }

        Ok(KeyMaze {
            num_robots,
            distances,
            doors,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MazeState {
    robot_locations: SimpleBitSet,
    visited_waypoints: SimpleBitSet,
}

impl MazeState {
    fn new(num_robots: u8) -> Result<Self> {
        let robot_locations = (0..num_robots)
            .map(Waypoint::for_entry_point)
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .fold(SimpleBitSet(0), |result, w| result | w);
        Ok(MazeState {
            robot_locations,
            visited_waypoints: robot_locations,
        })
    }
}

/// A sortable struct suitable for storing arbitrary values in a binary heap.
#[derive(Debug, PartialEq, Eq)]
struct DistanceSortable<T: Eq> {
    value: T,
    distance: u64,
}

impl<T: Eq> DistanceSortable<T> {
    fn new(value: T, distance: u64) -> Self {
        DistanceSortable { value, distance }
    }
}

impl<T: Eq> Ord for DistanceSortable<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<T: Eq> PartialOrd for DistanceSortable<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A key or robot entry point (represented by @ or a letter on the map).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Waypoint(u8);

impl Waypoint {
    fn for_entry_point(i: u8) -> Result<Self> {
        if i > u8::MAX - 27 {
            return Err(AOCError::new("Invalid entry point number").into());
        }
        // entry points are stored after 'z'
        Ok(Self(i + 27))
    }

    fn for_key(c: char) -> Result<Self> {
        if !c.is_ascii_lowercase() {
            return Err(AOCError::new("Invalid key").into());
        }
        Ok(Self(c as u8 - b'a'))
    }

    fn for_door(c: char) -> Result<Self> {
        if !c.is_ascii_uppercase() {
            return Err(AOCError::new("Invalid key").into());
        }
        Ok(Self(c as u8 - b'A'))
    }

    fn is_key(&self) -> bool {
        self.0 <= 26
    }

    fn is_entrypoint(&self) -> bool {
        !self.is_key()
    }
}

impl std::convert::From<Waypoint> for u8 {
    fn from(waypoint: Waypoint) -> u8 {
        waypoint.0
    }
}

#[cfg(test)]
mod tests {
    use super::KeyMaze;

    #[test]
    fn case_1() {
        let input = "#########\
                   \n#b.A.@.a#\
                   \n#########";
        let maze: KeyMaze = input.parse().unwrap();

        assert_eq!(maze.steps(), Some(8));
    }

    #[test]
    fn case_2() {
        let input = "########################\
                   \n#f.D.E.e.C.b.A.@.a.B.c.#\
                   \n######################.#\
                   \n#d.....................#\
                   \n########################";
        let maze: KeyMaze = input.parse().unwrap();

        assert_eq!(maze.steps(), Some(86));
    }

    #[test]
    fn case_3() {
        let input = "########################\
                   \n#...............b.C.D.f#\
                   \n#.######################\
                   \n#.....@.a.B.c.d.A.e.F.g#\
                   \n########################";
        let maze: KeyMaze = input.parse().unwrap();

        assert_eq!(maze.steps(), Some(132));
    }

    #[test]
    fn case_4() {
        let input = "#################\
                   \n#i.G..c...e..H.p#\
                   \n########.########\
                   \n#j.A..b...f..D.o#\
                   \n########@########\
                   \n#k.E..a...g..B.n#\
                   \n########.########\
                   \n#l.F..d...h..C.m#\
                   \n#################";
        let maze: KeyMaze = input.parse().unwrap();

        assert_eq!(maze.steps(), Some(136));
    }

    #[test]
    fn case_5() {
        let input = "########################\
                   \n#@..............ac.GI.b#\
                   \n###d#e#f################\
                   \n###A#B#C################\
                   \n###g#h#i################\
                   \n########################";
        let maze: KeyMaze = input.parse().unwrap();

        assert_eq!(maze.steps(), Some(81));
    }

    #[test]
    fn case_6() {
        let input = "#######\
                   \n#a.#Cd#\
                   \n##@#@##\
                   \n#######\
                   \n##@#@##\
                   \n#cB#Ab#\
                   \n#######";
        let maze: KeyMaze = input.parse().unwrap();

        assert_eq!(maze.steps(), Some(8));
    }

    #[test]
    fn case_7() {
        let input = "###############\
                   \n#d.ABC.#.....a#\
                   \n######@#@######\
                   \n###############\
                   \n######@#@######\
                   \n#b.....#.....c#\
                   \n###############";
        let maze: KeyMaze = input.parse().unwrap();

        assert_eq!(maze.steps(), Some(24));
    }

    #[test]
    fn case_8() {
        let input = "#############\
                   \n#DcBa.#.GhKl#\
                   \n#.###@#@#I###\
                   \n#e#d#####j#k#\
                   \n###C#@#@###J#\
                   \n#fEbA.#.FgHi#\
                   \n#############";
        let maze: KeyMaze = input.parse().unwrap();

        assert_eq!(maze.steps(), Some(32));
    }
}
