use std::convert::TryFrom;

use num_enum::{IntoPrimitive, TryFromPrimitive};

pub trait Direction: Sized {
    fn turn(&self, turn: Turn) -> Self;
    fn step(&self) -> (i8, i8);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..4).map(|i| Self::try_from(i).unwrap())
    }
}

impl Direction for CardinalDirection {
    fn turn(&self, turn: Turn) -> Self {
        let i = match turn {
            Turn::Clockwise => (u8::from(*self) + 1).rem_euclid(4),
            Turn::CounterClockwise => (u8::from(*self) + 3).rem_euclid(4),
        };
        Self::try_from(i).unwrap()
    }

    fn step(&self) -> (i8, i8) {
        match self {
            Self::North => (0, 1),
            Self::East => (1, 0),
            Self::South => (0, -1),
            Self::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum CompassPoint {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl CompassPoint {
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..8).map(|i| Self::try_from(i).unwrap())
    }
}

impl Direction for CompassPoint {
    fn turn(&self, turn: Turn) -> Self {
        let i = match turn {
            Turn::Clockwise => (u8::from(*self) + 1).rem_euclid(8),
            Turn::CounterClockwise => (u8::from(*self) + 7).rem_euclid(8),
        };
        Self::try_from(i).unwrap()
    }

    fn step(&self) -> (i8, i8) {
        match self {
            Self::N => (0, 1),
            Self::NE => (1, 1),
            Self::E => (1, 0),
            Self::SE => (1, -1),
            Self::S => (0, -1),
            Self::SW => (-1, -1),
            Self::W => (-1, 0),
            Self::NW => (-1, 1),
        }
    }
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
