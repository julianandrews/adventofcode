#![feature(iterator_try_collect)]

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let disk_map: DiskMap = input.trim().parse()?;

    println!("Part 1: {}", part1(&disk_map));
    println!("Part 2: {}", part2(&disk_map));

    Ok(())
}

fn part1(disk_map: &DiskMap) -> usize {
    disk_map.fragmented_checksum()
}

fn part2(disk_map: &DiskMap) -> usize {
    disk_map.checksum()
}

struct DiskMap(Vec<usize>);

impl DiskMap {
    fn fragmented_checksum(&self) -> usize {
        let mut result = 0;
        let mut position = 0;
        let mut rev_files = self.0.iter().step_by(2).enumerate().rev();
        let mut rev_file_id = (self.0.len() + 1) / 2;
        let mut rev_file_size = 0;

        for (i, n) in self.0.iter().enumerate() {
            match i % 2 {
                0 => {
                    let file_id = i / 2;
                    let mut space = *n;
                    if file_id == rev_file_id {
                        space = rev_file_size;
                        rev_file_size = 0;
                    }
                    result += Self::contribution(file_id, position, space);
                    position += space;
                }
                _ => {
                    let mut n = *n;
                    while n > 0 {
                        if rev_file_size == 0 {
                            (rev_file_id, rev_file_size) = match rev_files.next() {
                                Some((a, &b)) => (a, b),
                                None => break,
                            };
                        }
                        if rev_file_id <= i / 2 {
                            return result;
                        }
                        let space = n.min(rev_file_size);
                        result += Self::contribution(rev_file_id, position, space);
                        position += space;
                        rev_file_size -= space;
                        n -= space;
                    }
                }
            }
        }
        result
    }

    fn checksum(&self) -> usize {
        let mut result = 0;
        let mut positions: Vec<_> = self
            .0
            .iter()
            .scan(0, |total, &n| {
                let value = *total;
                *total += n;
                Some(value)
            })
            .collect();
        let mut spaces: Vec<usize> = self.0.iter().skip(1).step_by(2).copied().collect();
        for (file_id, &file_size) in self.0.iter().step_by(2).enumerate().rev() {
            let mut moved = false;
            // TODO: Avoid re-checking filled spaces.
            for (i, space) in spaces
                .iter_mut()
                .enumerate()
                .take_while(|&(i, _)| file_id > i)
            {
                if *space >= file_size {
                    *space -= file_size;
                    let position = positions[2 * i + 1];
                    positions[2 * i + 1] += file_size;
                    result += Self::contribution(file_id, position, file_size);
                    moved = true;
                    break;
                }
            }
            if !moved {
                let position = positions[file_id * 2];
                result += Self::contribution(file_id, position, file_size);
            }
        }
        result
    }

    fn contribution(file_id: usize, position: usize, space: usize) -> usize {
        file_id * (2 * position + space - 1) * space / 2
    }
}

impl std::str::FromStr for DiskMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DiskMap(
            s.chars()
                .map(|c| match c {
                    '0'..='9' => Ok((c as u8 - '0' as u8) as usize),
                    _ => bail!("Unrecognized character {}", c),
                })
                .try_collect()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "2333133121414131402";

    #[test]
    fn fragmented_checksum() {
        let disk_map: DiskMap = TEST_DATA.parse().unwrap();
        assert_eq!(disk_map.fragmented_checksum(), 1928);
    }

    #[test]
    fn checksum() {
        let disk_map: DiskMap = TEST_DATA.parse().unwrap();
        assert_eq!(disk_map.checksum(), 2858);
    }

    #[test]
    fn checksum_move_wrong_way() {
        let disk_map: DiskMap = "12345".parse().unwrap();
        assert_eq!(disk_map.checksum(), 132);
    }
}
