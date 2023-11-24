use std::collections::{BinaryHeap, HashMap};
use std::convert::{TryFrom, TryInto};

use anyhow::{anyhow, bail, Result};

use aoc::utils::get_input;

type Energy = u64;

fn main() -> Result<()> {
    let input = get_input()?;
    let burrow: Burrow = input.trim().parse()?;

    println!("Part 1: {}", part1(burrow.clone())?);
    println!("Part 2: {}", part2(burrow)?);

    Ok(())
}

fn part1(burrow: Burrow) -> Result<Energy> {
    min_energy(burrow).ok_or_else(|| anyhow!("Failed to find path"))
}

fn part2(mut burrow: Burrow) -> Result<Energy> {
    burrow.extend()?;
    min_energy(burrow).ok_or_else(|| anyhow!("Failed to find path"))
}

fn min_energy(burrow: Burrow) -> Option<Energy> {
    // Djijstra's algorithm
    let mut costs = HashMap::new();
    let mut queue = BinaryHeap::new();
    costs.insert(burrow.clone(), 0);
    queue.push(SearchNode {
        cost: 0,
        state: burrow,
    });
    #[cfg(feature = "verbose")]
    let mut parents: HashMap<Burrow, SearchNode> = HashMap::new();

    while let Some(SearchNode { cost, state }) = queue.pop() {
        if state.is_solved() {
            #[cfg(feature = "verbose")]
            print_chain(&SearchNode { cost, state }, parents);
            return Some(cost);
        } else if cost > *costs.get(&state).unwrap_or(&u64::MAX) {
            continue;
        }

        for (neighbor, neighbor_cost) in state.neighbors() {
            let old_cost = costs.entry(neighbor.clone()).or_insert(u64::MAX);
            let new_cost = cost + neighbor_cost;
            if new_cost < *old_cost {
                *old_cost = new_cost;
                #[cfg(feature = "verbose")]
                parents.insert(
                    neighbor.clone(),
                    SearchNode {
                        cost,
                        state: state.clone(),
                    },
                );
                queue.push(SearchNode {
                    cost: new_cost,
                    state: neighbor,
                });
            }
        }
    }
    None
}

