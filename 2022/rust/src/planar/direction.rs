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
}
