use std::convert::From;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: ops::Add<Output = T>> ops::Add for Point2D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::AddAssign for Point2D<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl From<Point2D<i8>> for Point2D<i64> {
    fn from(point: Point2D<i8>) -> Self {
        Self {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}

impl From<Point2D<i8>> for Point2D<i32> {
    fn from(point: Point2D<i8>) -> Self {
        Self {
            x: point.x.into(),
            y: point.y.into(),
        }
    }
}
