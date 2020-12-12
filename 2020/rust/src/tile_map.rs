use std::convert::TryFrom;
use std::str::FromStr;

pub static DIRECTIONS: [(i64, i64); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];
pub static MANHATTAN_DIRECTIONS: [(i64, i64); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileMap<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T> TileMap<T> {
    pub fn get_tile(&self, x: i64, y: i64) -> Option<&T> {
        self.rows
            .get(usize::try_from(y).ok()?)
            .map(|row| row.get(usize::try_from(x).ok()?))
            .flatten()
    }

    pub fn neighbors(&self, x: i64, y: i64) -> Vec<&T> {
        DIRECTIONS
            .iter()
            .filter_map(|(dx, dy)| self.get_tile(x + dx, y + dy))
            .collect()
    }

    pub fn manhattan_neighbors(&self, x: i64, y: i64) -> Vec<&T> {
        MANHATTAN_DIRECTIONS
            .iter()
            .filter_map(|(dx, dy)| self.get_tile(x + dx, y + dy))
            .collect()
    }
}

impl<T> TileMap<T>
where
    T: PartialEq,
{
    pub fn variant_count(&self, value: &T) -> usize {
        self.rows
            .iter()
            .flat_map(|row| row.iter().filter(|&tile| tile == value))
            .count()
    }
}

impl<T> FromStr for TileMap<T>
where
    T: TryFrom<char>,
{
    type Err = <T as TryFrom<char>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().map(T::try_from).collect::<Result<_, _>>())
            .collect::<Result<_, _>>()?;
        Ok(Self { rows })
    }
}

impl<'a, T: 'a> std::fmt::Display for TileMap<T>
where
    T: Clone,
    char: From<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lines: Vec<String> = self
            .rows
            .iter()
            .map(|row| row.iter().map(|t| char::from(t.clone())).collect())
            .collect();

        write!(f, "{}", lines.join("\n"))
    }
}
