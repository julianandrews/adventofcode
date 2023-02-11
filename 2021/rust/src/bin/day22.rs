use std::convert::TryInto;
use std::ops::Range;

use anyhow::{anyhow, bail, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let steps: Vec<Step> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&steps));
    println!("Part 2: {}", part2(&steps));

    Ok(())
}

fn part1(steps: &[Step]) -> i64 {
    let steps = small_cube_steps(steps);
    activated_volume(&steps)
}

fn part2(steps: &[Step]) -> i64 {
    activated_volume(&steps)
}

fn small_cube_steps(steps: &[Step]) -> Vec<Step> {
    steps
        .iter()
        .filter(|step| step.is_small())
        .cloned()
        .collect()
}

fn activated_volume(steps: &[Step]) -> i64 {
    // Step backwards from the last instruction.
    // If the action is "on", then all points we've never seen before are new active volume.
    // Either way, add the cuboid to the set of seen cuboids - the points in this cuboid are
    // either already counted, or have just been turned off.
    let mut volume = 0;
    let mut cuboid_set = CuboidSet::default();
    for step in steps.iter().rev() {
        let new_volume = cuboid_set.add(step.cuboid.clone());
        if matches!(step.action, Action::On) {
            volume += new_volume;
        }
    }

    volume
}

#[derive(Debug, Clone)]
struct Step {
    action: Action,
    cuboid: Cuboid,
}

impl Step {
    fn is_small(&self) -> bool {
        !self
            .cuboid
            .ranges
            .iter()
            .flat_map(|r| [r.start, r.end - 1])
            .any(|c| c.abs() > 50)
    }
}

#[derive(Debug, Clone)]
enum Action {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Cuboid {
    ranges: [Range<i64>; 3],
}

impl Cuboid {
    fn intersection(&self, other: &Cuboid) -> Cuboid {
        let pairs = self.ranges.iter().zip(&other.ranges);
        let ranges: Vec<_> = pairs
            .map(|(r1, r2)| {
                let start = r1.start.max(r2.start);
                let end = r1.end.min(r2.end);
                start..end.max(start)
            })
            .collect();
        let ranges = ranges.try_into().unwrap();
        Cuboid { ranges }
    }

    /// Split self and other into cuboids. Returns (self_pieces, interesection, other_pieces)
    fn disjoint_pieces(&self, other: &Cuboid) -> Option<(Vec<Cuboid>, Cuboid, Vec<Cuboid>)> {
        let intersection = self.intersection(other);
        if intersection.is_empty() {
            return None;
        }
        let self_pieces = self.pieces(&intersection);
        let other_pieces = other.pieces(&intersection);
        Some((self_pieces, intersection, other_pieces))
    }

    /// Return the cuboid pieces you get from removing `other` from self assuming overlap.
    fn pieces(&self, other: &Cuboid) -> Vec<Cuboid> {
        // TODO: Consider optimizing this to mimimize the number of non-empty cuboids.
        let bottom = [
            self.ranges[0].clone(),
            self.ranges[1].clone(),
            self.ranges[2].start..other.ranges[2].start,
        ];
        let top = [
            self.ranges[0].clone(),
            self.ranges[1].clone(),
            other.ranges[2].end..self.ranges[2].end,
        ];
        let left = [
            self.ranges[0].start..other.ranges[0].start,
            self.ranges[1].clone(),
            other.ranges[2].clone(),
        ];
        let right = [
            other.ranges[0].end..self.ranges[0].end,
            self.ranges[1].clone(),
            other.ranges[2].clone(),
        ];
        let back = [
            other.ranges[0].clone(),
            self.ranges[1].start..other.ranges[1].start,
            other.ranges[2].clone(),
        ];
        let front = [
            other.ranges[0].clone(),
            other.ranges[1].end..self.ranges[1].end,
            other.ranges[2].clone(),
        ];
        vec![bottom, top, left, right, back, front]
            .into_iter()
            .filter_map(Cuboid::from_ranges)
            .collect()
    }

    fn is_empty(&self) -> bool {
        self.ranges.iter().any(|range| range.is_empty())
    }

    fn volume(&self) -> i64 {
        self.ranges.iter().map(|r| r.end - r.start).product()
    }

