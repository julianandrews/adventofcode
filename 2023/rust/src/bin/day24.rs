use std::ops::RangeInclusive;

use anyhow::{anyhow, Result};
use z3::ast::{Ast, Int, Real};

use aoc::iterators::iter_pairs;
use aoc::utils::{get_input, parse_fields};

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
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let new_value = |s: &str| -> Real { Real::new_const(&ctx, s) };
    let p = [new_value("x"), new_value("y"), new_value("z")];
    let v = [new_value("vx"), new_value("vy"), new_value("vz")];

    for (j, hailstone) in hailstones.iter().enumerate() {
        let t = new_value(&format!("t_{}", j));
        for i in 0..3 {
            let p_i = Real::from_int(&Int::from_i64(&ctx, hailstone.position[i]));
            let v_i = Real::from_int(&Int::from_i64(&ctx, hailstone.velocity[i]));
            solver.assert(&(p_i + (v_i - &v[i]) * &t)._eq(&p[i]));
        }
    }

    let model = match solver.check() {
        z3::SatResult::Sat => solver.get_model()?,
        _ => return None,
    };
    let get_value = |value: &Real| -> Option<i64> {
        match model.eval(value, true)?.as_real()? {
            (num, 1) => Some(num),
            _ => None,
        }
    };
    let position = [get_value(&p[0])?, get_value(&p[1])?, get_value(&p[2])?];
    let velocity = [get_value(&v[0])?, get_value(&v[1])?, get_value(&v[2])?];
    Some(Hailstone { position, velocity })
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hailstone<T> {
    position: [T; 3],
    velocity: [T; 3],
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