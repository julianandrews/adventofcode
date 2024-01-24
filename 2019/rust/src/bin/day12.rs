extern crate num_integer;

use aoc::aoc_error::AOCError;
use aoc::iterators::cycle_detect;
use aoc::point::{Axis3D, Point3D};
use std::iter;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
type Point = Point3D<i64>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    position: Point,
    velocity: Point,
}

impl Moon {
    fn new(position: Point) -> Moon {
        Moon {
            position,
            velocity: Point { x: 0, y: 0, z: 0 },
        }
    }

    fn update_velocity(&mut self, other: &Moon) {
        for axis in Axis3D::iter() {
            self.velocity[axis] += -(self.position[axis].cmp(&other.position[axis]) as i64);
        }
    }

    fn update_position(&mut self) {
        for axis in Axis3D::iter() {
            self.position[axis] += self.velocity[axis];
        }
    }

    fn energy(&self) -> i64 {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs())
            * (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs())
    }
}

impl FromStr for Moon {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = &s[1..s.len() - 1]
            .splitn(3, ", ")
            .map(|s| s[2..].parse())
            .collect::<std::result::Result<Vec<i64>, _>>()?;
        if parts.len() != 3 {
            return Err(AOCError::new("Failed to parse moon."))?;
        }

        Ok(Moon::new(Point {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        }))
    }
}

#[derive(Debug, Clone)]
struct PlanetarySystem {
    moons: Vec<Moon>,
}

impl FromStr for PlanetarySystem {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(PlanetarySystem {
            moons: s.lines().map(&str::parse).collect::<Result<_>>()?,
        })
    }
}

impl PlanetarySystem {
    fn step(&mut self) {
        // iterate over every pair of moons and update velocity.
        for i in 0..self.moons.len() {
            let (a, b) = self.moons.split_at_mut(i);
            let moon_b = &mut b[0];
            for moon_a in a {
                moon_a.update_velocity(moon_b);
                moon_b.update_velocity(moon_a);
            }
        }
        for moon in &mut self.moons {
            moon.update_position();
        }
    }

    fn energy(&self) -> i64 {
        self.moons.iter().map(Moon::energy).sum()
    }

    fn states(&self) -> impl Iterator<Item = PlanetarySystem> {
        iter::successors(Some(self.clone()), |system| {
            let mut system = system.clone();
            system.step();
            Some(system)
        })
    }
}

fn cycle_length(system: &PlanetarySystem) -> Result<usize> {
    let cycles = Axis3D::iter()
        .filter_map(|axis| {
            cycle_detect(system.states().map(|system| {
                system
                    .moons
                    .iter()
                    .map(|m| (m.position[axis], m.velocity[axis]))
                    .collect::<Vec<_>>()
            }))
        })
        .collect::<Vec<_>>();
    if cycles.len() != 3 {
        Err(AOCError::new("Expected to find 3 cycles"))?;
    }

    let cycle_start = cycles.iter().map(|c| c.start).max().unwrap();
    let cycle_length = cycles
        .iter()
        .fold(1, |lcm, cycle| num_integer::lcm(lcm, cycle.length));

    Ok(cycle_start + cycle_length)
}

fn part1(mut system: PlanetarySystem) -> i64 {
    for _ in 0..1000 {
        system.step();
    }

    system.energy()
}

fn part2(system: &PlanetarySystem) -> Result<usize> {
    cycle_length(system)
}

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let system: PlanetarySystem = input.parse()?;

    println!("Part 1: {}", part1(system.clone()));
    println!("Part 2: {}", part2(&system)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn energy_test_1() {
        let input = "<x=-1, y=0, z=2>\n\
                     <x=2, y=-10, z=-7>\n\
                     <x=4, y=-8, z=8>\n\
                     <x=3, y=5, z=-1>";
        let mut system: PlanetarySystem = input.parse().unwrap();
        for _ in 0..10 {
            system.step();
        }
        assert_eq!(system.energy(), 179);
    }

    #[test]
    fn energy_test_2() {
        let input = "<x=-8, y=-10, z=0>\n\
                     <x=5, y=5, z=10>\n\
                     <x=2, y=-7, z=3>\n\
                     <x=9, y=-8, z=-3>";
        let mut system: PlanetarySystem = input.parse().unwrap();
        for _ in 0..100 {
            system.step();
        }
        assert_eq!(system.energy(), 1940);
    }

    #[test]
    fn cycle_test_1() {
        let input = "<x=-1, y=0, z=2>\n\
                     <x=2, y=-10, z=-7>\n\
                     <x=4, y=-8, z=8>\n\
                     <x=3, y=5, z=-1>";
        let system: PlanetarySystem = input.parse().unwrap();
        assert_eq!(cycle_length(&system).unwrap(), 2772);
    }

    #[test]
    fn cycle_test_2() {
        let input = "<x=-8, y=-10, z=0>\n\
                     <x=5, y=5, z=10>\n\
                     <x=2, y=-7, z=3>\n\
                     <x=9, y=-8, z=-3>";
        let system: PlanetarySystem = input.parse().unwrap();
        assert_eq!(cycle_length(&system).unwrap(), 4686774924);
    }
}