    fn from_ranges(ranges: [Range<i64>; 3]) -> Option<Cuboid> {
        if ranges.iter().all(|range| range.end >= range.start) {
            Some(Cuboid { ranges })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Default)]
struct CuboidSet {
    // TODO: Consider using a set of some sort
    cuboids: Vec<Cuboid>,
}

impl CuboidSet {
    /// Adds the cuboid to the set and return the newly added volume
    fn add(&mut self, cuboid: Cuboid) -> i64 {
        let mut to_process = vec![cuboid];
        let mut added_volume = 0;
        while let Some(cuboid) = to_process.pop() {
            let mut found_intersection = false;
            for (i, other) in self.cuboids.iter().enumerate() {
                if let Some((new_pieces, intersection, old_pieces)) = cuboid.disjoint_pieces(other)
                {
                    found_intersection = true;
                    self.cuboids.remove(i);
                    self.cuboids.extend(old_pieces);
                    self.cuboids.push(intersection);
                    to_process.extend(new_pieces);
                    break;
                }
            }
            if !found_intersection {
                added_volume += cuboid.volume();
                self.cuboids.push(cuboid);
            }
        }
        added_volume
    }
}

impl std::str::FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action_part, cuboid_part) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Failed to parse step: {}", s))?;
        let action = match action_part {
            "on" => Action::On,
            "off" => Action::Off,
            _ => bail!("Failed to parse action: {}", action_part),
        };
        let cuboid = cuboid_part.parse()?;
        Ok(Self { action, cuboid })
    }
}

