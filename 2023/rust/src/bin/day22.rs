use anyhow::Result;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let bricks: Vec<Brick> = parse_fields(input.trim(), '\n')?;
    let brick_stack = BrickStack::from_bricks(bricks);

    println!("Part 1: {}", part1(&brick_stack));
    println!("Part 2: {}", part2(&brick_stack));

    Ok(())
}

fn part1(brick_stack: &BrickStack) -> usize {
    brick_stack
        .essential_bricks()
        .iter()
        .map(|b| !b as usize)
        .sum()
}

fn part2(brick_stack: &BrickStack) -> usize {
    brick_stack.brick_support_counts().iter().sum()
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    footprint: Footprint,
    z: u64,
    height: usize,
}

/// The 10x10 horizontal cross-section of a `Brick` encoded for efficient intersection testing.
#[derive(Debug, Clone, Copy)]
struct Footprint(u128);

impl Footprint {
    fn intersects(&self, other: &Footprint) -> bool {
        self.0 & other.0 != 0
    }
}

/// A brick dependency graph.
///
/// `supports[i]` contains the list of indices brick `i` sits on.
#[derive(Debug, Clone, Default)]
struct BrickStack {
    supports: Vec<Vec<usize>>,
}

impl BrickStack {
    fn brick_support_counts(&self) -> Vec<usize> {
        let mut support_counts = vec![0; self.brick_count()];
        for (i, essential) in self.essential_bricks().into_iter().enumerate() {
            if essential {
                support_counts[i] = self.bricks_supported(i);
            }
        }
        support_counts
    }

    fn bricks_supported(&self, i: usize) -> usize {
        let mut destroyed_bricks = vec![false; self.brick_count()];
        destroyed_bricks[i] = true;
        for j in (i + 1)..self.brick_count() {
            if !self.supports[j].is_empty()
                && self.supports[j]
                    .iter()
                    .all(|&k| *destroyed_bricks.get(k).unwrap_or(&false))
            {
                destroyed_bricks[j] = true;
            }
        }
        destroyed_bricks.into_iter().filter(|&b| b).count() - 1
    }

    fn essential_bricks(&self) -> Vec<bool> {
        let mut essential_bricks = vec![false; self.brick_count()];
        for v in &self.supports {
            if v.len() == 1 {
                essential_bricks[v[0]] = true;
            }
        }
        essential_bricks
    }

    fn brick_count(&self) -> usize {
        self.supports.len()
    }

    fn from_bricks(mut bricks: Vec<Brick>) -> BrickStack {
        bricks.sort_unstable_by_key(|brick| brick.z);

        let mut supports = vec![vec![]; bricks.len()];
        let mut layers: Vec<Vec<(usize, Footprint)>> = vec![];
        for (i, brick) in bricks.iter().enumerate() {
            let mut base_height = 0;
            for (h, layer) in layers.iter().enumerate().rev() {
                for (j, footprint) in layer {
                    if footprint.intersects(&brick.footprint) {
                        supports[i].push(*j);
                    }
                }
                if !supports[i].is_empty() {
                    base_height = h + 1;
                    break;
                }
            }
            for height in base_height..(base_height + brick.height) {
                while layers.len() < height + 1 {
                    layers.push(vec![]);
                }
                let layer = layers.get_mut(height).unwrap();
                layer.push((i, brick.footprint));
            }
        }
        BrickStack { supports }
    }
}

mod parsing {
    use super::{Brick, Footprint};

    use anyhow::{anyhow, bail, Result};

    use aoc::iterators::AocIterators;

    impl std::str::FromStr for Brick {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (a, b) = s
                .split_once('~')
                .ok_or_else(|| anyhow!("Invalid Brick {}", s))?;
            let (x1, y1, z1) = parse_coordinates(a)?;
            let (x2, y2, z2) = parse_coordinates(b)?;
            let mut footprint = 0;
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    footprint |= 1 << (x + 10 * y);
                }
            }
            Ok(Brick {
                footprint: Footprint(footprint),
                z: z1.min(z2),
                height: z1.abs_diff(z2) as usize + 1,
            })
        }
    }

    fn parse_coordinates(s: &str) -> Result<(u64, u64, u64)> {
        let [x, y, z] = s
            .splitn(3, ',')
            .exactly_n::<3>()
            .ok_or_else(|| anyhow!("Invalid coordinates {}", s))?;
        let (x, y, z) = (x.parse()?, y.parse()?, z.parse()?);
        if x > 9 || y > 9 {
            bail!("Coordinates out of range: {:?}", (x, y, z));
        }

        Ok((x, y, z))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        1,0,1~1,2,1\n\
        0,0,2~2,0,2\n\
        0,2,3~2,2,3\n\
        0,0,4~0,2,4\n\
        2,0,5~2,2,5\n\
        0,1,6~2,1,6\n\
        1,1,8~1,1,9";

    #[test]
    fn essential_bricks() {
        let bricks: Vec<Brick> = parse_fields(TEST_DATA, '\n').unwrap();
        let brick_stack = BrickStack::from_bricks(bricks);
        let expected: Vec<_> = vec![true, false, false, false, false, true, false];

        assert_eq!(brick_stack.essential_bricks(), expected);
    }

    #[test]
    fn brick_support_counts() {
        let bricks: Vec<Brick> = parse_fields(TEST_DATA, '\n').unwrap();
        let brick_stack = BrickStack::from_bricks(bricks);
        let expected = [6, 0, 0, 0, 0, 1, 0];

        assert_eq!(brick_stack.brick_support_counts(), expected);
    }
}
