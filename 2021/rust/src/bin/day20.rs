use anyhow::{anyhow, bail, Result};
use ndarray::Array2;
use ndarray_conv::Conv2DExt;

use aoc::utils::get_input;

fn main() -> Result<()> {
    let input = get_input()?;
    let (algorithm_part, image_part) = input
        .trim()
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Failed to split algorithm from image"))?;
    let algorithm: Algorithm = algorithm_part.parse()?;
    let image: Image = image_part.parse()?;

    println!("Part 1: {}", part1(&image, &algorithm)?);
    println!("Part 2: {}", part2(&image, &algorithm)?);

    Ok(())
}

fn part1(image: &Image, algorithm: &Algorithm) -> Result<u32> {
    let image = image.enhance(algorithm)?;
    let image = image.enhance(algorithm)?;
    Ok(image.lit_pixels())
}

fn part2(image: &Image, algorithm: &Algorithm) -> Result<u32> {
    let mut image = image.clone();
    for _ in 0..50 {
        image = image.enhance(algorithm)?;
    }
    Ok(image.lit_pixels())
}

#[derive(Debug, Clone)]
struct Image {
    /// An array covering the interesting region of space
    array: Array2<u32>,
    /// The value filling all of infinite space outside of `array`
    fill_value: u32,
}

impl Image {
    fn enhance(&self, algorithm: &Algorithm) -> Result<Self> {
        let [width, height] = self.array.shape() else { unreachable!() };

        // Pad out the array by 2 on each side so that we correctly account for fill
        // when we shrink the array down to just 1 space padding on each side.
        let mut array = if self.fill_value == 0 {
            Array2::<u32>::zeros(ndarray::Ix2(width + 4, height + 4))
        } else {
            Array2::<u32>::ones(ndarray::Ix2(width + 4, height + 4))
        };
        array
            .slice_mut(ndarray::s![2..width + 2, 2..height + 2])
            .assign(&self.array);

        // Convolve and lookup
        let kernel = Array2::from_shape_vec((3, 3), vec![256, 128, 64, 32, 16, 8, 4, 2, 1])?;
        array = array.conv_2d(&kernel).unwrap();
        array.mapv_inplace(|value| algorithm.lookup(value));

        // Shrink the array back down to just 1 extra on each side.
        array = array.slice_move(ndarray::s![1..width + 3, 1..height + 3]);

        let fill_value = algorithm.lookup(if self.fill_value == 0 { 0 } else { 0b111111111 });
        Ok(Self { array, fill_value })
    }

    fn lit_pixels(&self) -> u32 {
        self.array.sum()
    }
}

#[derive(Debug, Clone)]
struct Algorithm {
    values: Vec<u32>,
}

impl Algorithm {
    fn lookup(&self, value: u32) -> u32 {
        self.values[value as usize]
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = vec![];
        for row in self.array.rows() {
            lines.push(
                row.iter()
                    .map(|&value| if value == 0 { '.' } else { '#' })
                    .collect(),
            );
        }
        write!(f, "{}", lines.join("\n"))
    }
}

impl std::str::FromStr for Algorithm {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = parse_line(s)?;
        if values.len() != 512 {
            bail!("Invalid algorithm: wrong length");
        }
        Ok(Self { values })
    }
}

impl std::str::FromStr for Image {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<u32>> = s.lines().map(parse_line).collect::<Result<_>>()?;
        let height = lines.len();
        let width = lines.get(0).map(|line| line.len()).unwrap_or(0);
        if lines.iter().any(|line| line.len() != width) {
            bail!("Non rectangular image!");
        }
        let values: Vec<u32> = lines.into_iter().flatten().collect();
        let array = Array2::from_shape_vec((width, height), values)?;
        let fill_value = 0;
        Ok(Self { array, fill_value })
    }
}

fn parse_line(s: &str) -> Result<Vec<u32>> {
    s.bytes()
        .map(|b| match b {
            b'#' => Ok(1),
            b'.' => Ok(0),
            _ => bail!("Unexpected character in algorithm"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_IMAGE: &str = "\
        #..#.\n\
        #....\n\
        ##..#\n\
        ..#..\n\
        ..###";

    static TEST_ALGORITHM: &str = "\
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";

    #[test]
    fn enhance_once() {
        let image: Image = TEST_IMAGE.parse().unwrap();
        let algorithm: Algorithm = TEST_ALGORITHM.parse().unwrap();
        let enhanced = image.enhance(&algorithm).unwrap();
        println!("{}", enhanced);
        let expected = "\
            .##.##.\n\
            #..#.#.\n\
            ##.#..#\n\
            ####..#\n\
            .#..##.\n\
            ..##..#\n\
            ...#.#.";
        assert_eq!(enhanced.lit_pixels(), 24);
        assert_eq!(enhanced.to_string(), expected);
    }

    #[test]
    fn enhance_twice() {
        let image: Image = TEST_IMAGE.parse().unwrap();
        let algorithm: Algorithm = TEST_ALGORITHM.parse().unwrap();
        let enhanced = image
            .enhance(&algorithm)
            .unwrap()
            .enhance(&algorithm)
            .unwrap();
        let expected = "\
            .......#.\n\
            .#..#.#..\n\
            #.#...###\n\
            #...##.#.\n\
            #.....#.#\n\
            .#.#####.\n\
            ..#.#####\n\
            ...##.##.\n\
            ....###..";

        assert_eq!(enhanced.lit_pixels(), 35);
        assert_eq!(enhanced.to_string(), expected);
    }

    #[test]
    fn enhance_lots() {
        let image: Image = TEST_IMAGE.parse().unwrap();
        let algorithm: Algorithm = TEST_ALGORITHM.parse().unwrap();
        let pixel_count = part2(&image, &algorithm).unwrap();
        assert_eq!(pixel_count, 3351);
    }
}
