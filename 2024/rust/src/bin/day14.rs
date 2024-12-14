use anyhow::{anyhow, Result};

use aoc::planar::Point;

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let robots: Vec<Robot> = aoc::utils::parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&robots));
    println!("Part 2: {}", part2(&robots)?);

    Ok(())
}

fn part1(robots: &[Robot]) -> usize {
    quadrant_counts::<101, 103>(robots, 100)
        .into_iter()
        .product()
}

/// Find the first time a tree appears.
///
/// x coordinates repeat every 101 steps, and y coordinates every 103 which gives 10403 distinct
/// arrangements of robots. We will search this space in order of increasing total variance.
///
/// In principle we need to search every possible arrangment, but a pattern using a meaningful
/// fraction of robots should have lower variance, so searching is this order is efficient.
/// As it happens, for the real puzzle the result seems to always be the first value.
fn part2(robots: &[Robot]) -> Result<i64> {
    // Variances in x coordinates by time for the first 101 steps
    let x_variances: [i64; 101] = {
        let positions: Vec<i64> = robots.iter().map(|robot| robot.position.x).collect();
        let velocities: Vec<i64> = robots.iter().map(|robot| robot.velocity.x).collect();
        variances_by_time::<101>(&positions, &velocities)
    };
    // Variances in y coordinates by time for the first 103 steps
    let y_variances: [i64; 103] = {
        let positions: Vec<i64> = robots.iter().map(|robot| robot.position.y).collect();
        let velocities: Vec<i64> = robots.iter().map(|robot| robot.velocity.y).collect();
        variances_by_time::<103>(&positions, &velocities)
    };
    // Times to check, sorted by sum of x and y variances
    let sorted_times = {
        let mut times = Vec::with_capacity(101 * 103);
        for (tx, vx) in x_variances.iter().enumerate() {
            for (ty, vy) in y_variances.iter().enumerate() {
                times.push((vx + vy, min_congruence(tx as i64, ty as i64)));
            }
        }
        times.sort_unstable();
        times.into_iter().map(|(_, t)| t)
    };

    for time in sorted_times {
        if has_picture(robots, time) {
            return Ok(time);
        }
    }
    Err(anyhow!("Failed to find tree"))
}

fn quadrant_counts<const W: i64, const H: i64>(robots: &[Robot], time: i64) -> [usize; 4] {
    let mut counts = [0; 4];
    robots
        .iter()
        .filter_map(|robot| robot.step(time).quadrant::<W, H>())
        .for_each(|quadrant| counts[quadrant] += 1);
    counts
}

fn variances_by_time<const N: usize>(positions: &[i64], velocities: &[i64]) -> [i64; N] {
    let mut result = [0; N];
    for (t, entry) in result.iter_mut().enumerate() {
        let values: Vec<i64> = positions
            .iter()
            .zip(velocities)
            .map(|(position, velocity)| (position + t as i64 * velocity).rem_euclid(N as i64))
            .collect();
        let mean = values.iter().sum::<i64>() / values.len() as i64;
        *entry = values.iter().map(|v| (v - mean) * (v - mean)).sum()
    }
    result
}

#[cfg(feature = "verbose")]
fn diagram<const W: i64, const H: i64>(robots: &[Robot]) -> String {
    let mut diagram: Vec<u8> = vec![b'.'; ((W + 1) * H - 1) as usize];
    for y in 1..H {
        diagram[((W + 1) * y - 1) as usize] = b'\n';
    }
    for robot in robots {
        diagram[((W + 1) * robot.position.y.rem_euclid(H) + robot.position.x.rem_euclid(W))
            as usize] = b'*';
    }
    unsafe { String::from_utf8_unchecked(diagram) }
}

/// Calculate the smallest t such that t = tx mod 101 and t = ty mod 103.
fn min_congruence(tx: i64, ty: i64) -> i64 {
    // 51 is the multiplicative inverse of 101 mod 103
    101 * (51 * (ty - tx)).rem_euclid(103) + tx
}

