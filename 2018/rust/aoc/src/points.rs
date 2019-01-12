use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x: x, y: y }
    }

    pub fn manhattan_neighbors(&self) -> HashSet<Point> {
        let mut result = HashSet::new();
        result.insert(Point::new(self.x - 1, self.y));
        result.insert(Point::new(self.x + 1, self.y));
        result.insert(Point::new(self.x, self.y - 1));
        result.insert(Point::new(self.x, self.y + 1));

        result
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointParseError(());

impl ::std::str::FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let v: Vec<isize> = s.split(", ").flat_map(|v| v.parse()).collect();
        if let [x, y] = v[..] {
            Ok(Point { x, y })
        } else {
            Err(PointParseError(()))
        }
    }
}
