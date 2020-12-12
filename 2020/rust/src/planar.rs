use self::Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn manhattan_norm(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64
    }

    pub fn advance(&mut self, direction: Direction, distance: u64) {
        match direction {
            North => self.y += distance as i64,
            South => self.y -= distance as i64,
            East => self.x += distance as i64,
            West => self.x -= distance as i64,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn left_turn(&self) -> Self {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn right_turn(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}
