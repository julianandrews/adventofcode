#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Turn {
    Clockwise,
    CounterClockwise,
}

impl Turn {
    pub fn reverse(&self) -> Self {
        match self {
            Turn::Clockwise => Turn::CounterClockwise,
            Turn::CounterClockwise => Turn::Clockwise,
        }
    }
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Direction> {
        [Self::North, Self::East, Self::South, Self::West]
            .iter()
            .copied()
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn turn(&self, turn: Turn) -> Self {
        let i = match turn {
            Turn::Clockwise => (*self as i8 + 1).rem_euclid(4),
            Turn::CounterClockwise => (*self as i8 - 1).rem_euclid(4),
        };
        match i {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!(),
        }
    }

    pub fn step(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}
