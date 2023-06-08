//! NOTE: this module is currently under construction

use crate::merkleization::Node;
use bitvec::prelude::{bitvec, BitVec, Lsb0};

#[derive(Default, Debug, Clone)]
pub struct Cache {
    leaf_count: usize,
    dirty_leaves: BitVec,
    root: Node,
}

impl Cache {
    pub fn with_leaves(leaf_count: usize) -> Self {
        Self { leaf_count, dirty_leaves: bitvec![usize, Lsb0; 1; leaf_count], ..Default::default() }
    }

    pub fn valid(&self) -> bool {
        let has_dirty_leaves = self.dirty_leaves.any();
        let did_resize = self.leaf_count != self.dirty_leaves.len();
        !(has_dirty_leaves || did_resize)
    }

    pub fn invalidate(&mut self, leaf_index: usize) {
        if let Some(mut bit) = self.dirty_leaves.get_mut(leaf_index) {
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
