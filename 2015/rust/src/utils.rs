use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::hash::{BuildHasher, Hash};
use std::io::{self, Read};
use std::str::pattern::Pattern;
use std::str::FromStr;

pub fn get_input() -> std::io::Result<String> {
    let args: Vec<String> = env::args().collect();
    let mut reader: Box<dyn io::Read> = if args.len() == 2 && args[1] != "-" {
        Box::new(io::BufReader::new(File::open(&args[1])?))
    } else {
        Box::new(io::stdin())
    };

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    Ok(input)
}

pub fn parse_fields<'a, P: Pattern<'a>, T: FromStr, C: FromIterator<T>>(
    data: &'a str,
    pat: P,
) -> Result<C, <T as FromStr>::Err> {
    data.split(pat).map(&str::parse).collect()
}

/// Take a map of maps and encode the values in vectors for efficient lookups.
///
/// Returns a vector of the keys, and a vector of vectors with the values indexed by position in
/// the keys vector.
pub fn build_index_map<K, V, F, H1, H2>(
    mut map: HashMap<K, HashMap<K, V, H2>, H1>,
    mut default: F,
) -> (Vec<K>, Vec<Vec<V>>)
where
    K: Clone + Eq + Hash,
    F: FnMut() -> V,
    H1: BuildHasher,
    H2: BuildHasher,
{
    let keys: Vec<_> = map.keys().cloned().collect();
    let mut values: Vec<Vec<V>> = (0..keys.len())
        .map(|_| (0..keys.len()).map(|_| default()).collect())
        .collect();
    for (i, k1) in keys.iter().enumerate() {
        let m = map.get_mut(k1).unwrap();
        for (j, _) in keys.iter().enumerate() {
            if let Some(v) = m.remove(&keys[j]) {
                values[i][j] = v
            }
        }
    }
    (keys, values)
}
