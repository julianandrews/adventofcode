use super::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileMap<T> {
    pub rows: Vec<Vec<T>>,
    pub width: usize,
}

impl<T> TileMap<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.rows.get(y)?.get(x)
    }

    pub fn set(&mut self, x: usize, y: usize, tile: T) -> Option<T>
    where
        T: Copy,
    {
        let old = self.rows.get_mut(y)?.get_mut(x)?;
        let result = *old;
        *old = tile;
        Some(result)
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn step<D: Direction>(&self, x: usize, y: usize, direction: D) -> Option<(usize, usize)> {
        fn clamp(value: usize, limit: usize) -> Option<usize> {
            if value <= limit {
                Some(value)
            } else {
                None
            }
        }
        let (dx, dy) = direction.step();
        Some((
            clamp(x.checked_add_signed(dx.into())?, self.width() - 1)?,
            clamp(y.checked_add_signed(dy.into())?, self.height() - 1)?,
        ))
    }

    pub fn iter_coords(&'_ self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.rows.len()).flat_map(move |y| (0..self.rows[y].len()).map(move |x| (x, y)))
    }

    pub fn find(&self, tile: T) -> Option<(usize, usize)>
    where
        T: PartialEq,
    {
        self.iter_coords()
            .find(|&(x, y)| self.get(x, y) == Some(&tile))
    }
}

impl<T: TryFrom<char>> std::str::FromStr for TileMap<T> {
    type Err = <T as TryFrom<char>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().map(T::try_from).collect::<Result<_, _>>())
            .collect::<Result<_, _>>()?;
        let width = rows.iter().map(|row| row.len()).max().unwrap_or(0);
        Ok(Self { rows, width })
    }
}

impl<T> std::fmt::Display for TileMap<T>
where
    char: for<'a> From<&'a T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lines: Vec<String> = self
            .rows
            .iter()
            .map(|row| row.iter().map(char::from).collect())
            .collect();

        write!(f, "{}", lines.join("\n"))
    }
}
