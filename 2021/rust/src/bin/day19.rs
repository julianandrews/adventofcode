use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, bail, Result};
use nalgebra::{Matrix3, Vector3};

use aoc::utils::{get_input, parse_fields};

const MIN_BEACON_MATCHES: usize = 12;
const MIN_EDGE_MATCHES: usize = MIN_BEACON_MATCHES * (MIN_BEACON_MATCHES - 1) / 2;

type Point = Vector3<i64>;
type Rotation = Matrix3<i64>;

fn main() -> Result<()> {
    let input = get_input()?;
    let scanners: Vec<Scanner> = input
        .trim()
        .split("\n\n")
        .map(|s| s.parse())
        .collect::<Result<_>>()?;

    println!("Part 1: {}", part1(&scanners)?);
    println!("Part 2: {}", part2(&scanners)?);

    Ok(())
}

fn part1(scanners: &[Scanner]) -> Result<usize> {
    let beacons = locate_beacons(scanners).ok_or_else(|| anyhow!("Failed to find beacons"))?;
    Ok(beacons.len())
}

fn part2(scanners: &[Scanner]) -> Result<i64> {
    let transforms = locate_scanners(scanners).ok_or_else(|| anyhow!("Failed to find scanners"))?;
    let mut best = 0;
    for a in &transforms {
        for b in &transforms {
            let offset = a.offset - b.offset;
            let distance = offset[0].abs() + offset[1].abs() + offset[2].abs();
            best = best.max(distance);
        }
    }
    Ok(best)
}

fn locate_beacons(scanners: &[Scanner]) -> Option<HashSet<Point>> {
    let transforms = locate_scanners(scanners)?;
    let mut beacons = HashSet::new();
    for (scanner, transform) in scanners.iter().zip(transforms) {
        for beacon_position in &scanner.beacons {
            beacons.insert(transform.apply(beacon_position));
        }
    }
    Some(beacons)
}

fn locate_scanners(scanners: &[Scanner]) -> Option<Vec<Transform>> {
    let mut transforms = vec![None; scanners.len()];
    transforms[0] = Some(Transform::identity());
    let candidates = possible_neighbors(scanners);
    let mut to_check = vec![0];
    while let Some(id) = to_check.pop() {
        let scanner = &scanners[id];
        let base_transform: Transform = transforms[id].as_ref()?.clone();
        for &neighbor in &candidates[id] {
            if transforms[neighbor.id].is_some() {
                continue;
            }
            if let Some(transform) = scanner.find_transform(neighbor) {
                transforms[neighbor.id] = Some(base_transform.compose(&transform));
                to_check.push(neighbor.id);
            }
        }
    }
    transforms.into_iter().collect()
}

fn possible_neighbors(scanners: &[Scanner]) -> Vec<Vec<&Scanner>> {
    let distances: Vec<_> = scanners
        .iter()
        .map(|scanner| scanner.squared_distances())
        .collect();
    let mut candidates = vec![vec![]; scanners.len()];
    for (j, (scanner_1, dists_1)) in scanners.iter().zip(&distances).enumerate().skip(1) {
        for (i, (scanner_2, dists_2)) in scanners.iter().take(j).zip(&distances).enumerate() {
            let dist_matches = dists_1.intersection(dists_2).count();
            if dist_matches >= MIN_EDGE_MATCHES {
                candidates[i].push(scanner_1);
                candidates[j].push(scanner_2);
            }
        }
    }
    candidates
}

#[derive(Debug, Clone)]
struct Scanner {
    id: usize,
    beacons: Vec<Point>,
}

impl Scanner {
    fn find_transform(&self, other: &Scanner) -> Option<Transform> {
        let mut transforms = vec![];
        let diffs = find_diffs(&self.beacons);
        for rotation in all_rotations() {
            let rotated: Vec<_> = other.beacons.iter().map(|p| rotation * p).collect();
            let new_diffs = find_diffs(&rotated);
            let mut diff_matches = HashMap::new();
            for (diff, beacons) in &diffs {
                if let Some(other_beacons) = new_diffs.get(diff) {
                    for beacon_1 in beacons {
                        for beacon_2 in other_beacons {
                            *diff_matches.entry(beacon_1 - beacon_2).or_insert(0) += 1;
                        }
                    }
                }
            }
            for (offset, count) in diff_matches {
                if count >= 2 * MIN_EDGE_MATCHES {
                    transforms.push(Transform { offset, rotation });
                }
            }
        }
        if transforms.len() > 1 {
            eprintln!("Multiple transform candidates found");
            return None;
        }
        transforms.into_iter().next()
    }

