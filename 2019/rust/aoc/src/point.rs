use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Point2D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
