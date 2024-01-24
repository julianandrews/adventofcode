#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimpleBitSet(pub u64);

impl Default for SimpleBitSet {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleBitSet {
    pub fn new() -> Self {
        SimpleBitSet(0)
    }

    pub fn contains<T: Into<u8>>(&self, value: T) -> bool {
        self.0 & (1 << value.into()) != 0
    }

    pub fn contains_set(&self, other: &SimpleBitSet) -> bool {
        self.0 & other.0 == other.0
    }
}

impl<T: Into<u8>> std::ops::BitOr<T> for SimpleBitSet {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self {
        SimpleBitSet(self.0 | (1 << rhs.into()))
    }
}

impl<T: Into<u8>> std::ops::BitOrAssign<T> for SimpleBitSet {
    fn bitor_assign(&mut self, rhs: T) {
        self.0 |= 1 << rhs.into();
    }
}

impl<T: Into<u8>> std::ops::Sub<T> for SimpleBitSet {
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        SimpleBitSet(self.0 & !(1 << rhs.into()))
    }
}
