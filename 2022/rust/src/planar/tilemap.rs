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

    pub fn iter_coords(&'_ self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.rows.len()).flat_map(move |y| (0..self.rows[y].len()).map(move |x| (x, y)))
    }

    pub fn iter_dir(&'_ self, x: usize, y: usize, dir: Direction) -> impl Iterator<Item = &T> + '_ {
        DirIterator {
            map: self,
            x,
            y,
            dir,
        }
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
            .map(|row| row.iter().map(|t| char::from(t)).collect())
            .collect();

        write!(f, "{}", lines.join("\n"))
    }
}

struct DirIterator<'a, T> {
    map: &'a TileMap<T>,
    x: usize,
    y: usize,
    dir: Direction,
}

impl<'a, T: 'a> Iterator for DirIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.x, self.y) = match self.dir {
            Direction::North => (self.x, self.y.checked_sub(1)?),
            Direction::South => (self.x, self.y.checked_add(1)?),
            Direction::West => (self.x.checked_sub(1)?, self.y),
            Direction::East => (self.x.checked_add(1)?, self.y),
        };
        self.map.get(self.x, self.y)
    }
}
