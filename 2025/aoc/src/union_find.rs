use std::collections::HashMap;

pub struct UnionFind {
    indicies: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let mut indicies = Vec::with_capacity(size);
        for i in 0..size {
            indicies.push(i);
        }
        Self { indicies, sizes: vec![1; size] } //, maxes: [0; 3] }
    }

    pub fn find(&mut self, i: usize) -> usize {
        let mut curr = i;
        while curr != self.indicies[curr] {
            let parent = self.indicies[curr];
            self.indicies[curr] = self.indicies[parent];
            curr = parent;
        }
        curr
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let parent_i = self.find(i);
        let parent_j = self.find(j);
        if parent_i == parent_j {
            return;
        }
        self.indicies[parent_i] = parent_j;
        self.sizes[parent_j] += self.sizes[parent_i];
    }

    pub fn size(&mut self, i: usize) -> usize {
        let parent_i = self.find(i);
        self.sizes[parent_i]
    }

    pub fn indicies(&self) -> &Vec<usize> {
        &self.indicies
    }

    pub fn sizes(&mut self) -> HashMap<usize, usize> {
        let mut sizes = HashMap::new();
        for i in 0..self.indicies.len() {
            let parent = self.find(i);
            if !sizes.contains_key(&parent) {
                sizes.insert(parent, self.size(parent));
            }
        }
        sizes
    }

    pub fn all_connected(&mut self) -> bool {
        let parent = self.find(0);
        for i in 1..self.indicies.len() {
            if self.find(i) != parent {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_union_find() {
    let size = 10;
    let mut uf = UnionFind::new(size);

    for i in 0..size {
        assert_eq!(uf.find(i), i);
        assert_eq!(uf.size(i), 1);
    }

    uf.union(1, 2);
    assert_eq!(uf.find(1), 2);
    assert_eq!(uf.size(1), 2);

    uf.union(2, 3);
    assert_eq!(uf.find(2), 3);
    assert_eq!(uf.size(2), 3);

    uf.union(3, 4);
    assert_eq!(uf.find(3), 4);
    assert_eq!(uf.find(1), 4);

    assert_eq!(uf.size(4), 4);
    assert_eq!(uf.size(1), 4);
}