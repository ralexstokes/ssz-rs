use super::hash_nodes;
use crate::merkleization::{MerkleizationError, Node, BYTES_PER_CHUNK, CONTEXT};
use bitvec::prelude::{bitvec, BitVec};
use sha2::{Digest, Sha256};
use std::ops::{Deref, DerefMut};

type GeneralizedIndex = usize;

const ROOT_GENERALIZED_INDEX: GeneralizedIndex = 1;

#[derive(Debug, Clone, Copy)]
enum TreeNode {
    Zero(usize),
    Data(Node),
}

impl TreeNode {
    fn as_bytes(&self) -> &[u8] {
        match self {
            TreeNode::Zero(index) => &CONTEXT[*index],
            TreeNode::Data(node) => &node.0,
        }
    }

    fn as_mut_bytes(&mut self) -> &mut [u8] {
        match self {
            TreeNode::Zero(_) => {
                unreachable!("this variant does not have children so cannot be a parent")
            }
            TreeNode::Data(node) => &mut node.0,
        }
    }
}

impl Default for TreeNode {
    fn default() -> Self {
        Self::Zero(0)
    }
}

/// `Cache` maintains a Merkle tree optimized for generating proofs
/// against arbitrary elements using "generalized indices".
///
/// For background on "generalized indices", see:
/// https://github.com/ethereum/consensus-specs/blob/dev/ssz/merkle-proofs.md
///
/// The `tree` is a dense `Vec` of nodes where a "zero" variant of the node
/// represents a perfect binary (sub)tree where the entire leaf set are just
/// "chunks" of `0x0 * BYTES_PER_CHUNK`.
/// A "chunk" is a leaf in the (full) tree that is materialized as the "bottom"
/// layer. Leaves in the `tree` that are "zero" nodes are handle "virtually" and
/// simply "summarized" by the aforementioned "zero" nodes.
///
/// Thus, a tree generally has the following layout:
/// [ ROOT | foliage, with zero nodes as necessary | chunks ]
/// where the "foliage" are the parent nodes generated from Merklizing
/// the `chunks` with "zero" ndoes interspersed as necessary to represent
/// perfect binary trees of all "zero" nodes
///
/// The `tree` is indexed by the "generalized index", with an offset of 1
/// to avoid allocating a node at index 0 that is never used.
#[derive(Default, Debug, Clone)]
pub struct Cache {
    dirty_chunks: BitVec,
    chunks_start: GeneralizedIndex,
    tree: Vec<TreeNode>,
    hasher: Sha256,
}

fn compute_tree_size(chunk_count: usize, leaf_count: usize) -> usize {
    let materialized_leaves = if chunk_count % 2 == 0 {
        chunk_count
    } else {
        chunk_count + 1
    };
    let zero_leaves = leaf_count - chunk_count;
    let chunk_count = 2 * leaf_count - 1;
    BYTES_PER_CHUNK * chunk_count
}

// NOTE: we skip storing the "extra" node in the tree backing at tree index 0
fn storage_index_from(generalized_index: GeneralizedIndex) -> usize {
    generalized_index - 1
}

fn update_branch(hasher: &mut Sha256, tree: &mut Vec<TreeNode>, mut focus: GeneralizedIndex) {
    while focus != 1 {
        let (left, right) = if focus % 2 == 0 {
            let left_index = storage_index_from(focus);
            let left = &tree[left_index];
            let sibling = focus ^ 1;
            let right_index = storage_index_from(sibling);
            let right = &tree[right_index];
            (left, right)
        } else {
            let right_index = storage_index_from(focus);
            let right = &tree[right_index];
            let sibling = focus ^ 1;
            let left_index = storage_index_from(sibling);
            let left = &tree[left_index];
            (left, right)
        };
        focus = focus / 2;
        let parent_index = storage_index_from(focus);
        let parent = &mut tree[parent_index];
        hash_nodes(
            hasher,
            left.as_bytes(),
            right.as_bytes(),
            parent.as_mut_bytes(),
        );
    }
}

impl Cache {
    pub fn with_chunks(chunk_count: usize) -> Self {
        let leaf_count = chunk_count.next_power_of_two();
        Self::new(chunk_count, leaf_count)
    }

    fn new(chunk_count: usize, leaf_count: usize) -> Self {
        // TODO
        // return zero nodes and position
        // along w/ tree layout
        let tree_size = compute_tree_size(chunk_count, leaf_count);
        Self {
            dirty_chunks: bitvec![1; chunk_count],
            chunks_start: 0,
            tree: vec![TreeNode::default(); tree_size],
            hasher: Sha256::new(),
        }
    }

    fn generalized_index_for(&self, chunk_index: usize) -> GeneralizedIndex {
        self.chunks_start + chunk_index
    }

    pub fn update<C>(&mut self, mut chunk_provider: C) -> Result<(), MerkleizationError>
    where
        C: FnMut(usize) -> Result<Node, MerkleizationError>,
    {
        for chunk_index in self.dirty_chunks.iter_ones() {
            // update chunk
            let chunk = chunk_provider(chunk_index)?;
            let generalized_index = self.generalized_index_for(chunk_index);
            let storage_index = storage_index_from(generalized_index);
            self.tree[storage_index] = TreeNode::Data(chunk);

            // TODO: batch all updates into one stage
            update_branch(&mut self.hasher, &mut self.tree, generalized_index);
        }
        self.reset_validation();
        Ok(())
    }

    pub fn is_stale(&self) -> bool {
        // let has_dirty_chunks = self.dirty_chunks.any();
        // let did_resize = self.leaf_count != self.dirty_chunks.len();
        // !(has_dirty_chunks || did_resize)

        // TODO: compute actual validity
        true
    }

    pub fn invalidate(&mut self, chunk_index: usize) {
        let mut bit = self
            .dirty_chunks
            .get_mut(chunk_index)
            .expect("caller forbids out of bounds");
        *bit = true;
    }

    fn reset_validation(&mut self) {
        self.dirty_chunks.set_elements(0);
    }

    fn get_node_at_generalized_index(&self, index: GeneralizedIndex) -> Node {
        let storage_index = storage_index_from(index);
        match self.tree[storage_index] {
            TreeNode::Zero(index) => CONTEXT[index].try_into().unwrap(),
            TreeNode::Data(node) => node,
        }
    }

    pub fn root(&mut self) -> Node {
        debug_assert!(!self.is_stale());
        self.get_node_at_generalized_index(ROOT_GENERALIZED_INDEX)
    }
}

#[derive(Default, Debug, Clone)]
pub struct CacheWithLimit {
    cache: Cache,
    length_leaf: Node,
}

impl CacheWithLimit {
    pub fn new(chunk_count: usize, limit: usize) -> Result<Self, MerkleizationError> {
        if limit < chunk_count {
            return Err(MerkleizationError::InputExceedsLimit(limit));
        }

        let leaf_count = limit.next_power_of_two();
        Ok(Self {
            cache: Cache::new(chunk_count, leaf_count),
            ..Default::default()
        })
    }

    pub fn resize(&mut self, bound: usize) {
        self.dirty_chunks.resize(bound, true);
    }
}

impl Deref for CacheWithLimit {
    type Target = Cache;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl DerefMut for CacheWithLimit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cache
    }
}
