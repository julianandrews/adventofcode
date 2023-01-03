use anyhow::{anyhow, Result};

use aoc::utils::{get_input, parse_fields};

fn main() -> Result<()> {
    let input = get_input()?;
    let numbers: Vec<i64> = parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));

    Ok(())
}

fn part1(numbers: &[i64]) -> i64 {
    let mut list = MixList::from_values(numbers).unwrap(); // TODO
    list.mix(1);
    let (x, y, z) = list.grove_coordinates();
    x + y + z
}

fn part2(numbers: &[i64]) -> i64 {
    let mut list = MixList::from_encrypted_values(&numbers).unwrap(); // TODO
    list.mix(10);
    let (x, y, z) = list.grove_coordinates();
    x + y + z
}

#[derive(Debug)]
struct MixList {
    end: Link,
    len: usize,
}

impl MixList {
    fn mix(&mut self, n: usize) {
        let nodes = self.nodes();
        for _ in 0..n {
            for &node in &nodes {
                self.mix_node(node);
            }
        }
    }

    fn mix_node(&self, node: Link) {
        unsafe {
            let value = (*node).value;
            let offset = value.rem_euclid(self.len as i64 - 1) as usize;
            if offset == 0 {
                return;
            }
            let mut dest = node;
            for _ in 0..offset {
                dest = (*dest).next;
            }

            // Cut node out of the loop
            (*(*node).previous).next = (*node).next;
            (*(*node).next).previous = (*node).previous;

            // Insert node back into the loop!
            (*node).previous = dest;
            (*node).next = (*dest).next;
            (*(*dest).next).previous = node;
            (*dest).next = node;
        }
    }

    fn grove_coordinates(&self) -> (i64, i64, i64) {
        let mut node = self.find(0).unwrap(); // TODO
        let mut values = [0, 0, 0];
        for i in 0..3 {
            for _ in 0..1000 {
                node = unsafe { (*node).next };
            }
            values[i] = unsafe { (*node).value };
        }
        (values[0], values[1], values[2])
    }

    fn find(&self, value: i64) -> Option<Link> {
        for node in self.nodes() {
            if unsafe { (*node).value } == value {
                return Some(node);
            }
        }
        None
    }

    fn from_values(values: &[i64]) -> Result<Self> {
        let first_node: Link = Box::into_raw(Box::new(Node {
            value: *values.get(0).ok_or(anyhow!("Empty list"))?,
            next: std::ptr::null_mut(),
            previous: std::ptr::null_mut(),
        }));
        let mut current_node: Link = first_node;
        unsafe {
            for &value in &values[1..] {
                let new_node = Box::into_raw(Box::new(Node {
                    value,
                    next: std::ptr::null_mut(),
                    previous: current_node,
                }));
                (*current_node).next = new_node;
                current_node = new_node;
            }
            (*current_node).next = first_node;
            (*first_node).previous = current_node;
        }
        Ok(MixList {
            end: current_node,
            len: values.len(),
        })
    }

    fn from_encrypted_values(values: &[i64]) -> Result<Self> {
        static DECRYPTION_KEY: i64 = 811589153;
        let decrypted: Vec<i64> = values.iter().map(|x| x * DECRYPTION_KEY).collect();
        Self::from_values(&decrypted)
    }

    fn nodes(&self) -> Vec<Link> {
        let mut result = vec![];
        let mut node = self.end;
        for _ in 0..self.len {
            node = unsafe { (*node).next };
            result.push(node);
        }
        result
    }
}

type Link = *mut Node;

#[derive(Debug)]
struct Node {
    value: i64,
    next: Link,
    previous: Link,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &[i64] = &[1, 2, -3, 3, -2, 0, 4];

    #[test]
    fn simple_mix() {
        let mut list = MixList::from_values(TEST_DATA).unwrap();
        list.mix(1);
        assert_eq!(list.grove_coordinates(), (4, -3, 2));
    }

    #[test]
    fn decrypted_mix() {
        let mut list = MixList::from_encrypted_values(TEST_DATA).unwrap();
        list.mix(10);
        assert_eq!(
            list.grove_coordinates(),
            (811589153, 2434767459, -1623178306)
        );
    }
}
