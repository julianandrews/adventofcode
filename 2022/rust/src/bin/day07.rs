use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Result};
use rustc_hash::FxHashMap;

use aoc::utils::get_input;

type Cache<'a> = FxHashMap<&'a PathBuf, u32>;

fn main() -> Result<()> {
    let input = get_input()?;
    let filesystem = input.trim().parse()?;
    let mut cache: Cache = FxHashMap::default();

    println!("Part 1: {}", part1(&filesystem, &mut cache));
    println!("Part 2: {}", part2(&filesystem, &mut cache));

    Ok(())
}

fn part1<'fs: 'cache, 'cache>(filesystem: &'fs FileSystem, cache: &mut Cache<'cache>) -> u32 {
    filesystem
        .directories()
        .map(|path| filesystem.size(path, cache))
        .filter(|&size| size <= 100000)
        .sum()
}

fn part2<'fs: 'cache, 'cache>(filesystem: &'fs FileSystem, cache: &mut Cache<'cache>) -> u32 {
    static MAX_FILESYSTEM_SIZE: u32 = 40000000;
    let to_free = filesystem
        .size("/", cache)
        .saturating_sub(MAX_FILESYSTEM_SIZE);
    let mut dir_sizes: Vec<u32> = filesystem
        .directories()
        .map(|path| filesystem.size(path, cache))
        .filter(|&size| size >= to_free)
        .collect();
    dir_sizes.sort_unstable();
    *dir_sizes.first().unwrap_or(&0)
}

#[derive(Debug, Clone)]
struct FileSystem(FxHashMap<PathBuf, Node>);

impl FileSystem {
    fn size<'cache, 'fs: 'cache, P: AsRef<Path>>(
        &'fs self,
        path: P,
        cache: &mut Cache<'cache>,
    ) -> u32 {
        let path = path.as_ref();
        let (path, node) = match self.0.get_key_value(path) {
            Some((path, node)) => (path, node),
            None => return 0,
        };
        if let Some(&value) = cache.get(path) {
            return value;
        }
        let value = match node {
            Node::Directory(paths) => paths.iter().map(|p| self.size(path.join(p), cache)).sum(),
            Node::File(size) => *size,
        };
        cache.insert(path, value);
        value
    }

    fn directories(&self) -> impl Iterator<Item = &Path> {
        self.0
            .iter()
            .filter(|(_, node)| matches!(node, Node::Directory(_)))
            .map(|(path, _)| path.as_path())
    }
}

impl std::str::FromStr for FileSystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut filesystem = FxHashMap::default();
        let mut pwd = PathBuf::new();
        let mut lines = s.lines().peekable();
        while let Some(line) = lines.next() {
            match line {
                "$ cd /" => {
                    pwd.clear();
                    pwd.push("/");
                }
                "$ cd .." => {
                    pwd.pop();
                }
                s if s.starts_with("$ cd ") => pwd.push(&s[5..]),
                "$ ls" => {
                    let mut files = vec![];
                    while lines.peek().map(|s| !s.starts_with('$')).unwrap_or(false) {
                        let line = lines.next().unwrap();
                        let (descriptor, name) = line
                            .split_once(' ')
                            .ok_or_else(|| anyhow!("Invalid dir entry: {}", line))?;
                        let mut path = pwd.clone();
                        path.push(name);
                        files.push(name.into());
                        let node = match descriptor {
                            "dir" => Node::Directory(vec![]),
                            _ => Node::File(descriptor.parse()?),
                        };
                        filesystem.insert(path, node);
                    }
                    filesystem.insert(pwd.clone(), Node::Directory(files));
                }
                _ => bail!("Expected command"),
            }
        }
        Ok(FileSystem(filesystem))
    }
}

#[derive(Debug, Clone)]
enum Node {
    Directory(Vec<PathBuf>),
    File(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &str = "\
        $ cd /\n\
        $ ls\n\
        dir a\n\
        14848514 b.txt\n\
        8504156 c.dat\n\
        dir d\n\
        $ cd a\n\
        $ ls\n\
        dir e\n\
        29116 f\n\
        2557 g\n\
        62596 h.lst\n\
        $ cd e\n\
        $ ls\n\
        584 i\n\
        $ cd ..\n\
        $ cd ..\n\
        $ cd d\n\
        $ ls\n\
        4060174 j\n\
        8033020 d.log\n\
        5626152 d.ext\n\
        7214296 k";

    #[test]
    fn size_1() {
        let filesystem: FileSystem = TEST_DATA.parse().unwrap();
        assert_eq!(filesystem.size("/a/e", &mut FxHashMap::default()), 584);
    }

    #[test]
    fn size_2() {
        let filesystem: FileSystem = TEST_DATA.parse().unwrap();
        assert_eq!(filesystem.size("/a", &mut FxHashMap::default()), 94853);
    }

    #[test]
    fn size_3() {
        let filesystem: FileSystem = TEST_DATA.parse().unwrap();
        assert_eq!(filesystem.size("/d", &mut FxHashMap::default()), 24933642);
    }

    #[test]
    fn size_4() {
        let filesystem: FileSystem = TEST_DATA.parse().unwrap();
        assert_eq!(filesystem.size("/", &mut FxHashMap::default()), 48381165);
    }

    #[test]
    fn sum_small() {
        let filesystem: FileSystem = TEST_DATA.parse().unwrap();
        assert_eq!(part1(&filesystem, &mut FxHashMap::default()), 95437);
    }

    #[test]
    fn to_delete() {
        let filesystem: FileSystem = TEST_DATA.parse().unwrap();
        assert_eq!(part2(&filesystem, &mut FxHashMap::default()), 24933642);
    }
}
