use super::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileMap<T> {
    rows: Vec<Vec<T>>,
    width: usize,
}

impl<T> TileMap<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.rows.get(y)?.get(x)
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn step(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        fn clamped_increment(value: usize, limit: usize) -> Option<usize> {
            if value < limit {
                Some(value + 1)
            } else {
                None
            }
        }
        match direction {
            Direction::North => Some((x, y.checked_sub(1)?)),
            Direction::South => Some((x, clamped_increment(y, self.height() - 1)?)),
            Direction::West => Some((x.checked_sub(1)?, y)),
            Direction::East => Some((clamped_increment(x, self.width() - 1)?, y)),
        }
    }

    pub fn iter_coords(&'_ self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.rows.len()).flat_map(move |y| (0..self.rows[y].len()).map(move |x| (x, y)))
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