#[cfg(feature = "verbose")]
fn print_chain(node: &SearchNode, mut parents: HashMap<Burrow, SearchNode>) {
    let mut chain = vec![];
    chain.push(node.clone());
    while let Some(node) = parents.remove(&chain.last().unwrap().state) {
        chain.push(node);
    }
    for node in chain.iter().rev() {
        println!("{}\n{}\n", node.cost, node.state.render());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchNode {
    cost: u64,
    state: Burrow,
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Note that this is reverse ordering since we want a min-heap.
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn movement_cost(&self) -> u64 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn room(&self) -> Location {
        match self {
            Amphipod::Amber => Location::AmberRoom,
            Amphipod::Bronze => Location::BronzeRoom,
            Amphipod::Copper => Location::CopperRoom,
            Amphipod::Desert => Location::DesertRoom,
        }
    }
}

impl TryFrom<char> for Amphipod {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Amphipod::Amber),
            'B' => Ok(Amphipod::Bronze),
            'C' => Ok(Amphipod::Copper),
            'D' => Ok(Amphipod::Desert),
            _ => bail!("Unrecognized amphipod: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Location {
    LeftHall1,
    LeftHall2,
    RightHall1,
    RightHall2,
    AmberRoom,
    BronzeRoom,
    CopperRoom,
    DesertRoom,
    MidHall1,
    MidHall2,
    MidHall3,
}

impl Location {
    fn is_room(self) -> bool {
        match self {
            Self::AmberRoom | Self::BronzeRoom | Self::CopperRoom | Self::DesertRoom => true,
            _ => false,
        }
    }

    fn waypoints(self, other: Location) -> Vec<Location> {
        use Location::*;
        match (self, other) {
            (AmberRoom, LeftHall2) => vec![LeftHall1],
            (AmberRoom, MidHall2) => vec![MidHall1],
            (AmberRoom, MidHall3) => vec![MidHall1, MidHall2],
            (AmberRoom, RightHall1) => vec![MidHall1, MidHall2, MidHall3],
            (AmberRoom, RightHall2) => vec![MidHall1, MidHall2, MidHall3, RightHall1],

            (BronzeRoom, LeftHall2) => vec![MidHall1, LeftHall1],
            (BronzeRoom, LeftHall1) => vec![MidHall1],
            (BronzeRoom, MidHall3) => vec![MidHall2],
            (BronzeRoom, RightHall1) => vec![MidHall2, MidHall3],
            (BronzeRoom, RightHall2) => vec![MidHall2, MidHall3, RightHall1],

            (CopperRoom, LeftHall1) => vec![MidHall1, MidHall2],
            (CopperRoom, LeftHall2) => vec![MidHall1, MidHall2, LeftHall1],
            (CopperRoom, MidHall1) => vec![MidHall2],
            (CopperRoom, RightHall1) => vec![MidHall3],
            (CopperRoom, RightHall2) => vec![MidHall3, RightHall1],

            (DesertRoom, LeftHall1) => vec![MidHall1, MidHall2, MidHall3],
            (DesertRoom, LeftHall2) => vec![MidHall1, MidHall2, MidHall3, LeftHall1],
            (DesertRoom, MidHall1) => vec![MidHall2, MidHall3],
            (DesertRoom, MidHall2) => vec![MidHall3],
            (DesertRoom, RightHall2) => vec![RightHall1],

            (LeftHall2, AmberRoom) => vec![LeftHall1],
            (LeftHall2, BronzeRoom) => vec![LeftHall1, MidHall1],
            (LeftHall2, CopperRoom) => vec![LeftHall1, MidHall1, MidHall2],
            (LeftHall2, DesertRoom) => vec![LeftHall1, MidHall1, MidHall2, MidHall3],
            (LeftHall1, BronzeRoom) => vec![MidHall1],
            (LeftHall1, CopperRoom) => vec![MidHall1, MidHall2],
            (LeftHall1, DesertRoom) => vec![MidHall1, MidHall2, MidHall3],

            (RightHall2, AmberRoom) => vec![RightHall1, MidHall1, MidHall2, MidHall3],
            (RightHall2, BronzeRoom) => vec![RightHall1, MidHall2, MidHall3],
            (RightHall2, CopperRoom) => vec![RightHall1, MidHall3],
            (RightHall2, DesertRoom) => vec![RightHall1],
            (RightHall1, AmberRoom) => vec![MidHall1, MidHall2, MidHall3],
            (RightHall1, BronzeRoom) => vec![MidHall2, MidHall3],
            (RightHall1, CopperRoom) => vec![MidHall3],

            (MidHall1, CopperRoom) => vec![MidHall2],
            (MidHall1, DesertRoom) => vec![MidHall2, MidHall3],
            (MidHall2, AmberRoom) => vec![MidHall1],
            (MidHall2, DesertRoom) => vec![MidHall3],
            (MidHall3, AmberRoom) => vec![MidHall2, MidHall1],
            (MidHall3, BronzeRoom) => vec![MidHall2],
            _ => vec![],
        }
    }

    fn neighbors(self) -> Vec<(Location, Energy)> {
        use Location::*;
        match self {
            LeftHall1 => vec![
                (AmberRoom, 2),
                (BronzeRoom, 4),
                (CopperRoom, 6),
                (DesertRoom, 8),
            ],
            LeftHall2 => vec![
                (AmberRoom, 3),
                (BronzeRoom, 5),
                (CopperRoom, 7),
                (DesertRoom, 9),
            ],
            RightHall1 => vec![
                (AmberRoom, 8),
                (BronzeRoom, 6),
                (CopperRoom, 4),
                (DesertRoom, 2),
            ],
            RightHall2 => vec![
                (AmberRoom, 9),
                (BronzeRoom, 7),
                (CopperRoom, 5),
                (DesertRoom, 3),
            ],
            AmberRoom => vec![
                (LeftHall1, 2),
                (LeftHall2, 3),
                (MidHall1, 2),
                (MidHall2, 4),
                (MidHall3, 6),
                (RightHall1, 8),
                (RightHall2, 9),
            ],
            BronzeRoom => vec![
                (LeftHall1, 4),
                (LeftHall2, 5),
                (MidHall1, 2),
                (MidHall2, 2),
                (MidHall3, 4),
                (RightHall1, 6),
                (RightHall2, 7),
            ],
            CopperRoom => vec![
                (LeftHall1, 6),
                (LeftHall2, 7),
                (MidHall1, 4),
                (MidHall2, 2),
                (MidHall3, 2),
                (RightHall1, 4),
                (RightHall2, 5),
            ],
            DesertRoom => vec![
                (LeftHall1, 8),
                (LeftHall2, 9),
                (MidHall1, 6),
                (MidHall2, 4),
                (MidHall3, 2),
                (RightHall1, 2),
                (RightHall2, 3),
            ],
            MidHall1 => vec![
                (AmberRoom, 2),
                (BronzeRoom, 2),
                (CopperRoom, 4),
                (DesertRoom, 6),
            ],
            MidHall2 => vec![
                (AmberRoom, 4),
                (BronzeRoom, 2),
                (CopperRoom, 2),
                (DesertRoom, 4),
            ],
            MidHall3 => vec![
                (AmberRoom, 6),
                (BronzeRoom, 4),
                (CopperRoom, 2),
                (DesertRoom, 2),
            ],
        }
    }
}

impl TryFrom<usize> for Location {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Location::LeftHall1),
            1 => Ok(Location::LeftHall2),
            2 => Ok(Location::RightHall1),
            3 => Ok(Location::RightHall2),
            4 => Ok(Location::AmberRoom),
            5 => Ok(Location::BronzeRoom),
            6 => Ok(Location::CopperRoom),
            7 => Ok(Location::DesertRoom),
            8 => Ok(Location::MidHall1),
            9 => Ok(Location::MidHall2),
            10 => Ok(Location::MidHall3),
            _ => bail!("Invalid room enum"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Burrow {
    room_capacity: u64,
    amphipods: [Vec<Amphipod>; 11],
    solved: [u64; 4],
}

impl Burrow {
    pub fn neighbors(&self) -> Vec<(Self, Energy)> {
        let mut neighbors = vec![];
        for (i, amphipods) in self.amphipods.iter().enumerate() {
            let origin: Location = i.try_into().expect("Invalid room index");
            if let Some(amphipod) = amphipods.iter().last() {
                for (destination, distance) in origin.neighbors() {
                    if let Some((neighbor, extra_steps)) = self.neighbor(origin, destination) {
                        let energy = (distance + extra_steps) * amphipod.movement_cost();
                        neighbors.push((neighbor, energy));
                    }
                }
            };
        }
        neighbors
    }

    pub fn is_solved(&self) -> bool {
        self.solved.iter().all(|&count| count == self.room_capacity)
    }

    pub fn extend(&mut self) -> Result<()> {
        if self.amphipods[4..8].iter().any(|l| l.len() != 2) {
            bail!("Expected two amphipods in room");
        }
        let amber_room = self.in_loc_mut(Location::AmberRoom);
        amber_room.insert(1, Amphipod::Desert);
        amber_room.insert(2, Amphipod::Desert);
        let bronze_room = self.in_loc_mut(Location::BronzeRoom);
        bronze_room.insert(1, Amphipod::Bronze);
        bronze_room.insert(2, Amphipod::Copper);
        let copper_room = self.in_loc_mut(Location::CopperRoom);
        copper_room.insert(1, Amphipod::Amber);
        copper_room.insert(2, Amphipod::Bronze);
        let desert_room = self.in_loc_mut(Location::DesertRoom);
        desert_room.insert(1, Amphipod::Copper);
        desert_room.insert(2, Amphipod::Amber);
        self.room_capacity = 4;
        Ok(())
    }

    #[cfg(feature = "verbose")]
    pub fn render(&self) -> String {
        use Location::*;
        let display_char =
            |location: Location, depth: u64| match self.in_loc(location).get(depth as usize) {
                None => '.',
                Some(Amphipod::Amber) => 'A',
                Some(Amphipod::Bronze) => 'B',
                Some(Amphipod::Copper) => 'C',
                Some(Amphipod::Desert) => 'D',
            };
        let mut lines = vec!["#############".to_string()];
        lines.push(format!(
            "#{}{}.{}.{}.{}.{}{}#",
            display_char(LeftHall2, 0),
            display_char(LeftHall1, 0),
            display_char(MidHall1, 0),
            display_char(MidHall2, 0),
            display_char(MidHall3, 0),
            display_char(RightHall1, 0),
            display_char(RightHall2, 0),
        ));
        lines.push(format!(
            "###{}#{}#{}#{}###",
            display_char(AmberRoom, self.room_capacity - 1),
            display_char(BronzeRoom, self.room_capacity - 1),
            display_char(CopperRoom, self.room_capacity - 1),
            display_char(DesertRoom, self.room_capacity - 1),
        ));
        for depth in (0..(self.room_capacity - 1)).rev() {
            lines.push(format!(
                "  #{}#{}#{}#{}#",
                display_char(AmberRoom, depth),
                display_char(BronzeRoom, depth),
                display_char(CopperRoom, depth),
                display_char(DesertRoom, depth),
            ));
        }
        lines.push("  #########".to_string());
        lines.join("\n")
    }

    fn in_loc(&self, location: Location) -> &Vec<Amphipod> {
        &self.amphipods[location as usize]
    }

    fn in_loc_mut(&mut self, location: Location) -> &mut Vec<Amphipod> {
        &mut self.amphipods[location as usize]
    }

    fn neighbor(&self, origin: Location, destination: Location) -> Option<(Self, u64)> {
        if !self.is_valid_move(origin, destination) {
            return None;
        }

        let amphipod = *self.in_loc(origin).last().unwrap();
        let extra_steps = if origin.is_room() {
            self.room_capacity - self.in_loc(origin).len() as u64
        } else {
            self.room_capacity - self.in_loc(destination).len() as u64 - 1
        };
        let mut burrow = self.clone();
        burrow.in_loc_mut(origin).pop().unwrap();
        burrow.in_loc_mut(destination).push(amphipod);
        if destination.is_room() {
            burrow.solved[amphipod as usize] += 1;
        }
        Some((burrow, extra_steps))
    }

    fn is_valid_move(&self, origin: Location, destination: Location) -> bool {
        let amphipod = match self.in_loc(origin).last() {
            None => return false,
            Some(&amphipod) => amphipod,
        };
        let origin_count = self.in_loc(origin).len() as u64;
        let dest_count = self.in_loc(destination).len() as u64;

        // Don't leave if you're where you should be
        if amphipod.room() == origin && self.solved[amphipod as usize] == origin_count {
            return false;
        }
        if destination.is_room() {
            // Don't enter a room you can't stay in
            if !(amphipod.room() == destination) || self.solved[amphipod as usize] < dest_count {
                return false;
            }
            // Don't enter a full room
            if dest_count >= self.room_capacity {
                return false;
            }
        } else if dest_count > 0 {
            // Don't enter an occupied hallway
            return false;
        }
        // Don't pass through an occupied waypoint
        for location in origin.waypoints(destination) {
            if !self.amphipods[location as usize].is_empty() {
                return false;
            }
        }
        true
    }
}

impl std::str::FromStr for Burrow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        if lines.len() < 5 {
            bail!("Not enough lines in input");
        }
        let mut amphipods: [Vec<Amphipod>; 11] = Default::default();
        fn parse_pair(a: Option<char>, b: Option<char>) -> Result<Vec<Amphipod>> {
            Ok(vec![
                a.ok_or(anyhow!("Missing character"))?.try_into()?,
                b.ok_or(anyhow!("Missing character"))?.try_into()?,
            ])
        }
        amphipods[4] = parse_pair(lines[3].chars().nth(3), lines[2].chars().nth(3))?;
        amphipods[5] = parse_pair(lines[3].chars().nth(5), lines[2].chars().nth(5))?;
        amphipods[6] = parse_pair(lines[3].chars().nth(7), lines[2].chars().nth(7))?;
        amphipods[7] = parse_pair(lines[3].chars().nth(9), lines[2].chars().nth(9))?;

        fn count_solved(room: &[Amphipod], amphipod: Amphipod) -> u64 {
            room.iter().take_while(|&a| a == &amphipod).count() as u64
        }
        let mut solved = [0, 0, 0, 0];
        solved[0] = count_solved(&amphipods[4], Amphipod::Amber);
        solved[1] = count_solved(&amphipods[5], Amphipod::Bronze);
        solved[2] = count_solved(&amphipods[6], Amphipod::Copper);
        solved[3] = count_solved(&amphipods[7], Amphipod::Desert);

        Ok(Burrow {
            room_capacity: 2,
            amphipods,
            solved,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        #############\n\
        #...........#\n\
        ###B#C#B#D###\n\
     \x20 #A#D#C#A#\n\
     \x20 #########";

    #[test]
    fn short_hallways() {
        let burrow: Burrow = TEST_DATA.parse().unwrap();
        let energy = min_energy(burrow).unwrap();

        assert_eq!(energy, 12521);
    }

    #[test]
    fn long_hallways() {
        let mut burrow: Burrow = TEST_DATA.parse().unwrap();
        burrow.extend().unwrap();
        let energy = min_energy(burrow).unwrap();

        assert_eq!(energy, 44169);
    }
}
