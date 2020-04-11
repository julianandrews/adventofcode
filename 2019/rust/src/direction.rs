use self::Direction::*;
use super::point::Point2D;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub fn right_turn(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn left_turn(&self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn offset(&self) -> Point2D<i8> {
        match self {
            North => Point2D { x: 0, y: 1 },
            East => Point2D { x: 1, y: 0 },
            South => Point2D { x: 0, y: -1 },
            West => Point2D { x: -1, y: 0 },
        }
    }

    pub fn next_position<T: std::ops::Add<Output = T> + std::convert::From<i8>>(
        &self,
        point: Point2D<T>,
    ) -> Point2D<T> {
        let offset = self.offset();
        Point2D::<T> {
            x: point.x + offset.x.into(),
            y: point.y + offset.y.into(),
        }
    }
    pub fn iterator() -> impl Iterator<Item = Direction> {
        [North, South, East, West].iter().copied()
    }
}
