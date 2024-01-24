use anyhow::{anyhow, bail, Result};

fn main() -> Result<()> {
    let input = aoc::utils::get_input()?;
    let weights: Vec<u64> = aoc::utils::parse_fields(input.trim(), '\n')?;

    println!("Part 1: {}", part1(&weights)?);
    println!("Part 2: {}", part2(&weights)?);

    Ok(())
}

fn part1(weights: &[u64]) -> Result<u128> {
    min_split_entanglement(weights, 3)
}

fn part2(weights: &[u64]) -> Result<u128> {
    min_split_entanglement(weights, 4)
}

fn min_split_entanglement(weights: &[u64], groups: u64) -> Result<u128> {
    if weights.iter().any(|&w| w >= 128) {
        bail!("Only weights up to 128 supported by PackageSet. Use something else.");
    }
    package_split(weights, groups)
        .ok_or(anyhow!("Package split not found."))
        .map(|subset| subset.entanglement())
}

fn package_split(weights: &[u64], groups: u64) -> Option<PackageSet> {
    let total: u64 = weights.iter().sum();
    if groups == 0 {
        match total == 0 {
            true => return Some(PackageSet::new()),
            false => return None,
        }
    }
    if total % groups != 0 {
        return None;
    }
    let goal = total / groups;
    let max_len = weights.len() as u64 / groups;
    let mut possible_subsets = subsets_with_sum(weights, goal, max_len);
    possible_subsets.sort_by_key(|subset| (subset.len(), subset.entanglement()));
    for subset in possible_subsets {
        let remaining: Vec<u64> = weights
            .iter()
            .copied()
            .filter(|&w| !subset.contains(w))
            .collect();
        if package_split(&remaining, groups - 1).is_some() {
            return Some(subset);
        }
    }
    None
}

fn subsets_with_sum(items: &[u64], total: u64, max_len: u64) -> Vec<PackageSet> {
    type Cache = Vec<Vec<Option<bool>>>;

    fn p(items: &[u64], s: u64, r: PackageSet, max_len: u64, cache: &mut Cache) -> Vec<PackageSet> {
        if s == 0 {
            return vec![r];
        }
        if r.len() >= max_len {
            return vec![];
        }
        let (item, rest) = match items.split_last() {
            Some((item, rest)) => (*item, rest),
            None => return vec![],
        };
        let mut result = vec![];
        if q(rest, s, cache) {
            result.extend(p(rest, s, r, max_len, cache));
        }

        if s >= item && q(rest, s - item, cache) {
            let mut x = r;
            x.insert(item);
            result.extend(p(rest, s - item, x, max_len, cache));
        }
        result
    }

    fn q(items: &[u64], s: u64, cache: &mut Cache) -> bool {
        if let Some(value) = cache[items.len()][s as usize] {
            return value;
        }
        let value = match items.split_last() {
            Some((&item, rest)) => (s >= item && q(rest, s - item, cache)) || q(rest, s, cache),
            None => s == 0,
        };
        cache[items.len()][s as usize] = Some(value);
        value
    }

    let mut items = items.to_vec();
    items.sort_unstable();
    let mut cache: Cache = vec![vec![None; total as usize + 1]; items.len()];
    p(&items, total, PackageSet::new(), max_len, &mut cache)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PackageSet {
    elements: u128,
    entanglement: u128,
}

impl PackageSet {
    pub fn new() -> PackageSet {
        PackageSet {
            elements: 0,
            entanglement: 1,
        }
    }

    pub fn contains(&self, value: u64) -> bool {
        self.elements & (1 << value) != 0
    }

    pub fn insert(&mut self, value: u64) {
        self.elements |= 1 << value;
        self.entanglement *= value as u128;
    }

    pub fn len(&self) -> u64 {
        self.elements.count_ones() as u64
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        (0..128).filter(|&i| self.contains(i))
    }

    pub fn entanglement(&self) -> u128 {
        self.entanglement
    }
}

impl std::iter::FromIterator<u64> for PackageSet {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut package_set = PackageSet::new();
        for x in iter {
            package_set.insert(x);
        }
        package_set
    }
}

impl std::fmt::Display for PackageSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements: Vec<_> = self.iter().map(|x| x.to_string()).collect();
        write!(f, "{{ {} }}", elements.join(", "))
    }
}

impl std::fmt::Debug for PackageSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PackageSet({})", self)
    }
}

#[cfg(test)]
mod tests {
    use super::{package_split, subsets_with_sum, PackageSet};

    use aoc::testing::assert_matches_ignoring_order;

    #[test]
    fn subset_sum_1() {
        let result = subsets_with_sum(&[1, 2, 3, 4, 5], 5, 2);
        let expected: Vec<PackageSet> = vec![
            [2, 3].into_iter().collect(),
            [1, 4].into_iter().collect(),
            [5].into_iter().collect(),
        ];

        assert_matches_ignoring_order(&result, &expected)
    }

    #[test]
    fn subset_sum_2() {
        let result = subsets_with_sum(&[1, 2, 3, 5, 6], 6, 3);
        let expected: Vec<PackageSet> = vec![
            [1, 5].into_iter().collect(),
            [1, 2, 3].into_iter().collect(),
            [6].into_iter().collect(),
        ];

        assert_matches_ignoring_order(&result, &expected)
    }

    #[test]
    fn subset_sum_3() {
        let result = subsets_with_sum(&[1, 2, 3, 4, 5, 7, 8, 9, 10, 11], 20, 5);
        let expected: Vec<PackageSet> = vec![
            [11, 9].into_iter().collect(),
            [11, 8, 1].into_iter().collect(),
            [11, 7, 2].into_iter().collect(),
            [11, 5, 4].into_iter().collect(),
            [11, 5, 3, 1].into_iter().collect(),
            [11, 4, 3, 2].into_iter().collect(),
            [10, 9, 1].into_iter().collect(),
            [10, 8, 2].into_iter().collect(),
            [10, 7, 3].into_iter().collect(),
            [10, 7, 2, 1].into_iter().collect(),
            [10, 5, 4, 1].into_iter().collect(),
            [10, 5, 3, 2].into_iter().collect(),
            [10, 4, 3, 2, 1].into_iter().collect(),
            [9, 8, 3].into_iter().collect(),
            [9, 8, 2, 1].into_iter().collect(),
            [9, 7, 4].into_iter().collect(),
            [9, 7, 3, 1].into_iter().collect(),
            [9, 5, 4, 2].into_iter().collect(),
            [9, 5, 3, 2, 1].into_iter().collect(),
            [8, 7, 5].into_iter().collect(),
            [8, 7, 4, 1].into_iter().collect(),
            [8, 7, 3, 2].into_iter().collect(),
            [8, 5, 4, 3].into_iter().collect(),
            [8, 5, 4, 2, 1].into_iter().collect(),
            [7, 5, 4, 3, 1].into_iter().collect(),
        ];

        assert_matches_ignoring_order(&result, &expected)
    }

    #[test]
    fn package_split_1() {
        let weights = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let result = package_split(&weights, 3).unwrap();
        let expected: PackageSet = [11, 9].into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn package_split_2() {
        let weights = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let result = package_split(&weights, 4).unwrap();
        let expected: PackageSet = [11, 4].into_iter().collect();
        assert_eq!(result, expected);
    }
}
