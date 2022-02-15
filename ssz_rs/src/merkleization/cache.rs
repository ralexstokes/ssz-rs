use crate::merkleization::Node;
use bitvec::prelude::{BitVec};

#[derive(Default, Debug, Clone)]
pub struct Cache {
    leaf_count: usize,
    dirty_leaves: BitVec,
    root: Node,
}

impl Cache {
    pub fn with_leaves(leaf_count: usize) -> Self {
        // ensure dirty_leaves is length of leaf_count
        // and initialized to ones
        let mut dirty_leaves = BitVec::new();
        for _i in 0..leaf_count {
            dirty_leaves.push(true);
        }
        // quick checks (panic if not equal)
        // 1) is length ok?
        // 2) all elems == 1?
        assert_eq!(dirty_leaves.len(), leaf_count);
        assert_eq!(leaf_count, dirty_leaves.count_ones());

        // pass dirty_leaves to Self, return Self
        Self {
            leaf_count,
            dirty_leaves,
            ..Default::default()
        }
    }

    pub fn valid(&self) -> bool {
        let has_dirty_leaves = self.dirty_leaves.any();
        let did_resize = self.leaf_count != self.dirty_leaves.len();
        !(has_dirty_leaves || did_resize)
    }

    pub fn invalidate(&mut self, leaf_index: usize) {
        if let Some(mut bit) = self.dirty_leaves.get_mut(leaf_index) {
            // TODO: unconditionally access bit
            *bit = true;
        }
    }

    pub fn resize(&mut self, bound: usize) {
        self.dirty_leaves.resize(bound, true);
    }

    pub fn update(&mut self, root: Node) {
        self.root = root;
    }

    pub fn root(&self) -> Node {
        self.root
    }
}
