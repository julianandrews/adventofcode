#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<u32>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    /// Finds the root of the set containing `x`.
    ///
    /// # Panics
    ///
    /// Panics if `x >= self.len()`.
    pub fn find(&self, x: usize) -> usize {
        let mut current = x;
        while self.parent[current] != current {
            current = self.parent[current];
        }
        current
    }

    /// Finds the root of the set containing `x`, compressing the path.
    ///
    /// This is a mutable version of [`DisjointSet::find`] that performs
    /// path compression, updating parent pointers so that future calls
    /// to either [`find`] or [`find_mut`] will be faster.
    ///
    /// # Panics
    ///
    /// Panics if `x >= self.len()`.
    pub fn find_mut(&mut self, mut x: usize) -> usize {
        let root = self.find(x);
        while self.parent[x] != root {
            let parent = self.parent[x];
            self.parent[x] = root;
            x = parent;
        }
        root
    }

    /// Unions the sets containing `x` and `y`.
    ///
    /// Returns `true` if the sets were separate and have been merged,
    /// or `false` if `x` and `y` were already in the same set.
    ///
    /// # Panics
    ///
    /// Panics if `x >= self.len()` or `y >= self.len()`.
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.find_mut(x);
        let y_root = self.find_mut(y);

        if x_root == y_root {
            return false;
        }

        match self.rank[x_root].cmp(&self.rank[y_root]) {
            std::cmp::Ordering::Less => self.parent[x_root] = y_root,
            std::cmp::Ordering::Greater => self.parent[y_root] = x_root,
            std::cmp::Ordering::Equal => {
                self.parent[y_root] = x_root;
                self.rank[x_root] += 1;
            }
        }

        true
    }

    /// Returns the number of elements in the disjoint set.
    pub fn len(&self) -> usize {
        self.parent.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ds = DisjointSet::new(5);
        assert_eq!(ds.parent, vec![0, 1, 2, 3, 4]);
        assert_eq!(ds.rank, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_find_singletons() {
        let ds = DisjointSet::new(5);
        for i in 0..5 {
            assert_eq!(ds.find(i), i);
        }
        // After find, parent should still point to self (no compression needed)
        assert_eq!(ds.parent, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_union_two_elements() {
        let mut ds = DisjointSet::new(5);

        // Union 0 and 1
        assert!(ds.union(0, 1));
        assert_eq!(ds.find(0), ds.find(1));
        assert_ne!(ds.find(0), 2); // Shouldn't be connected to others

        // Check parent structure
        // Either 0 is parent of 1 or vice versa based on rank
        let root = ds.find(0);
        assert!(root == 0 || root == 1);

        // Try to union again - should return false
        assert!(!ds.union(0, 1));
        assert!(!ds.union(1, 0)); // Should work both ways
    }

    #[test]
    fn test_path_compression() {
        let mut ds = DisjointSet::new(5);

        // Create chain: 0 ← 1 ← 2
        ds.parent = vec![0, 0, 1, 3, 4];
        ds.rank = vec![1, 0, 0, 0, 0];

        // find(2) should compress path
        assert_eq!(ds.find_mut(2), 0);
        // After compression, parent[2] should point directly to 0
        assert_eq!(ds.parent[2], 0);
    }

    #[test]
    fn test_union_by_rank() {
        let mut ds = DisjointSet::new(5);

        // Union 0 and 1 - both rank 0, so one becomes parent
        ds.union(0, 1);
        let root01 = ds.find(0);
        assert_eq!(ds.rank[root01], 1); // Root should have rank 1

        // Union 2 and 3 - both rank 0
        ds.union(2, 3);
        let root23 = ds.find(2);
        assert_eq!(ds.rank[root23], 1);

        // Now union the two trees: one rank 1, one rank 1
        ds.union(0, 2);
        let final_root = ds.find(0);
        assert_eq!(ds.rank[final_root], 2); // Rank should increase to 2
    }

    #[test]
    fn test_multiple_unions() {
        let mut ds = DisjointSet::new(10);

        // Create groups: {0,1,2}, {3,4}, {5,6,7,8}, {9}
        ds.union(0, 1);
        ds.union(1, 2);
        ds.union(3, 4);
        ds.union(5, 6);
        ds.union(6, 7);
        ds.union(7, 8);

        // Verify groups
        assert_eq!(ds.find(0), ds.find(1));
        assert_eq!(ds.find(0), ds.find(2));
        assert_eq!(ds.find(3), ds.find(4));
        assert_eq!(ds.find(5), ds.find(6));
        assert_eq!(ds.find(5), ds.find(7));
        assert_eq!(ds.find(5), ds.find(8));
        assert_eq!(ds.find(9), 9); // Singleton

        // Verify groups are separate
        assert_ne!(ds.find(0), ds.find(3));
        assert_ne!(ds.find(0), ds.find(5));
        assert_ne!(ds.find(0), ds.find(9));
        assert_ne!(ds.find(3), ds.find(5));
    }

    #[test]
    fn test_connected_components_count() {
        let mut ds = DisjointSet::new(7);

        // Start with 7 components
        let mut components = 7;

        // Union 0-1: 6 components
        ds.union(0, 1);
        components -= 1;

        // Union 2-3: 5 components
        ds.union(2, 3);
        components -= 1;

        // Union within same component: should not change count
        ds.union(0, 1);
        // components stays same

        // Union across components: 4 components
        ds.union(1, 2);
        components -= 1;

        // Count unique roots
        let unique_roots: std::collections::HashSet<_> = (0..7).map(|i| ds.find(i)).collect();
        assert_eq!(unique_roots.len(), components);
    }

    #[test]
    fn test_large_chain() {
        let n = 1000;
        let mut ds = DisjointSet::new(n);

        // Create a chain: 0-1-2-...-999
        for i in 0..n - 1 {
            ds.union(i, i + 1);
        }

        // All should have same root
        let root = ds.find(0);
        for i in 1..n {
            assert_eq!(ds.find(i), root);
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_bounds_check_find() {
        let ds = DisjointSet::new(5);
        ds.find(10); // Should panic
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_bounds_check_union() {
        let mut ds = DisjointSet::new(5);
        ds.union(0, 10); // Should panic
    }
}
