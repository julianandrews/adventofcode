pub fn permute<E, C: AsRef<[E]> + AsMut<[E]>>(elements: C) -> Permutations<E, C> {
    let n = elements.as_ref().len();
    let state = vec![0; n];
    Permutations {
        elements,
        state,
        i: 0,
        phantom: std::marker::PhantomData,
    }
}

pub struct Permutations<E, C: AsRef<[E]> + AsMut<[E]>> {
    elements: C,
    state: Vec<usize>,
    i: usize,
    phantom: std::marker::PhantomData<E>,
}

impl<E, C: AsRef<[E]> + AsMut<[E]>> Permutations<E, C> {
    pub fn next_perm(&mut self) -> Option<&C> {
        // Heap's algorithm
        if self.i == 0 {
            self.i += 1;
            return Some(&self.elements);
        }
        if self.i >= self.elements.as_ref().len() {
            return None;
        }
        while self.state[self.i] >= self.i {
            self.state[self.i] = 0;
            self.i += 1;
            if self.i >= self.elements.as_ref().len() {
                return None;
            }
        }
        match self.i % 2 == 0 {
            true => self.elements.as_mut().swap(0, self.i),
            false => self.elements.as_mut().swap(self.state[self.i], self.i),
        }
        self.state[self.i] += 1;
        self.i = 1;
        Some(&self.elements)
    }
}

#[cfg(test)]
mod tests {
    use super::{permute, Permutations};

    use crate::testing::assert_matches_ignoring_order;

    #[test]
    fn permutations_of_0() {
        let mut perms: Permutations<u64, _> = permute([]);
        let result: Vec<_> = std::iter::from_fn(move || perms.next_perm().copied()).collect();
        let expected = vec![[]];
        assert_matches_ignoring_order(&result, &expected);
    }

    #[test]
    fn permutations_of_1() {
        let mut perms = permute([1]);
        let result: Vec<_> = std::iter::from_fn(move || perms.next_perm().copied()).collect();
        let expected = vec![[1]];
        assert_matches_ignoring_order(&result, &expected);
    }

    #[test]
    fn permutations_of_2() {
        let mut perms = permute([1, 2]);
        let result: Vec<_> = std::iter::from_fn(move || perms.next_perm().copied()).collect();
        let expected = vec![[1, 2], [2, 1]];
        assert_matches_ignoring_order(&result, &expected);
    }

    #[test]
    fn permutations_of_3() {
        let mut perms = permute([1, 2, 3]);
        let result: Vec<_> = std::iter::from_fn(move || perms.next_perm().copied()).collect();
        #[rustfmt::skip]
        let expected = vec![[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]];
        assert_matches_ignoring_order(&result, &expected);
    }

    #[test]
    fn permutations_of_4() {
        let mut perms = permute([1, 2, 3, 4]);
        let result: Vec<_> = std::iter::from_fn(move || perms.next_perm().copied()).collect();
        #[rustfmt::skip]
        let expected = vec![
            [1, 2, 3, 4], [1, 2, 4, 3], [1, 3, 2, 4], [1, 3, 4, 2], [1, 4, 2, 3], [1, 4, 3, 2],
            [2, 1, 3, 4], [2, 1, 4, 3], [2, 3, 1, 4], [2, 3, 4, 1], [2, 4, 1, 3], [2, 4, 3, 1],
            [3, 1, 2, 4], [3, 1, 4, 2], [3, 2, 1, 4], [3, 2, 4, 1], [3, 4, 1, 2], [3, 4, 2, 1],
            [4, 1, 2, 3], [4, 1, 3, 2], [4, 2, 1, 3], [4, 2, 3, 1], [4, 3, 1, 2], [4, 3, 2, 1],
        ];
        assert_matches_ignoring_order(&result, &expected);
    }
}
