use super::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Direction> {
        [Self::North, Self::South, Self::East, Self::West]
            .iter()
            .copied()
    }

    pub fn unit_vector(&self) -> Point {
        match self {
            Direction::North => Point { x: 0, y: 1 },
            Direction::South => Point { x: 0, y: -1 },
            Direction::East => Point { x: 1, y: 0 },
            Direction::West => Point { x: -1, y: 0 },
        }
    }
}
