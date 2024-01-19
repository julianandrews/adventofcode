use std::collections::HashMap;

pub fn assert_matches_ignoring_order<T>(a: &[T], b: &[T])
where
    T: Eq + std::hash::Hash + std::fmt::Debug,
{
    let mut a_counts = HashMap::new();
    for x in a.iter() {
        *a_counts.entry(x).or_insert(0) += 1;
    }
    let mut b_counts = HashMap::new();
    for x in b.iter() {
        *b_counts.entry(x).or_insert(0) += 1;
    }
    assert_eq!(a_counts, b_counts);
}