    fn squared_distances(&self) -> HashSet<i64> {
        let mut distances = HashSet::new();
        for beacon_1 in &self.beacons {
            for beacon_2 in &self.beacons {
                let offset = beacon_1 - beacon_2;
                distances.insert(offset.dot(&offset));
            }
        }
        distances.remove(&0);
        distances
    }
}

fn find_diffs(points: &[Point]) -> HashMap<Point, Vec<Point>> {
    let mut diffs = HashMap::new();
    for (i, beacon_1) in points.iter().enumerate() {
        for beacon_2 in points.iter().skip(i + 1) {
            diffs
                .entry(beacon_1 - beacon_2)
                .or_insert_with(Vec::new)
                .push(*beacon_1);
            diffs
                .entry(beacon_2 - beacon_1)
                .or_insert_with(Vec::new)
                .push(*beacon_2);
        }
    }
    diffs
}

#[derive(Debug, Clone)]
struct Transform {
    offset: Point,
    rotation: Rotation,
}

impl Transform {
    fn identity() -> Self {
        Self {
            offset: Vector3::zeros(),
            rotation: Matrix3::identity(),
        }
    }

    fn apply(&self, position: &Point) -> Point {
        self.offset + self.rotation * position
    }

    fn compose(&self, other: &Transform) -> Self {
        Self {
            offset: self.offset + self.rotation * other.offset,
            rotation: self.rotation * other.rotation,
        }
    }
}

fn all_rotations() -> impl Iterator<Item = Rotation> {
    static ALL_ROTATIONS: [[i64; 9]; 24] = [
        [1, 0, 0, 0, 1, 0, 0, 0, 1],
        [1, 0, 0, 0, -1, 0, 0, 0, -1],
        [-1, 0, 0, 0, 1, 0, 0, 0, -1],
        [-1, 0, 0, 0, -1, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, -1, 0, 1, 0],
        [1, 0, 0, 0, 0, 1, 0, -1, 0],
        [-1, 0, 0, 0, 0, 1, 0, 1, 0],
        [-1, 0, 0, 0, 0, -1, 0, -1, 0],
        [0, 1, 0, 1, 0, 0, 0, 0, -1],
        [0, -1, 0, 1, 0, 0, 0, 0, 1],
        [0, 1, 0, -1, 0, 0, 0, 0, 1],
        [0, -1, 0, -1, 0, 0, 0, 0, -1],
        [0, 1, 0, 0, 0, 1, 1, 0, 0],
        [0, -1, 0, 0, 0, -1, 1, 0, 0],
        [0, 1, 0, 0, 0, -1, -1, 0, 0],
        [0, -1, 0, 0, 0, 1, -1, 0, 0],
        [0, 0, 1, 1, 0, 0, 0, 1, 0],
        [0, 0, -1, 1, 0, 0, 0, -1, 0],
        [0, 0, -1, -1, 0, 0, 0, 1, 0],
        [0, 0, 1, -1, 0, 0, 0, -1, 0],
        [0, 0, -1, 0, 1, 0, 1, 0, 0],
        [0, 0, 1, 0, -1, 0, 1, 0, 0],
        [0, 0, 1, 0, 1, 0, -1, 0, 0],
        [0, 0, -1, 0, -1, 0, -1, 0, 0],
    ];
    ALL_ROTATIONS
        .iter()
        .map(|a| nalgebra::Matrix3::from_row_slice(a))
}

