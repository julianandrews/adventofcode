#![feature(str_split_once)]

use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

use aoc::aoc_error::AOCError;
use aoc::graphs::{bfs, Graph};
use aoc::orientation::Orientation;

use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let tiles = parse_input(&input)?;
    let image = Image::new(tiles)?;

    println!("Part 1: {}", part1(&image)?);
    println!("Part 2: {}", part2(&image));
    Ok(())
}

fn part1(image: &Image) -> Result<usize> {
    let tile_ids = image.tile_ids();
    let max_x = *tile_ids.keys().map(|(x, _y)| x).max().unwrap();
    let max_y = *tile_ids.keys().map(|(_x, y)| y).max().unwrap();
    let corner_ids = vec![(0, 0), (0, max_y), (max_x, 0), (max_x, max_y)]
        .iter()
        .map(|coords| tile_ids.get(coords).copied())
        .collect::<Option<Vec<_>>>()
        .ok_or(AOCError::new("Missing corner"))?;

    Ok(corner_ids.iter().product())
}

fn part2(image: &Image) -> usize {
    image.water_roughness()
}

fn parse_input(input: &str) -> Result<Vec<Tile>> {
    input
        .trim()
        .split("\n\n")
        .map(&str::parse)
        .collect::<Result<_>>()
}

struct Image {
    tiles: HashMap<(usize, usize), (Tile, Orientation)>,
}

impl Image {
    fn new(tiles: Vec<Tile>) -> Result<Self> {
        // Lets assume the first tile is at (0, 0) with identity orientation.
        let start = (tiles[0].id, (0, 0), Orientation::Identity);
        let tile_graph = TileGraph::new(tiles)?;
        let mut mapped_tiles: HashMap<(i64, i64), (Tile, Orientation)> = HashMap::new();

        // TileGraph knows how to find all the neighbors for a given tile with appropriate
        // coordinates and orientations.
        for node in bfs(&tile_graph, start) {
            let (tile_id, coords, orientation) = node.value;
            let tile = tile_graph.tiles_by_id.get(&tile_id).unwrap().clone();
            if mapped_tiles.contains_key(&coords) {
                return Err(AOCError::new("Tiles overlap in space").into());
            }
            mapped_tiles.insert(coords, (tile, orientation));
        }

        // Normalize the coordinates so the upper left corner is at (0, 0)
        let min_x = *mapped_tiles.keys().map(|(x, _)| x).min().unwrap();
        let min_y = *mapped_tiles.keys().map(|(_, y)| y).min().unwrap();
        let tiles = mapped_tiles
            .into_iter()
            .map(|((x, y), v)| (((x - min_x) as usize, (y - min_y) as usize), v))
            .collect();

        Ok(Image { tiles })
    }

    fn pixel_at(&self, x: usize, y: usize) -> bool {
        let (tile, &orientation) = match self.tiles.get(&(x / 8, y / 8)) {
            Some((tile, orientation)) => (tile, orientation),
            None => return false,
        };
        tile.pixel_at(x % 8, y % 8, orientation)
    }

    fn tile_ids(&self) -> HashMap<(usize, usize), usize> {
        self.tiles
            .iter()
            .map(|(&coords, (tile, _))| (coords, tile.id))
            .collect()
    }