impl std::str::FromStr for Cuboid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_range(s: &str) -> Result<Range<i64>> {
            let (a, b) = s
                .split_once("..")
                .ok_or_else(|| anyhow!("Failed to parse range: {}", s))?;
            let a: i64 = a.parse()?;
            let b: i64 = b.parse::<i64>()? + 1;
            if a > b {
                bail!("Invalid range: {}", s);
            }
            Ok(a..b)
        }

        let ranges: Vec<_> = s
            .split(',')
            .zip(["x=", "y=", "z="])
            .map(|(range_str, prefix)| {
                parse_range(
                    range_str
                        .strip_prefix(prefix)
                        .ok_or_else(|| anyhow!("Failed to parse cuboid: {}", s))?,
                )
            })
            .collect::<Result<_>>()?;
        let ranges = TryInto::<[Range<i64>; 3]>::try_into(ranges)
            .map_err(|_| anyhow!("Wrong number of ranges in {}", s))?;
        Ok(Cuboid { ranges })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SIMPLE_TEST_DATA: &str = "\
        on x=10..12,y=10..12,z=10..12\n\
        on x=11..13,y=11..13,z=11..13\n\
        off x=9..11,y=9..11,z=9..11\n\
        on x=10..10,y=10..10,z=10..10";

    static LARGER_TEST_DATA: &str = "\
        on x=-20..26,y=-36..17,z=-47..7\n\
        on x=-20..33,y=-21..23,z=-26..28\n\
        on x=-22..28,y=-29..23,z=-38..16\n\
        on x=-46..7,y=-6..46,z=-50..-1\n\
        on x=-49..1,y=-3..46,z=-24..28\n\
        on x=2..47,y=-22..22,z=-23..27\n\
        on x=-27..23,y=-28..26,z=-21..29\n\
        on x=-39..5,y=-6..47,z=-3..44\n\
        on x=-30..21,y=-8..43,z=-13..34\n\
        on x=-22..26,y=-27..20,z=-29..19\n\
        off x=-48..-32,y=26..41,z=-47..-37\n\
        on x=-12..35,y=6..50,z=-50..-2\n\
        off x=-48..-32,y=-32..-16,z=-15..-5\n\
        on x=-18..26,y=-33..15,z=-7..46\n\
        off x=-40..-22,y=-38..-28,z=23..41\n\
        on x=-16..35,y=-41..10,z=-47..6\n\
        off x=-32..-23,y=11..30,z=-14..3\n\
        on x=-49..-5,y=-3..45,z=-29..18\n\
        off x=18..30,y=-20..-8,z=-3..13\n\
        on x=-41..9,y=-7..43,z=-33..15\n\
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877\n\
        on x=967..23432,y=45373..81175,z=27513..53682";

    static FULL_TEST_DATA: &str = "\
        on x=-5..47,y=-31..22,z=-19..33\n\
        on x=-44..5,y=-27..21,z=-14..35\n\
        on x=-49..-1,y=-11..42,z=-10..38\n\
        on x=-20..34,y=-40..6,z=-44..1\n\
        off x=26..39,y=40..50,z=-2..11\n\
        on x=-41..5,y=-41..6,z=-36..8\n\
        off x=-43..-33,y=-45..-28,z=7..25\n\
        on x=-33..15,y=-32..19,z=-34..11\n\
        off x=35..47,y=-46..-34,z=-11..5\n\
        on x=-14..36,y=-6..44,z=-16..29\n\
        on x=-57795..-6158,y=29564..72030,z=20435..90618\n\
        on x=36731..105352,y=-21140..28532,z=16094..90401\n\
        on x=30999..107136,y=-53464..15513,z=8553..71215\n\
        on x=13528..83982,y=-99403..-27377,z=-24141..23996\n\
        on x=-72682..-12347,y=18159..111354,z=7391..80950\n\
        on x=-1060..80757,y=-65301..-20884,z=-103788..-16709\n\
        on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\n\
        on x=-52752..22273,y=-49450..9096,z=54442..119054\n\
        on x=-29982..40483,y=-108474..-28371,z=-24328..38471\n\
        on x=-4958..62750,y=40422..118853,z=-7672..65583\n\
        on x=55694..108686,y=-43367..46958,z=-26781..48729\n\
        on x=-98497..-18186,y=-63569..3412,z=1232..88485\n\
        on x=-726..56291,y=-62629..13224,z=18033..85226\n\
        on x=-110886..-34664,y=-81338..-8658,z=8914..63723\n\
        on x=-55829..24974,y=-16897..54165,z=-121762..-28058\n\
        on x=-65152..-11147,y=22489..91432,z=-58782..1780\n\
        on x=-120100..-32970,y=-46592..27473,z=-11695..61039\n\
        on x=-18631..37533,y=-124565..-50804,z=-35667..28308\n\
        on x=-57817..18248,y=49321..117703,z=5745..55881\n\
        on x=14781..98692,y=-1341..70827,z=15753..70151\n\
        on x=-34419..55919,y=-19626..40991,z=39015..114138\n\
        on x=-60785..11593,y=-56135..2999,z=-95368..-26915\n\
        on x=-32178..58085,y=17647..101866,z=-91405..-8878\n\
        on x=-53655..12091,y=50097..105568,z=-75335..-4862\n\
        on x=-111166..-40997,y=-71714..2688,z=5609..50954\n\
        on x=-16602..70118,y=-98693..-44401,z=5197..76897\n\
        on x=16383..101554,y=4615..83635,z=-44907..18747\n\
        off x=-95822..-15171,y=-19987..48940,z=10804..104439\n\
        on x=-89813..-14614,y=16069..88491,z=-3297..45228\n\
        on x=41075..99376,y=-20427..49978,z=-52012..13762\n\
        on x=-21330..50085,y=-17944..62733,z=-112280..-30197\n\
        on x=-16478..35915,y=36008..118594,z=-7885..47086\n\
        off x=-98156..-27851,y=-49952..43171,z=-99005..-8456\n\
        off x=2032..69770,y=-71013..4824,z=7471..94418\n\
        on x=43670..120875,y=-42068..12382,z=-24787..38892\n\
        off x=37514..111226,y=-45862..25743,z=-16714..54663\n\
        off x=25699..97951,y=-30668..59918,z=-15349..69697\n\
        off x=-44271..17935,y=-9516..60759,z=49131..112598\n\
        on x=-61695..-5813,y=40978..94975,z=8655..80240\n\
        off x=-101086..-9439,y=-7088..67543,z=33935..83858\n\
        off x=18020..114017,y=-48931..32606,z=21474..89843\n\
        off x=-77139..10506,y=-89994..-18797,z=-80..59318\n\
        off x=8476..79288,y=-75520..11602,z=-96624..-24783\n\
        on x=-47488..-1262,y=24338..100707,z=16292..72967\n\
        off x=-84341..13987,y=2429..92914,z=-90671..-1318\n\
        off x=-37810..49457,y=-71013..-7894,z=-105357..-13188\n\
        off x=-27365..46395,y=31009..98017,z=15428..76570\n\
        off x=-70369..-16548,y=22648..78696,z=-1892..86821\n\
        on x=-53470..21291,y=-120233..-33476,z=-44150..38147\n\
        off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

    #[test]
    fn simple_test() {
        let steps: Vec<Step> = parse_fields(SIMPLE_TEST_DATA, '\n').unwrap();
        println!("Step 1:");
        let step_1_volume = activated_volume(&steps[0..1]);
        println!("Step 2:");
        let step_2_volume = activated_volume(&steps[0..2]);
        println!("Step 3:");
        let step_3_volume = activated_volume(&steps[0..3]);
        println!("Step 4:");
        let step_4_volume = activated_volume(&steps);

        assert_eq!(step_1_volume, 27, "step 1");
        assert_eq!(step_2_volume, 46, "step 2");
        assert_eq!(step_3_volume, 38, "step 3");
        assert_eq!(step_4_volume, 39, "step 4");
    }

    #[test]
    fn small_cubes() {
        let steps: Vec<Step> = parse_fields(LARGER_TEST_DATA, '\n').unwrap();
        let steps = small_cube_steps(&steps);
        let volume = activated_volume(&steps);

        assert_eq!(volume, 590784);
    }

    #[test]
    fn full_data() {
        let steps: Vec<Step> = parse_fields(FULL_TEST_DATA, '\n').unwrap();
        let volume = activated_volume(&steps);

        assert_eq!(volume, 2758514936282235);
    }
}
