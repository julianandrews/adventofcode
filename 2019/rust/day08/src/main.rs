extern crate aoc;

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::fmt;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
enum Pixel {
    Black = 0,
    White = 1,
    Transparent = 2,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pixel::Black => write!(f, " "),
            Pixel::White => write!(f, "█"),
            Pixel::Transparent => write!(f, "░"),
        }
    }
}

struct Image {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn from_data(data: &str, width: usize, height: usize) -> Result<Image> {
        let pixels = aoc::nums::digits(data)?
            .iter()
            .map(|&x| Pixel::try_from(x))
            .collect::<std::result::Result<Vec<Pixel>, _>>()?;

        Ok(Image {
            pixels: pixels,
            width: width,
            height: height,
        })
    }

    pub fn pixel_count(&self, layer: usize, pixel_type: Pixel) -> usize {
        self.pixels[layer * self.layer_size()..(layer + 1) * self.layer_size()]
            .iter()
            .filter(|&pixel| pixel_type == *pixel)
            .count()
    }

    pub fn layer_count(&self) -> usize {
        self.pixels.len() / self.layer_size()
    }

    fn layer_size(&self) -> usize {
        self.width * self.height
    }

    fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        for layer in 0..self.layer_count() {
            let pixel = self.pixels[layer * self.layer_size() + y * self.width + x];
            if pixel != Pixel::Transparent {
                return pixel;
            }
        }

        Pixel::Transparent
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines: Vec<String> = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| self.get_pixel(x, y).to_string())
                    .collect()
            })
            .collect();
        write!(f, "{}", lines.join(&"\n"))
    }
}

fn part1(image: &Image) -> Result<usize> {
    let layer = (0..image.layer_count())
        .min_by_key(|&x| image.pixel_count(x, Pixel::Black))
        .unwrap();

    Ok(image.pixel_count(layer, Pixel::White) * image.pixel_count(layer, Pixel::Transparent))
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let image = Image::from_data(input.trim(), 25, 6)?;

    println!("Part 1: {}", part1(&image)?);
    println!("Part 2: \n{}", image);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_display() {
        let result = Image::from_data("0222112222120000", 2, 2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), " █\n█ ");
    }
}
