use std::hash::Hash;

use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct Cycle {
    pub start: usize,
    pub length: usize,
}

pub fn cycle_detect<T>(it: impl Iterator<Item = T>) -> Option<Cycle>
where
    T: Eq + Hash,
{
    let mut seen = FxHashMap::default();
    for (i, value) in it.enumerate() {
        if let Some(j) = seen.get(&value) {
            return Some(Cycle {
                start: *j,
                length: i - j,
            });
        }
        seen.insert(value, i);
    }

    None
}