    fn coordinates<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.tiles.keys().flat_map(|(t_x, t_y)| {
            (0..8).flat_map(move |x| (0..8).map(move |y| (8 * t_x + x, 8 * t_y + y)))
        })
    }

    fn monster_pixels(&self, orientation: Orientation) -> HashSet<(usize, usize)> {
        static BASE_MONSTER: [(usize, usize); 15] = [
            (0, 1),
            (1, 0),
            (4, 0),
            (5, 1),
            (6, 1),
            (7, 0),
            (10, 0),
            (11, 1),
            (12, 1),
            (13, 0),
            (16, 0),
            (17, 1),
            (18, 2),
            (18, 1),
            (19, 1),
        ];
        let monster: Vec<_> = BASE_MONSTER
            .iter()
            .map(|&(x, y)| orientation_transform(x, 19, y, 2, orientation))
            .collect();
        let mut monster_pixels = HashSet::new();
        for (x_0, y_0) in self.coordinates() {
            let pixels: Vec<_> = monster.iter().map(|&(x, y)| (x + x_0, y + y_0)).collect();
            if pixels.iter().all(|&(x, y)| self.pixel_at(x, y)) {
                pixels.into_iter().for_each(|pixel| {
                    monster_pixels.insert(pixel);
                });
            }
        }
        monster_pixels
    }

    fn water_roughness(&self) -> usize {
        let monster_tile_count = Orientation::iterator()
            .map(|orientation| self.monster_pixels(orientation).len())
            .max()
            .unwrap_or(0);
        let rough_count = self
            .tiles
            .values()
            .map(|(tile, _)| tile.roughness())
            .sum::<usize>();

        rough_count - monster_tile_count
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_tx = self.tiles.keys().map(|(x, _)| x).max().unwrap_or(&0);
        let max_ty = self.tiles.keys().map(|(_, y)| y).max().unwrap_or(&0);
        let lines: Vec<_> = (0..8 * (max_ty + 1))
            .map(|y| {
                (0..8 * (max_tx + 1))
                    .map(|x| if self.pixel_at(x, y) { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect();
        write!(f, "{}", lines.join("\n"))
    }
}

struct TileGraph {
    tiles_by_id: HashMap<usize, Tile>,
    pairs: HashMap<u16, ((usize, Orientation), (usize, Orientation))>,
}

impl TileGraph {
    fn new(tiles: Vec<Tile>) -> Result<Self> {
        if tiles.is_empty() {
            return Err(AOCError::new("No tiles provided").into());
        }

        let mut edge_values = HashMap::new();
        let mut tiles_by_id: HashMap<usize, Tile> = HashMap::new();
        for tile in tiles {
            for orientation in Orientation::iterator() {
                let value = tile.edge_hash(orientation);
                edge_values
                    .entry(value)
                    .or_insert_with(Vec::new)
                    .push((tile.id, orientation));
            }
            tiles_by_id.insert(tile.id, tile);
        }
        if edge_values.values().any(|items| items.len() > 2) {
            return Err(AOCError::new("Multiple edges line up!").into());
        }
        let pairs = edge_values
            .into_iter()
            .filter(|(_, items)| items.len() == 2 && items[0].1.is_rotation())
            .map(|(value, items)| (value, (items[0], items[1])))
            .collect();

        Ok(TileGraph { tiles_by_id, pairs })
    }
}

impl<'a> Graph<'a> for TileGraph {
    type Item = (usize, (i64, i64), Orientation);

    fn nodes(&'a self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        unimplemented!()
    }

    fn neighbors(&'a self, value: &Self::Item) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        let &(tile_id, (x, y), tile_orientation) = value;
        let tile = self.tiles_by_id.get(&tile_id).unwrap();
        let mut neighbors = vec![];
        for rel_edge_orientation in Orientation::iterator() {
            if let Some(&(a, b)) = self.pairs.get(&tile.edge_hash(rel_edge_orientation)) {
                let edge = tile_orientation * rel_edge_orientation;
                let new_coords = new_tile_coords(x, y, edge);
                let (new_id, rel_orientation) = if tile.id == a.0 {
                    (b.0, a.1 * b.1.inverse())
                } else {
                    (a.0, b.1 * a.1.inverse())
                };
                // Line our new tile up with the old tile, and flip across the edge.
                let new_orientation = edge_flip(edge) * tile_orientation * rel_orientation;

                neighbors.push((new_id, new_coords, new_orientation));
            }
        }

        Box::new(neighbors.into_iter())
    }
}

fn new_tile_coords(x: i64, y: i64, edge: Orientation) -> (i64, i64) {
    use Orientation::*;
    match edge {
        Identity | HorizontalFlip => (x, y - 1),
        Rotate90 | SecondDiagonalFlip => (x + 1, y),
        Rotate180 | VerticalFlip => (x, y + 1),
        Rotate270 | MainDiagonalFlip => (x - 1, y),
    }
}

fn edge_flip(edge: Orientation) -> Orientation {
    use Orientation::*;
    match edge {
        Identity | Rotate180 | HorizontalFlip | VerticalFlip => VerticalFlip,
        _ => HorizontalFlip,
    }
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    image: [u8; 8],
    edges: [u16; 4],
}

impl Tile {
    fn edge_hash(&self, orientation: Orientation) -> u16 {
        match orientation {
            Orientation::Identity => self.edges[0],
            Orientation::HorizontalFlip => reverse_n_bits(10, self.edges[0]),
            Orientation::Rotate90 => self.edges[1],
            Orientation::SecondDiagonalFlip => reverse_n_bits(10, self.edges[1]),
            Orientation::Rotate180 => self.edges[2],
            Orientation::VerticalFlip => reverse_n_bits(10, self.edges[2]),
            Orientation::Rotate270 => self.edges[3],
            Orientation::MainDiagonalFlip => reverse_n_bits(10, self.edges[3]),
        }
    }

    fn pixel_at(&self, x: usize, y: usize, orientation: Orientation) -> bool {
        if x > 7 || y > 7 {
            return false;
        }
        let (x, y) = orientation_transform(x, 7, y, 7, orientation);
        self.image.get(y).unwrap() & (1 << x) != 0
    }

    fn roughness(&self) -> usize {
        self.image.iter().map(|x| x.count_ones()).sum::<u32>() as usize
    }
}

fn orientation_transform(
    x: usize,
    max_x: usize,
    y: usize,
    max_y: usize,
    orientation: Orientation,
) -> (usize, usize) {
    match orientation {
        Orientation::Identity => (x, y),
        Orientation::Rotate90 => (y, max_x - x),
        Orientation::Rotate180 => (max_x - x, max_y - y),
        Orientation::Rotate270 => (max_y - y, x),
        Orientation::HorizontalFlip => (max_x - x, y),
        Orientation::SecondDiagonalFlip => (max_y - y, max_x - x),
        Orientation::VerticalFlip => (x, max_y - y),
        Orientation::MainDiagonalFlip => (y, x),
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<_> = (0..8)
            .map(|y| {
                (0..8)
                    .map(|x| {
                        if self.pixel_at(x, y, Orientation::Identity) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect();
        write!(f, "{}", lines.join("\n"))
    }
}

impl std::str::FromStr for Tile {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (header, image_part) = s.split_once('\n').ok_or(AOCError::new("Invalid Tile"))?;
        let id = header
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse()?;
        let lines: Vec<_> = image_part.lines().collect();
        if lines.len() != 10 || lines.iter().any(|line| line.len() != 10) {
            return Err(AOCError::new("Invalid tile").into());
        }
        let edges = [
            make_binary(lines[0].chars())?,
            make_binary(lines.iter().map(|line| line.chars().last().unwrap()))?,
            make_binary(lines[9].chars().rev())?,
            make_binary(lines.iter().map(|line| line.chars().next().unwrap()).rev())?,
        ];
        let image: Vec<u8> = lines[1..9]
            .iter()
            .map(|line| make_binary(line.chars().skip(1).take(8)).map(|x| x as u8))
            .collect::<Result<_>>()?;
        let image: [u8; 8] = image
            .try_into()
            .map_err(|_| AOCError::new("Invalid tile"))?;

        Ok(Self { id, image, edges })
    }
}

fn make_binary(chars: impl Iterator<Item = char>) -> Result<u16> {
    let mut value = 0;
    for (i, c) in chars.enumerate() {
        value |= match c {
            '#' => 1 << i,
            '.' => 0,
            _ => return Err(AOCError::new("Invalid tile").into()),
        }
    }
    Ok(value)
}

fn reverse_n_bits(n: u8, mut value: u16) -> u16 {
    let mut result = 0;
    for i in (16 - n..16).rev() {
        result += (value & 1) << i;
        value >>= 1;
    }

    result >> 16 - n
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tile 3079 moved to top so that it will be assumed to be in identity
    // orientation and test results will match the provided sample.
    static TEST_INPUT: &'static str = "Tile 3079:\
                                     \n#.#.#####.\
                                     \n.#..######\
                                     \n..#.......\
                                     \n######....\
                                     \n####.#..#.\
                                     \n.#...#.##.\
                                     \n#.#####.##\
                                     \n..#.###...\
                                     \n..#.......\
                                     \n..#.###...\
                                     \n\
                                     \nTile 2311:\
                                     \n..##.#..#.\
                                     \n##..#.....\
                                     \n#...##..#.\
                                     \n####.#...#\
                                     \n##.##.###.\
                                     \n##...#.###\
                                     \n.#.#.#..##\
                                     \n..#....#..\
                                     \n###...#.#.\
                                     \n..###..###\
                                     \n\
                                     \nTile 1951:\
                                     \n#.##...##.\
                                     \n#.####...#\
                                     \n.....#..##\
                                     \n#...######\
                                     \n.##.#....#\
                                     \n.###.#####\
                                     \n###.##.##.\
                                     \n.###....#.\
                                     \n..#.#..#.#\
                                     \n#...##.#..\
                                     \n\
                                     \nTile 1171:\
                                     \n####...##.\
                                     \n#..##.#..#\
                                     \n##.#..#.#.\
                                     \n.###.####.\
                                     \n..###.####\
                                     \n.##....##.\
                                     \n.#...####.\
                                     \n#.##.####.\
                                     \n####..#...\
                                     \n.....##...\
                                     \n\
                                     \nTile 1427:\
                                     \n###.##.#..\
                                     \n.#..#.##..\
                                     \n.#.##.#..#\
                                     \n#.#.#.##.#\
                                     \n....#...##\
                                     \n...##..##.\
                                     \n...#.#####\
                                     \n.#.####.#.\
                                     \n..#..###.#\
                                     \n..##.#..#.\
                                     \n\
                                     \nTile 1489:\
                                     \n##.#.#....\
                                     \n..##...#..\
                                     \n.##..##...\
                                     \n..#...#...\
                                     \n#####...#.\
                                     \n#..#.#.#.#\
                                     \n...#.#.#..\
                                     \n##.#...##.\
                                     \n..##.##.##\
                                     \n###.##.#..\
                                     \n\
                                     \nTile 2473:\
                                     \n#....####.\
                                     \n#..#.##...\
                                     \n#.##..#...\
                                     \n######.#.#\
                                     \n.#...#.#.#\
                                     \n.#########\
                                     \n.###.#..#.\
                                     \n########.#\
                                     \n##...##.#.\
                                     \n..###.#.#.\
                                     \n\
                                     \nTile 2971:\
                                     \n..#.#....#\
                                     \n#...###...\
                                     \n#.#.###...\
                                     \n##.##..#..\
                                     \n.#####..##\
                                     \n.#..####.#\
                                     \n#..#.#..#.\
                                     \n..####.###\
                                     \n..#.#.###.\
                                     \n...#.#.#.#\
                                     \n\
                                     \nTile 2729:\
                                     \n...#.#.#.#\
                                     \n####.#....\
                                     \n..#.#.....\
                                     \n....#..#.#\
                                     \n.##..##.#.\
                                     \n.#.####...\
                                     \n####.#.#..\
                                     \n##.####...\
                                     \n##..#.##..\
                                     \n#.##...##.";

    #[test]
    fn reverse_bits() {
        assert_eq!(reverse_n_bits(10, 0b1111111110), 0b0111111111);
        assert_eq!(reverse_n_bits(10, 0b0000001011), 0b1101000000);
        assert_eq!(reverse_n_bits(10, 0b1100100000), 0b0000010011);
    }

    #[test]
    fn build_image() {
        let tiles = parse_input(TEST_INPUT).unwrap();
        let image = Image::new(tiles).unwrap();
        let expected = vec![
            ((0, 0), 1951, Orientation::VerticalFlip),
            ((1, 0), 2311, Orientation::VerticalFlip),
            ((2, 0), 3079, Orientation::Identity),
            ((0, 1), 2729, Orientation::VerticalFlip),
            ((1, 1), 1427, Orientation::VerticalFlip),
            ((2, 1), 2473, Orientation::SecondDiagonalFlip),
            ((0, 2), 2971, Orientation::VerticalFlip),
            ((1, 2), 1489, Orientation::VerticalFlip),
            ((2, 2), 1171, Orientation::HorizontalFlip),
        ]
        .into_iter()
        .collect();
        let result: HashSet<_> = image
            .tiles
            .iter()
            .map(|(&coords, (tile, orientation))| (coords, tile.id, *orientation))
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn display_image() {
        let tiles = parse_input(TEST_INPUT).unwrap();
        let image = Image::new(tiles).unwrap();
        let expected = ".#.#..#.##...#.##..#####\
                      \n###....#.#....#..#......\
                      \n##.##.###.#.#..######...\
                      \n###.#####...#.#####.#..#\
                      \n##.#....#.##.####...#.##\
                      \n...########.#....#####.#\
                      \n....#..#...##..#.#.###..\
                      \n.####...#..#.....#......\
                      \n#..#.##..#..###.#.##....\
                      \n#.####..#.####.#.#.###..\
                      \n###.#.#...#.######.#..##\
                      \n#.####....##..########.#\
                      \n##..##.#...#...#.#.#.#..\
                      \n...#..#..#.#.##..###.###\
                      \n.#.#....#.##.#...###.##.\
                      \n###.#...#..#.##.######..\
                      \n.#.#.###.##.##.#..#.##..\
                      \n.####.###.#...###.#..#.#\
                      \n..#.#..#..#.#.#.####.###\
                      \n#..####...#.#.#.###.###.\
                      \n#####..#####...###....##\
                      \n#.##..#..#...#..####...#\
                      \n.#.###..##..##..####.##.\
                      \n...###...##...#...#..###";
        assert_eq!(image.to_string(), expected);
    }

    #[test]
    fn find_roughness() {
        let tiles = parse_input(TEST_INPUT).unwrap();
        let image = Image::new(tiles).unwrap();

        assert_eq!(image.water_roughness(), 273);
    }
}
