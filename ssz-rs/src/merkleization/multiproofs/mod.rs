mod generalized_index;
mod indexed;

pub use generalized_index::*;
pub use indexed::*;

use crate::merkleization::Node;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

fn get_branch_indices(tree_index: GeneralizedIndex) -> Vec<GeneralizedIndex> {
    let mut focus = sibling(tree_index);
    let mut result = vec![focus.clone()];
    while focus > 1 {
        focus = sibling(parent(focus));
        result.push(focus.clone());
    }
    result.truncate(result.len() - 1);
    result
}

fn get_path_indices(tree_index: GeneralizedIndex) -> Vec<GeneralizedIndex> {
    let mut focus = tree_index;
    let mut result = vec![focus.clone()];
    while focus > 1 {
        focus = parent(focus);
        result.push(focus.clone());
    }
    result.truncate(result.len() - 1);
    result
}

fn get_helper_indices(indices: &[GeneralizedIndex]) -> Vec<GeneralizedIndex> {
    let mut all_helper_indices = HashSet::new();
    let mut all_path_indices = HashSet::new();

    for index in indices {
        all_helper_indices.extend(get_branch_indices(*index).iter());
        all_path_indices.extend(get_path_indices(*index).iter());
    }

    let mut all_branch_indices =
        all_helper_indices.difference(&all_path_indices).cloned().collect::<Vec<_>>();
    all_branch_indices.sort_by(|a: &GeneralizedIndex, b: &GeneralizedIndex| b.cmp(a));
    all_branch_indices
}

pub fn calculate_merkle_root(leaf: &Node, proof: &[Node], index: GeneralizedIndex) -> Node {
    debug_assert_eq!(proof.len(), get_path_length(index));
    let mut result = *leaf;

    let mut hasher = Sha256::new();
    for (i, next) in proof.iter().enumerate() {
        if get_bit(index, i) {
            hasher.update(next.as_ref());
            hasher.update(result.as_ref());
        } else {
            hasher.update(result.as_ref());
            hasher.update(next.as_ref());
        }
        result.as_mut().copy_from_slice(&hasher.finalize_reset());
    }
    result
}

pub fn verify_merkle_proof(
    leaf: &Node,
    proof: &[Node],
    index: GeneralizedIndex,
    root: &Node,
) -> bool {
    calculate_merkle_root(leaf, proof, index) == *root
}

pub fn calculate_multi_merkle_root(
    leaves: &[Node],
    proof: &[Node],
    indices: &[GeneralizedIndex],
) -> Node {
    debug_assert_eq!(leaves.len(), indices.len());
    let helper_indices = get_helper_indices(indices);
    debug_assert_eq!(proof.len(), helper_indices.len());

    let mut objects = HashMap::new();
    for (index, node) in indices.iter().zip(leaves.iter()) {
        objects.insert(*index, *node);
    }
    for (index, node) in helper_indices.iter().zip(proof.iter()) {
        objects.insert(*index, *node);
    }

    let mut keys = objects.keys().cloned().collect::<Vec<_>>();
    keys.sort_by(|a, b| b.cmp(a));

    let mut hasher = Sha256::new();
    let mut pos = 0;
    while pos < keys.len() {
        let key = keys.get(pos).unwrap();
        let key_present = objects.contains_key(key);
        let sibling_present = objects.contains_key(&sibling(*key));
        let parent_index = parent(*key);
        let parent_missing = !objects.contains_key(&parent_index);
        let should_compute = key_present && sibling_present && parent_missing;
        if should_compute {
            let right_index = key | 1;
            let left_index = sibling(right_index);
            let left_input = objects.get(&left_index).unwrap();
            let right_input = objects.get(&right_index).unwrap();
            hasher.update(left_input.as_ref());
            hasher.update(right_input.as_ref());

            let parent = objects.entry(parent_index).or_insert_with(|| Node::default());
            parent.as_mut().copy_from_slice(&hasher.finalize_reset());
            keys.push(parent_index);
        }
        pos += 1;
    }

    objects.get(&1).unwrap().clone()
}

pub fn verify_merkle_multiproof(
    leaves: &[Node],
    proof: &[Node],
    indices: &[GeneralizedIndex],
    root: &Node,
) -> bool {
    &calculate_multi_merkle_root(leaves, proof, indices) == root
}
