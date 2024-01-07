use std::ops::RangeInclusive;

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use ndarray::{array, Array1, Array2};
use ndarray_linalg::Solve;

use aoc::iterators::iter_pairs;
use aoc::utils::{get_input, parse_fields};

static MAX_ERROR: f64 = 0.4;

fn main() -> Result<()> {
    let input = get_input()?;
    let hailstones: Vec<Hailstone<i64>> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&hailstones));
    println!("Part 2: {}", part2(&hailstones)?);

    Ok(())
}

fn part1(hailstones: &[Hailstone<i64>]) -> usize {
    intersections_in_range(hailstones, 200000000000000.0..=400000000000000.0)
}

fn part2(hailstones: &[Hailstone<i64>]) -> Result<i64> {
    let bullet = magic_bullet(hailstones).ok_or_else(|| anyhow!("No magic bullet exists."))?;
    Ok(bullet.position[0] + bullet.position[1] + bullet.position[2])
}

fn intersections_in_range(hailstones: &[Hailstone<i64>], range: RangeInclusive<f64>) -> usize {
    iter_pairs(hailstones)
        .map(|(a, b)| Hailstone::from(a).intersection(&b.into()))
        .filter(|(x, y, ta, tb)| ta >= &0.0 && tb >= &0.0 && range.contains(x) && range.contains(y))
        .count()
}

fn magic_bullet(hailstones: &[Hailstone<i64>]) -> Option<Hailstone<i64>> {
    let hailstones: Vec<Hailstone<f64>> = hailstones.iter().map(|h| h.into()).collect();
    let bullet = hailstones
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b, c)| three_stone_killer(a, b, c))
        .next()?;
    Hailstone::try_from(&bullet).ok()
}

