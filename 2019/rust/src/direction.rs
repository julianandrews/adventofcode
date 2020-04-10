use super::point::Point2D;

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn right_turn(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn left_turn(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn offset(&self) -> Point2D<i8> {
        match self {
            Direction::North => Point2D { x: 0, y: 1 },
            Direction::East => Point2D { x: 1, y: 0 },
            Direction::South => Point2D { x: 0, y: -1 },
            Direction::West => Point2D { x: -1, y: 0 },
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
}