/// Decide if the robots are a drawing
fn has_picture(robots: &[Robot], time: i64) -> bool {
    let robots: Vec<_> = robots.iter().map(|robot| robot.step(time)).collect();
    #[cfg(feature = "verbose")]
    println!("{}", diagram::<101, 103>(&robots));
    let mut grid = vec![vec![false; 101]; 103];
    for robot in &robots {
        grid[robot.position.y.rem_euclid(103) as usize]
            [robot.position.x.rem_euclid(101) as usize] = true;
    }
    // In practice, we don't know what a tree looks like, so settle for any improbably large
    // contiguous region that's probably a picture. This should prevent most false positives.
    for (y, row) in grid.iter().enumerate() {
        for (x, &b) in row.iter().enumerate() {
            if !b {
                continue;
            }
            let mut area = 0;
            let mut seen = vec![vec![false; 101]; 103];
            let mut to_visit = vec![(x, y)];
            while let Some((x, y)) = to_visit.pop() {
                if seen[y][x] {
                    continue;
                }
                seen[y][x] = true;
                area += 1;
                if area >= 20 {
                    return true;
                }
                for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                    if (0..101).contains(&nx) && (0..103).contains(&ny) {
                        to_visit.push((nx as usize, ny as usize));
                    }
                }
            }
        }
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn step(&self, time: i64) -> Robot {
        let mut robot = *self;
        robot.position += self.velocity * time;
        robot
    }

    fn quadrant<const W: i64, const H: i64>(&self) -> Option<usize> {
        use std::cmp::Ordering;

        let (x, y) = (self.position.x.rem_euclid(W), self.position.y.rem_euclid(H));
        // Wrong for even W or H, but we don't care.
        match (x.cmp(&(W / 2)), y.cmp(&(H / 2))) {
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Greater, Ordering::Less) => Some(1),
            (Ordering::Less, Ordering::Greater) => Some(2),
            (Ordering::Greater, Ordering::Greater) => Some(3),
            _ => None,
        }
    }
}

impl std::str::FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        fn split_parts(s: &str) -> Option<(&str, &str, &str, &str)> {
            let (position_part, velocity_part) = s.split_once(" v=")?;
            let (x, y) = position_part.strip_prefix("p=")?.split_once(',')?;
            let (vx, vy) = velocity_part.split_once(',')?;
            Some((x, y, vx, vy))
        }
        let (x, y, vx, vy) = split_parts(s).ok_or_else(|| anyhow!("Failed to split {}", s))?;
        Ok(Robot {
            position: Point {
                x: x.parse()?,
                y: y.parse()?,
            },
            velocity: Point {
                x: vx.parse()?,
                y: vy.parse()?,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        p=0,4 v=3,-3\n\
        p=6,3 v=-1,-3\n\
        p=10,3 v=-1,2\n\
        p=2,0 v=2,-1\n\
        p=0,0 v=1,3\n\
        p=3,0 v=-2,-2\n\
        p=7,6 v=-1,-3\n\
        p=3,0 v=-1,-2\n\
        p=9,3 v=2,3\n\
        p=7,3 v=-1,2\n\
        p=2,4 v=2,-3\n\
        p=9,5 v=-3,-3";

    #[test]
    fn quadrants() {
        let robots: Vec<Robot> = aoc::utils::parse_fields(TEST_DATA, '\n').unwrap();
        let result = quadrant_counts::<11, 7>(&robots, 100);
        assert_eq!(result, [1, 3, 4, 1]);
    }

    #[cfg(feature = "verbose")]
    #[test]
    fn draw_diagram() {
        let robots: Vec<Robot> = aoc::utils::parse_fields(TEST_DATA, '\n').unwrap();
        let result = diagram::<11, 7>(&robots);
        let expected = "\
            *.**.......\n\
            ...........\n\
            ...........\n\
            ......**.**\n\
            *.*........\n\
            .........*.\n\
            .......*...";
        assert_eq!(result, expected);
    }
}
