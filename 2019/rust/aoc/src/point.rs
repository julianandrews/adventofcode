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

impl<T> ops::Index<usize> for Point2D<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!(format!("Invalid index {} for Point2D", index)),
        }
    }
}

impl<T> ops::IndexMut<usize> for Point2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!(format!("Invalid index {} for Point2D", index)),
        }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: ops::Add<Output = T>> ops::Add for Point3D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::AddAssign for Point3D<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl<T> ops::Index<usize> for Point3D<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(format!("Invalid index {} for Point3D", index)),
        }
    }
}

impl<T> ops::IndexMut<usize> for Point3D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(format!("Invalid index {} for Point3D", index)),
        }
    }
}

impl From<Point3D<i8>> for Point3D<i64> {
    fn from(point: Point3D<i8>) -> Self {
        Self {
            x: point.x.into(),
            y: point.y.into(),
            z: point.z.into(),
        }
    }
}

impl From<Point3D<i8>> for Point3D<i32> {
    fn from(point: Point3D<i8>) -> Self {
        Self {
            x: point.x.into(),
            y: point.y.into(),
            z: point.z.into(),
        }
    }
}
