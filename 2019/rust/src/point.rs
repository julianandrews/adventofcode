use std::convert::From;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

pub enum Axis2D {
    X,
    Y,
}

impl Axis2D {
    pub fn iter() -> impl Iterator<Item = Axis3D> {
        [Axis3D::X, Axis3D::Y, Axis3D::Z].iter().copied()
    }
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

impl<T> ops::Index<Axis2D> for Point2D<T> {
    type Output = T;

    fn index(&self, index: Axis2D) -> &Self::Output {
        match index {
            Axis2D::X => &self.x,
            Axis2D::Y => &self.y,
        }
    }
}

impl<T> ops::IndexMut<Axis2D> for Point2D<T> {
    fn index_mut(&mut self, index: Axis2D) -> &mut Self::Output {
        match index {
            Axis2D::X => &mut self.x,
            Axis2D::Y => &mut self.y,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis3D {
    X,
    Y,
    Z,
}

impl Axis3D {
    pub fn iter() -> impl Iterator<Item = Axis3D> {
        [Axis3D::X, Axis3D::Y, Axis3D::Z].iter().copied()
    }
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

impl<T> ops::Index<Axis3D> for Point3D<T> {
    type Output = T;

    fn index(&self, index: Axis3D) -> &Self::Output {
        match index {
            Axis3D::X => &self.x,
            Axis3D::Y => &self.y,
            Axis3D::Z => &self.z,
        }
    }
}

impl<T> ops::IndexMut<Axis3D> for Point3D<T> {
    fn index_mut(&mut self, index: Axis3D) -> &mut Self::Output {
        match index {
            Axis3D::X => &mut self.x,
            Axis3D::Y => &mut self.y,
            Axis3D::Z => &mut self.z,
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
