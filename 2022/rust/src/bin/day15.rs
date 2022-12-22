use std::cmp::Ordering;

use anyhow::{anyhow, bail, Result};

use aoc::planar::Point;
use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let sensors: Vec<_> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&sensors));
    println!("Part 2: {}", part2(&sensors)?);

    Ok(())
}

fn part1(sensors: &[Sensor]) -> usize {
    count_row(sensors, 2000000)
}

fn part2(sensors: &[Sensor]) -> Result<i64> {
    let p = match find_beacon(sensors, 4000000) {
        Some(p) => p,
        None => bail!("Failed to find beacon!"),
    };
    Ok(p.x * 4000000 + p.y)
}

fn count_row(sensors: &[Sensor], row: i64) -> usize {
    let intervals = row_intervals(sensors, row);
    let covered_count: i64 = intervals.into_iter().map(|(a, b)| b - a + 1).sum();
    let mut row_beacons: Vec<_> = sensors
        .iter()
        .filter(|s| s.beacon.y == row)
        .map(|s| s.beacon.x)
        .collect();
    row_beacons.sort_unstable();
    row_beacons.dedup();
    covered_count as usize - row_beacons.len()
}

fn row_intervals(sensors: &[Sensor], row: i64) -> Vec<(i64, i64)> {
    let mut merged = vec![];
    let mut intervals: Vec<_> = sensors.iter().map(|s| s.row_coverage(row)).collect();
    intervals.sort_unstable();
    let (mut a, mut b) = match intervals.first() {
        Some(interval) => interval,
        None => return merged,
    };
    for (c, d) in intervals.into_iter().skip(1) {
        if c <= b {
            a = a.min(c);
            b = b.max(d);
        } else {
            merged.push((a, b));
            (a, b) = (c, d);
        }
    }
    merged.push((a, b));
    merged
}

fn find_beacon(sensors: &[Sensor], range: i64) -> Option<Point> {
    for sensor in sensors {
        let mut p = sensor.sensor;
        p.y += sensor.range;
        p.x -= 1;
        for _ in 0..4 * (sensor.range + 1) {
            step_perimeter(&mut p, &sensor.sensor);
            if is_beacon(p, sensors, range) {
                return Some(p);
            }
        }
    }
    None
}

/// Advances a point clockwise around a center maintaining the same manhattan distance.
fn step_perimeter(p: &mut Point, center: &Point) {
    let x_cmp = p.x.cmp(&center.x);
    let y_cmp = p.y.cmp(&center.y);
    // Use signum to choose which way to turn at the corners.
    match x_cmp {
        Ordering::Less => p.y += 1,
        Ordering::Equal => p.y -= (p.y - center.y).signum(),
        Ordering::Greater => p.y -= 1,
    }
    match y_cmp {
        Ordering::Less => p.x -= 1,
        Ordering::Equal => p.x -= (p.x - center.x).signum(),
        Ordering::Greater => p.x += 1,
    }
}

fn is_beacon(p: Point, sensors: &[Sensor], range: i64) -> bool {
    if p.x < 0 || p.y < 0 || p.x > range || p.y > range {
        return false;
    }
    sensors
        .iter()
        .all(|s| s.sensor.manhattan_distance(&p) > s.range)
}

#[derive(Debug, Clone)]
struct Sensor {
    sensor: Point,
    beacon: Point,
    range: i64,
}

impl Sensor {
    fn row_coverage(&self, row: i64) -> (i64, i64) {
        let d = (self.range - (self.sensor.y - row).abs()).max(0);
        let center = self.sensor.x;
        (center - d, center + d)
    }
}

impl std::str::FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("Sensor at ")
            .ok_or_else(|| anyhow!("Invalid sensor: {}", s))?;
        let (sensor_part, beacon_part) = s
            .split_once(": closest beacon is at ")
            .ok_or_else(|| anyhow!("Invalid sensor: {}", s))?;
        let sensor = parse_point(sensor_part)?;
        let beacon = parse_point(beacon_part)?;
        let range = sensor.manhattan_distance(&beacon);
        Ok(Self {
            sensor,
            beacon,
            range,
        })
    }
}

fn parse_point(s: &str) -> Result<Point> {
    let (x_part, y_part) = s
        .split_once(", ")
        .ok_or_else(|| anyhow!("Invalid point: {}", s))?;
    let x_part = x_part
        .strip_prefix("x=")
        .ok_or_else(|| anyhow!("Invalid point: {}", s))?;
    let y_part = y_part
        .strip_prefix("y=")
        .ok_or_else(|| anyhow!("Invalid point: {}", s))?;
    Ok(Point {
        x: x_part.parse()?,
        y: y_part.parse()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
        Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
        Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
        Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
        Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
        Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
        Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
        Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
        Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
        Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
        Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
        Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
        Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
        Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn count_row_10() {
        let sensors = parse_fields(TEST_DATA, '\n').unwrap();
        assert_eq!(count_row(&sensors, 10), 26);
    }

    #[test]
    fn find_beacon_20() {
        let sensors = parse_fields(TEST_DATA, '\n').unwrap();
        let expected = Point::new(14, 11);
        assert_eq!(find_beacon(&sensors, 20), Some(expected));
    }
}