/// Return the unique hailstone that will hit `a`, `b` and `c`.
///
/// : For all i
/// xb - xi = (vxb - vxi) ti
/// yb - yi = (vyb - vyi) ti
/// (xb - xi) / (vxb - vxi) = (yb - yi) / (vyb - vyi)
/// (xb - xi) (vyb - vyi) = (yb - yi) (vxb - vxi)
/// xb * vyb - yb * vxb = xb * vyi + xi * vyb - xi * vyi - yb * vxi + yi * vxi - yi * vxb
/// : So, for all i, j
/// xb * vyi + xi * vyb - xi * vyi - yb * vxi + yi * vxi - yi * vxb = xb * vyj + xj * vyb - xj * vyj - yb * vxj + yj * vxj - yj * vxb
/// (vyi - vyj) xb + (vxj - vxi) yb + (yj - yi) vxb + (xi - xj) vyb = xi * vyi - xj * vyj - yi * vxi + yj * vxj
/// : This gets us 2 independent equations, and then we can get two more for each other coordinate pairing.
fn three_stone_killer(
    h0: &Hailstone<f64>,
    h1: &Hailstone<f64>,
    h2: &Hailstone<f64>,
) -> Option<Hailstone<f64>> {
    let [x0, y0, z0] = h0.position;
    let [vx0, vy0, vz0] = h0.velocity;
    let [x1, y1, z1] = h1.position;
    let [vx1, vy1, vz1] = h1.velocity;
    let [x2, y2, z2] = h2.position;
    let [vx2, vy2, vz2] = h2.velocity;
    let a: Array2<f64> = array![
        [vy1 - vy0, vx0 - vx1, 0.0, y0 - y1, x1 - x0, 0.0],
        [vy2 - vy0, vx0 - vx2, 0.0, y0 - y2, x2 - x0, 0.0],
        [0.0, vz1 - vz0, vy0 - vy1, 0.0, z0 - z1, y1 - y0],
        [0.0, vz2 - vz0, vy0 - vy2, 0.0, z0 - z2, y2 - y0],
        [vz0 - vz1, 0.0, vx1 - vx0, z1 - z0, 0.0, x0 - x1],
        [vz0 - vz2, 0.0, vx2 - vx0, z2 - z0, 0.0, x0 - x2],
    ];
    let b: Array1<f64> = array![
        x1 * vy1 + vx0 * y0 - x0 * vy0 - vx1 * y1,
        x2 * vy2 + vx0 * y0 - x0 * vy0 - vx2 * y2,
        y1 * vz1 + vy0 * z0 - y0 * vz0 - vy1 * z1,
        y2 * vz2 + vy0 * z0 - y0 * vz0 - vy2 * z2,
        z1 * vx1 + vz0 * x0 - z0 * vx0 - vz1 * x1,
        z2 * vx2 + vz0 * x0 - z0 * vx0 - vz2 * x2,
    ];
    let x = a.solve_into(b).ok()?;
    Some(Hailstone {
        position: [x[0], x[1], x[2]],
        velocity: [x[3], x[4], x[5]],
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hailstone<T> {
    position: [T; 3],
    velocity: [T; 3],
}

impl Hailstone<f64> {
    fn intersection(&self, b: &Hailstone<f64>) -> (f64, f64, f64, f64) {
        let m_self = self.velocity[1] / self.velocity[0];
        let m_other = b.velocity[1] / b.velocity[0];
        let x = (b.position[1] - self.position[1] + m_self * self.position[0]
            - m_other * b.position[0])
            / (m_self - m_other);
        let y = self.position[1] + m_self * (x - self.position[0]);
        let t_self = (x - self.position[0]) / self.velocity[0];
        let t_other = (x - b.position[0]) / b.velocity[0];
        (x, y, t_self, t_other)
    }
}

impl From<&Hailstone<i64>> for Hailstone<f64> {
    fn from(value: &Hailstone<i64>) -> Self {
        Hailstone {
            position: [
                value.position[0] as f64,
                value.position[1] as f64,
                value.position[2] as f64,
            ],
            velocity: [
                value.velocity[0] as f64,
                value.velocity[1] as f64,
                value.velocity[2] as f64,
            ],
        }
    }
}

impl TryFrom<&Hailstone<f64>> for Hailstone<i64> {
    type Error = anyhow::Error;

    fn try_from(value: &Hailstone<f64>) -> Result<Self, Self::Error> {
        Ok(Hailstone {
            position: [
                round_to_i64(value.position[0])?,
                round_to_i64(value.position[1])?,
                round_to_i64(value.position[2])?,
            ],
            velocity: [
                round_to_i64(value.velocity[0])?,
                round_to_i64(value.velocity[1])?,
                round_to_i64(value.velocity[2])?,
            ],
        })
    }
}

fn round_to_i64(x: f64) -> Result<i64> {
    let result = x.round();
    if (x - result).abs() > MAX_ERROR {
        bail!("Error too large");
    }
    Ok(result as i64)
}

mod parsing {
    use super::Hailstone;

    use std::str::FromStr;

    use anyhow::anyhow;

    use aoc::iterators::AocIterators;

    impl<T> FromStr for Hailstone<T>
    where
        T: FromStr,
        <T as FromStr>::Err: 'static + Send + Sync + std::error::Error,
    {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (x, y, z, vx, vy, vz) =
                split_parts(s).ok_or_else(|| anyhow!("Invalid hailstone {}", s))?;
            Ok(Hailstone::<T> {
                position: [x.trim().parse()?, y.trim().parse()?, z.trim().parse()?],
                velocity: [vx.trim().parse()?, vy.trim().parse()?, vz.trim().parse()?],
            })
        }
    }

    fn split_parts(s: &str) -> Option<(&str, &str, &str, &str, &str, &str)> {
        let (p, v) = s.split_once(" @ ")?;
        let [x, y, z] = p.split(", ").exactly_n::<3>()?;
        let [vx, vy, vz] = v.split(", ").exactly_n::<3>()?;
        Some((x, y, z, vx, vy, vz))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        19, 13, 30 @ -2,  1, -2\n\
        18, 19, 22 @ -1, -1, -2\n\
        20, 25, 34 @ -2, -2, -4\n\
        12, 31, 28 @ -1, -2, -1\n\
        20, 19, 15 @  1, -5, -3";

    #[test]
    fn intersections() {
        let hailstones: Vec<Hailstone<i64>> = parse_fields(TEST_DATA, '\n').unwrap();
        let result = intersections_in_range(&hailstones, 7.0..=27.0);

        assert_eq!(result, 2);
    }

    #[test]
    fn find_magic_bullet() {
        let hailstones: Vec<Hailstone<i64>> = parse_fields(TEST_DATA, '\n').unwrap();
        let result = magic_bullet(&hailstones);
        let expected = Some(Hailstone {
            position: [24, 13, 10],
            velocity: [-3, 1, 2],
        });

        assert_eq!(result, expected);
    }
}
