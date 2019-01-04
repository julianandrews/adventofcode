use std::io::{self, Read, Write};
use std::collections::HashSet;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(Debug)]
struct Claim {
    id: isize,
    x: isize,
    y: isize,
    w: isize,
    h: isize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClaimParseError(());

impl ::std::str::FromStr for Claim {
    type Err = ClaimParseError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let v: Vec<isize> = s.split(|c: char| !c.is_numeric()).flat_map(|s| s.parse()).collect();
        if let [id, x, y, w, h] = v[..] {
            Ok(Claim {id, x, y, w, h})
        } else {
            Err(ClaimParseError(()))
        }
    }
}

impl Claim {
    pub fn points(&self) -> Vec<(isize, isize)> {
        let mut points = Vec::new();
        for x in self.x..(self.x + self.w) {
            for y in self.y..(self.y + self.h) {
                points.push((x, y));
            }
        }

        points
    }
}

fn get_overlap<'a>(claims: impl Iterator<Item = &'a Claim>) -> HashSet<(isize, isize)> {
    let mut all_points = HashSet::new();
    let mut overlap = HashSet::new();

    for claim in claims {
        for &point in claim.points().iter() {
            if all_points.contains(&point) {
                overlap.insert(point);
            }
            all_points.insert(point);
        }
    }

    overlap
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let claims: Vec<Claim> = input.lines().flat_map(|line| line.parse()).collect();

    let overlap_count = get_overlap(claims.iter()).len();

    writeln!(io::stdout(), "{}", overlap_count)?;

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let claims: Vec<Claim> = input.lines().flat_map(|line| line.parse()).collect();

    let overlap = get_overlap(claims.iter());

    for claim in claims {
        if !claim.points().iter().any(|point| overlap.contains(point)) {
            writeln!(io::stdout(), "{}", claim.id)?;
        }
    }

    Ok(())
}
