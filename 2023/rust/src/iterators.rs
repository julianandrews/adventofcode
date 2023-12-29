/// Iterate over unique pairs of items in the slice.
///
/// This could be replaced with Itertools::combinations, but has the advantage of avoiding `Vec`
/// allocations which are unavoidable for the generic version.
pub fn iter_pairs<T>(items: &[T]) -> PairIterator<T> {
    PairIterator { items, i: 0, j: 1 }
}

#[derive(Debug, Clone)]
pub struct PairIterator<'a, T> {
    items: &'a [T],
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for PairIterator<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.items.get(self.i)?;
        let b = self.items.get(self.j)?;
        self.j += 1;
        if self.j >= self.items.len() {
            self.i += 1;
            self.j = self.i + 1;
        }
        Some((a, b))
    }
}

pub trait AocIterators: Iterator {
    /// Consume the iterator and return all elements if there are exactly N.
    fn exactly_n<const N: usize>(mut self) -> Option<[Self::Item; N]>
    where
        Self: Sized,
    {
        let first_n = self.by_ref().take(N).collect::<Vec<_>>().try_into().ok()?;
        match self.next() {
            Some(_) => None,
            None => Some(first_n),
        }
    }
}

impl<I: Iterator> AocIterators for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_iterator() {
        let values = vec![1, 2, 3, 4];
        let result: Vec<(u64, u64)> = iter_pairs(&values).map(|(&a, &b)| (a, b)).collect();
        let expected = [(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)];

        assert_eq!(result, expected);
    }
}
