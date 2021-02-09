use Orientation::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash)]
/// The dihedral group of order 8.
/// The main diagonal joins vertices 1 and 3.
pub enum Orientation {
    Identity,
    Rotate90,
    Rotate180,
    Rotate270,
    HorizontalFlip,
    SecondDiagonalFlip,
    VerticalFlip,
    MainDiagonalFlip,
}

impl Orientation {
    pub fn iterator() -> impl Iterator<Item = Self> {
        [
            Identity,
            Rotate90,
            Rotate180,
            Rotate270,
            HorizontalFlip,
            SecondDiagonalFlip,
            VerticalFlip,
            MainDiagonalFlip,
        ]
        .iter()
        .copied()
    }

    pub fn is_rotation(&self) -> bool {
        match self {
            Identity | Rotate90 | Rotate180 | Rotate270 => true,
            _ => false,
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            Identity => Identity,
            Rotate90 => Rotate270,
            Rotate180 => Rotate180,
            Rotate270 => Rotate90,
            HorizontalFlip => HorizontalFlip,
            SecondDiagonalFlip => SecondDiagonalFlip,
            VerticalFlip => VerticalFlip,
            MainDiagonalFlip => MainDiagonalFlip,
        }
    }

    /// Multiply by Rotate90
    pub fn rotate(&self) -> Self {
        match self {
            Identity => Rotate90,
            Rotate90 => Rotate180,
            Rotate180 => Rotate270,
            Rotate270 => Identity,
            HorizontalFlip => SecondDiagonalFlip,
            SecondDiagonalFlip => VerticalFlip,
            VerticalFlip => MainDiagonalFlip,
            MainDiagonalFlip => HorizontalFlip,
        }
    }

    /// Flip across the second diagonal (vertices 2 and 4).
    pub fn flip(&self) -> Self {
        match self {
            Identity => SecondDiagonalFlip,
            Rotate90 => HorizontalFlip,
            Rotate180 => MainDiagonalFlip,
            Rotate270 => VerticalFlip,
            HorizontalFlip => Rotate90,
            SecondDiagonalFlip => Identity,
            VerticalFlip => Rotate270,
            MainDiagonalFlip => Rotate180,
        }
    }
}

impl std::ops::Mul for Orientation {
    type Output = Self;

    /// a * b = result of a applied after b
    fn mul(self, rhs: Self) -> Self {
        match self {
            Identity => rhs,
            Rotate90 => rhs.rotate(),
            Rotate180 => rhs.rotate().rotate(),
            Rotate270 => rhs.rotate().rotate().rotate(),
            HorizontalFlip => rhs.rotate().flip(),
            SecondDiagonalFlip => rhs.flip(),
            VerticalFlip => rhs.flip().rotate(),
            MainDiagonalFlip => rhs.flip().rotate().rotate(),
        }
    }
}
