use anyhow::Result;

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let boxes: Vec<Box> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&boxes));
    println!("Part 2: {}", part2(&boxes));

    Ok(())
}

fn part1(boxes: &[Box]) -> u64 {
    boxes.iter().map(Box::wrapping_paper).sum()
}

fn part2(boxes: &[Box]) -> u64 {
    boxes.iter().map(Box::ribbon).sum()
}

struct Box {
    length: u64,
    width: u64,
    height: u64,
}

impl Box {
    fn wrapping_paper(&self) -> u64 {
        let sides = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        let surface_area = 2 * sides.iter().sum::<u64>();

        sides.iter().min().unwrap() + surface_area
    }

    fn ribbon(&self) -> u64 {
        let perimeters = [
            2 * (self.length + self.width),
            2 * (self.width + self.height),
            2 * (self.height + self.length),
        ];
        let volume = self.length * self.width * self.height;

        perimeters.iter().min().unwrap() + volume
    }
}

mod parsing {
    use super::Box;

    use anyhow::bail;

    impl std::str::FromStr for Box {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let dimensions: Vec<u64> = s
                .split('x')
                .map(|x| x.parse())
                .collect::<std::result::Result<_, _>>()?;
            if dimensions.len() != 3 {
                bail!("Failed to parse line: {}", s);
            }

            Ok(Box {
                length: dimensions[0],
                width: dimensions[1],
                height: dimensions[2],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Box;

    #[test]
    fn wrapping_paper_1() {
        let b: Box = "2x3x4".parse().unwrap();
        assert_eq!(b.wrapping_paper(), 58);
    }

    #[test]
    fn wrapping_paper_2() {
        let b: Box = "1x1x10".parse().unwrap();
        assert_eq!(b.wrapping_paper(), 43);
    }

    #[test]
    fn ribbon_1() {
        let b: Box = "2x3x4".parse().unwrap();
        assert_eq!(b.ribbon(), 34);
    }
    #[test]
    fn ribbon_2() {
        let b: Box = "1x1x10".parse().unwrap();
        assert_eq!(b.ribbon(), 14);
    }
}