impl std::str::FromStr for Scanner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines.next();
        let id: usize = header
            .and_then(|s| s.strip_prefix("--- scanner "))
            .and_then(|s| s.strip_suffix(" ---"))
            .ok_or_else(|| anyhow!("Failed to parse id"))?
            .parse()?;
        let beacons = lines
            .map(|line| {
                let nums = parse_fields(line, ',')?;
                if nums.len() != 3 {
                    bail!("Invalid point: {}", line);
                }
                Ok(Vector3::from_iterator(nums.into_iter()))
            })
            .collect::<Result<_>>()?;
        Ok(Self { id, beacons })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: [&str; 5] = [
        "--- scanner 0 ---\n\
        404,-588,-901\n\
        528,-643,409\n\
        -838,591,734\n\
        390,-675,-793\n\
        -537,-823,-458\n\
        -485,-357,347\n\
        -345,-311,381\n\
        -661,-816,-575\n\
        -876,649,763\n\
        -618,-824,-621\n\
        553,345,-567\n\
        474,580,667\n\
        -447,-329,318\n\
        -584,868,-557\n\
        544,-627,-890\n\
        564,392,-477\n\
        455,729,728\n\
        -892,524,684\n\
        -689,845,-530\n\
        423,-701,434\n\
        7,-33,-71\n\
        630,319,-379\n\
        443,580,662\n\
        -789,900,-551\n\
        459,-707,401",
        "--- scanner 1 ---\n\
        686,422,578\n\
        605,423,415\n\
        515,917,-361\n\
        -336,658,858\n\
        95,138,22\n\
        -476,619,847\n\
        -340,-569,-846\n\
        567,-361,727\n\
        -460,603,-452\n\
        669,-402,600\n\
        729,430,532\n\
        -500,-761,534\n\
        -322,571,750\n\
        -466,-666,-811\n\
        -429,-592,574\n\
        -355,545,-477\n\
        703,-491,-529\n\
        -328,-685,520\n\
        413,935,-424\n\
        -391,539,-444\n\
        586,-435,557\n\
        -364,-763,-893\n\
        807,-499,-711\n\
        755,-354,-619\n\
        553,889,-390",
        "--- scanner 2 ---\n\
        649,640,665\n\
        682,-795,504\n\
        -784,533,-524\n\
        -644,584,-595\n\
        -588,-843,648\n\
        -30,6,44\n\
        -674,560,763\n\
        500,723,-460\n\
        609,671,-379\n\
        -555,-800,653\n\
        -675,-892,-343\n\
        697,-426,-610\n\
        578,704,681\n\
        493,664,-388\n\
        -671,-858,530\n\
        -667,343,800\n\
        571,-461,-707\n\
        -138,-166,112\n\
        -889,563,-600\n\
        646,-828,498\n\
        640,759,510\n\
        -630,509,768\n\
        -681,-892,-333\n\
        673,-379,-804\n\
        -742,-814,-386\n\
        577,-820,562",
        "--- scanner 3 ---\n\
        -589,542,597\n\
        605,-692,669\n\
        -500,565,-823\n\
        -660,373,557\n\
        -458,-679,-417\n\
        -488,449,543\n\
        -626,468,-788\n\
        338,-750,-386\n\
        528,-832,-391\n\
        562,-778,733\n\
        -938,-730,414\n\
        543,643,-506\n\
        -524,371,-870\n\
        407,773,750\n\
        -104,29,83\n\
        378,-903,-323\n\
        -778,-728,485\n\
        426,699,580\n\
        -438,-605,-362\n\
        -469,-447,-387\n\
        509,732,623\n\
        647,635,-688\n\
        -868,-804,481\n\
        614,-800,639\n\
        595,780,-596",
        "--- scanner 4 ---\n\
        727,592,562\n\
        -293,-554,779\n\
        441,611,-461\n\
        -714,465,-776\n\
        -743,427,-804\n\
        -660,-479,-426\n\
        832,-632,460\n\
        927,-485,-438\n\
        408,393,-506\n\
        466,436,-512\n\
        110,16,151\n\
        -258,-428,682\n\
        -393,719,612\n\
        -211,-452,876\n\
        808,-476,-593\n\
        -575,615,604\n\
        -485,667,467\n\
        -680,325,-822\n\
        -627,-443,-432\n\
        872,-547,-609\n\
        833,512,582\n\
        807,604,487\n\
        839,-516,451\n\
        891,-625,532\n\
        -652,-548,-490\n\
        30,-46,-14",
    ];

    #[test]
    fn scanners() {
        let scanners: Vec<Scanner> = TEST_DATA.iter().map(|s| s.parse().unwrap()).collect();
        let scanner_positions: Vec<_> = locate_scanners(&scanners)
            .unwrap()
            .iter()
            .map(|transform| (transform.offset.x, transform.offset.y, transform.offset.z))
            .collect();
        let expected = vec![
            (0, 0, 0),
            (68, -1246, -43),
            (1105, -1205, 1229),
            (-92, -2380, -20),
            (-20, -1133, 1061),
        ];
        assert_eq!(scanner_positions, expected);
    }

    #[test]
    fn beacon_count() {
        let scanners: Vec<Scanner> = TEST_DATA.iter().map(|s| s.parse().unwrap()).collect();
        let beacons = locate_beacons(&scanners).unwrap();
        assert_eq!(beacons.len(), 79);
    }

    #[test]
    fn max_distance() {
        let scanners: Vec<Scanner> = TEST_DATA.iter().map(|s| s.parse().unwrap()).collect();
        let distance = part2(&scanners).unwrap();
        assert_eq!(distance, 3621)
    }
}
