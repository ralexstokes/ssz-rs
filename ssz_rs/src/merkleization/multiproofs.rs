use crate::merkleization::{GeneralizedIndex, Node};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

fn get_branch_indices(tree_index: &GeneralizedIndex) -> Vec<GeneralizedIndex> {
    let mut focus = tree_index.sibling();
    let mut result = vec![focus.clone()];
    while focus.0 > 1 {
        focus = focus.parent().sibling();
        result.push(focus.clone());
    }
    result.truncate(result.len() - 1);
    result
}

fn get_path_indices(tree_index: &GeneralizedIndex) -> Vec<GeneralizedIndex> {
    let mut focus = *tree_index;
    let mut result = vec![focus.clone()];
    while focus.0 > 1 {
        focus = focus.parent();
        result.push(focus.clone());
    }
    result.truncate(result.len() - 1);
    result
}

fn get_helper_indices(indices: &[GeneralizedIndex]) -> Vec<GeneralizedIndex> {
    let mut all_helper_indices = HashSet::new();
    let mut all_path_indices = HashSet::new();

    for index in indices {
        all_helper_indices.extend(get_branch_indices(index).iter());
        all_path_indices.extend(get_path_indices(index).iter());
    }

    let mut all_branch_indices = all_helper_indices
        .difference(&all_path_indices)
        .cloned()
        .collect::<Vec<_>>();
    all_branch_indices.sort_by(|a: &GeneralizedIndex, b: &GeneralizedIndex| b.cmp(a));
    all_branch_indices
}

pub fn calculate_merkle_root(leaf: &Node, proof: &[Node], index: &GeneralizedIndex) -> Node {
    debug_assert_eq!(proof.len(), index.get_path_length());
    let mut result = *leaf;

    let mut hasher = Sha256::new();
    for (i, next) in proof.iter().enumerate() {
        if index.get_bit(i) {
            hasher.update(&next.0);
            hasher.update(&result.0);
        } else {
            hasher.update(&result.0);
            hasher.update(&next.0);
        }
        result.0.copy_from_slice(&hasher.finalize_reset());
    }
    result
}

pub fn verify_merkle_proof(
    leaf: &Node,
    proof: &[Node],
    index: &GeneralizedIndex,
    root: &Node,
) -> bool {
    &calculate_merkle_root(leaf, proof, index) == root
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
        let sibling_present = objects.contains_key(&key.sibling());
        let parent_index = key.parent();
        let parent_missing = !objects.contains_key(&parent_index);
        let should_compute = key_present && sibling_present && parent_missing;
        if should_compute {
            let right_index = GeneralizedIndex(key.0 | 1);
            let left_index = right_index.sibling();
            let left_input = objects.get(&left_index).unwrap();
            let right_input = objects.get(&right_index).unwrap();
            hasher.update(&left_input.0);
            hasher.update(&right_input.0);

            let parent = objects
                .entry(parent_index)
                .or_insert_with(|| Node::default());
            parent.0.copy_from_slice(&hasher.finalize_reset());
            keys.push(parent_index);
        }
        pos += 1;
    }

    objects.get(&GeneralizedIndex(1)).unwrap().clone()
}

pub fn verify_merkle_multiproof(
    leaves: &[Node],
    proof: &[Node],
    indices: &[GeneralizedIndex],
    root: &Node,
) -> bool {
    &calculate_multi_merkle_root(leaves, proof, indices) == root
}
