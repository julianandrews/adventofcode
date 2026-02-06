pub struct ByteGrid<'a> {
    pub data: &'a [u8],
    pub width: usize,
    pub height: usize,
}

impl<'a> ByteGrid<'a> {
    pub fn new(s: &'a str) -> Result<Self, ByteGridError> {
        let width = s.lines().next().map(|line| line.len()).unwrap_or(0);
        let height = s.lines().count();
        if s.lines().any(|line| line.len() != width) {
            return Err(ByteGridError::UnevenGrid);
        }
        Ok(ByteGrid {
            data: s.as_bytes(),
            width,
            height,
        })
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.data.get(y * self.width + 1 + x).copied()
    }

    pub fn row(&self, y: usize) -> &[u8] {
        let stride = self.width + 1;
        let start = y * stride;
        &self.data[start..start + self.width]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[u8]> {
        (0..self.height).map(|y| self.row(y))
    }

    pub fn find_in_row(&self, value: u8, y: usize) -> impl Iterator<Item = usize> + '_ {
        self.row(y)
            .iter()
            .enumerate()
            .filter(move |(_, &b)| b == value)
            .map(|(x, _)| x)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ByteGridError {
    #[error("Uneven grid")]
    UnevenGrid,
}
