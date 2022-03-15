use regex::Regex;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let target_area = input.trim().parse()?;

    println!("Part 1: {}", part1(&target_area)?);
    println!("Part 2: {}", part2(&target_area));
    Ok(())
}

fn part1(target_area: &TargetArea) -> Result<i64> {
    // The projectile will go up, and return to y = 0 with velocity -v_y.
    // If v_y is greater in magnitude than the bottom of the target_area, then
    // it will immediately overshoot.
    for v_y in (*target_area.y_range.start()..=-*target_area.y_range.start()).rev() {
        for v_x in 1..=*target_area.x_range.end() {
            let trajectory = Trajectory { v_x, v_y };
            if trajectory.intersects(&target_area) {
                return Ok(trajectory.max_height());
            }
        }
    }
    Err(Box::new(AOCError::new(
        "Failed to find trajectory that intersects target area",
    )))
}

fn part2(target_area: &TargetArea) -> u64 {
    let mut count = 0;
    for v_y in *target_area.y_range.start()..=-*target_area.y_range.start() {
        for v_x in 1..=*target_area.x_range.end() {
            let trajectory = Trajectory { v_x, v_y };
            if trajectory.intersects(&target_area) {
                count += 1;
            }
        }
    }
    count
}

#[derive(Debug)]
struct Trajectory {
    v_x: i64,
    v_y: i64,
}

impl Trajectory {
    fn x(&self, t: i64) -> i64 {
        let v = self.v_x.abs();
        let x = if t <= v {
            t * (2 * v - t + 1) / 2
        } else {
            v * (v + 1) / 2
        };
        x * v.signum()
    }

    fn y(&self, t: i64) -> i64 {
        t * (2 * self.v_y - t + 1) / 2
    }

    fn max_height(&self) -> i64 {
        self.v_y * (self.v_y + 1) / 2
    }

    fn intersects(&self, target_area: &TargetArea) -> bool {
        for t in 0.. {
            let x = self.x(t);
            let y = self.y(t);
            if target_area.contains(x, y) {
                return true;
            } else if y < *target_area.y_range.start() {
                return false;
            }
        }
        unreachable!();
    }
}

#[derive(Debug, Clone)]
struct TargetArea {
    x_range: std::ops::RangeInclusive<i64>,
    y_range: std::ops::RangeInclusive<i64>,
}

impl TargetArea {
    fn contains(&self, x: i64, y: i64) -> bool {
        self.x_range.contains(&x) && self.y_range.contains(&y)
    }
}

impl std::str::FromStr for TargetArea {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"^target area: x=(-?\d*)..(-?\d*), y=(-?\d*)..(-?\d*)$").unwrap();
        let caps = re
            .captures(s)
            .ok_or(AOCError::new("Failed to parse TargetArea"))?;
        let x_range = caps[1].parse()?..=caps[2].parse()?;
        let y_range = caps[3].parse()?..=caps[4].parse()?;
        Ok(TargetArea { x_range, y_range })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TARGET_AREA: TargetArea = TargetArea {
        x_range: 20..=30,
        y_range: -10..=-5,
    };

    #[test]
    fn test_intersects_1() {
        let trajectory = Trajectory { v_x: 7, v_y: 2 };
        assert!(trajectory.intersects(&TARGET_AREA));
    }

    #[test]
    fn test_intersects_2() {
        let trajectory = Trajectory { v_x: 6, v_y: 3 };
        assert!(trajectory.intersects(&TARGET_AREA));
    }

    #[test]
    fn test_intersects_3() {
        let trajectory = Trajectory { v_x: 9, v_y: 0 };
        assert!(trajectory.intersects(&TARGET_AREA));
    }

    #[test]
    fn test_intersects_4() {
        let trajectory = Trajectory { v_x: 17, v_y: -4 };
        assert!(!trajectory.intersects(&TARGET_AREA));
    }

    #[test]
    fn test_intersects_5() {
        let trajectory = Trajectory { v_x: 6, v_y: 9 };
        assert!(trajectory.intersects(&TARGET_AREA));
    }

    #[test]
    fn test_intersects_6() {
        let trajectory = Trajectory { v_x: 6, v_y: 0 };
        assert!(trajectory.intersects(&TARGET_AREA));
    }

    #[test]
    fn test_intersects_7() {
        let trajectory = Trajectory { v_x: 7, v_y: -1 };
        assert!(trajectory.intersects(&TARGET_AREA));
    }

    #[test]
    fn test_part1() {
        let result = part1(&TARGET_AREA).unwrap();
        assert_eq!(result, 45);
    }

    #[test]
    fn test_part2() {
        let result = part2(&TARGET_AREA);
        assert_eq!(result, 112);
    }
}
