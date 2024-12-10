#![feature(iterator_try_collect)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
        let mut position: usize = self.0.iter().sum();
        let mut allocator = Allocator::new(self);
        for (index, &size) in self.0.iter().enumerate().rev() {
            position -= size;
            if index % 2 == 1 {
                continue;
            }
            let file_id = index / 2;
            let allocated_position = allocator.allocate(size, position).unwrap_or(position);
            result += Self::contribution(file_id, allocated_position, size);
        }
        result
    }

    fn contribution(file_id: usize, position: usize, space: usize) -> usize {
        file_id * (2 * position + space - 1) * space / 2
    }
}

/// Use 10 min heaps indexed by size for quick access to the first free space of a given size.
/// Shamelessly copied (in concept) from https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day09.rs
#[derive(Debug, Clone)]
struct Allocator(Vec<BinaryHeap<Reverse<usize>>>);

impl Allocator {
    fn new(disk_map: &DiskMap) -> Self {
        let mut heaps = vec![BinaryHeap::<Reverse<usize>>::new(); 10];
        let mut position = 0;
        for (i, &size) in disk_map.0.iter().enumerate() {
            if i % 2 == 1 && size > 0 {
                heaps[size].push(Reverse(position));
            }
            position += size;
        }
        Allocator(heaps)
    }

    /// Allocates a file to the leftmost position and returns the position.
    fn allocate(&mut self, file_size: usize, ceiling: usize) -> Option<usize> {
        let mut best = (usize::MAX, 0);
        for space in file_size..10 {
            if let Some(Reverse(position)) = self.0[space].peek() {
                if *position > ceiling {
                    continue;
                }
                if *position < best.0 {
                    best = (*position, space);
                }
            }
        }
        if best == (usize::MAX, 0) {
            None
        } else {
            let space = best.1;
            let Reverse(position) = self.0[space].pop().unwrap();
            if file_size < space {
                self.0[space - file_size].push(Reverse(position + file_size));
            }
            Some(position)
        }
    }
}

impl std::str::FromStr for DiskMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DiskMap(
            s.bytes()
                .map(|b| match b {
                    b'0'..=b'9' => Ok((b - b'0') as usize),
                    _ => bail!("Unrecognized character {}", b as char),
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
