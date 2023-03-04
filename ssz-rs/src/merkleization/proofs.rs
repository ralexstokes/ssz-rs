use crate::{
    field_inspect::FieldsIterMut,
    lib::*,
    merkleization::{merkleize_to_virtual_tree, GeneralizedIndex, Node},
    ElementsType, MerkleizationError, SimpleSerialize, SszReflect, SszTypeClass,
};
use alloc::collections::{BTreeMap, BTreeSet};
use sha2::{Digest, Sha256};

pub fn is_valid_merkle_branch<'a>(
    leaf: &Node,
    mut branch: impl Iterator<Item = &'a Node>,
    depth: usize,
    index: usize,
    root: &Node,
) -> bool {
    let mut value = *leaf;

    let mut hasher = Sha256::new();
    for i in 0..depth {
        let next_node = match branch.next() {
            Some(node) => node,
            None => return false,
        };
        if (index / 2usize.pow(i as u32)) % 2 != 0 {
            hasher.update(next_node.0);
            hasher.update(value.0);
        } else {
            hasher.update(value.0);
            hasher.update(next_node.0);
        }
        value.0.copy_from_slice(&hasher.finalize_reset());
    }
    value == *root
}

/// Generates a proof for potentially multiple elements in an SszObject.
pub fn generate_proof<T: SimpleSerialize + SszReflect>(
    mut data: T,
    indices: &[usize],
) -> Result<Vec<Node>, MerkleizationError> {
    // first merklize the data, return a virtual tree that maps the generalized index to the node
    // next calculate the required proof indices for given indices to prove.
    // return the nodes for those proof indices.
    let type_class = data.ssz_type_class();
    let leaves = match type_class {
        SszTypeClass::Basic | SszTypeClass::Union => Err(MerkleizationError::CannotMerkleize)?,
        SszTypeClass::Container => {
            let fields = data.as_mut_field_inspectable().expect("SszTypeClass is a container; qed");
            let mut leaves = vec![];

            for (_, (_, field)) in FieldsIterMut::new(fields).enumerate() {
                let leaf = field.hash_tree_root()?;
                leaves.push(leaf);
            }

            leaves
        }
        SszTypeClass::Elements(_) | SszTypeClass::Bits(_) => {
            let iterator = data.list_iterator_mut().expect("Type class declared as elements; qed");
            let mut leaves = vec![];

            for elem in iterator {
                // todo: check for composite types?
                let leaf = elem.hash_tree_root()?;
                leaves.push(leaf);
            }

            leaves
        }
    };

    let virtual_tree = merkleize_to_virtual_tree(leaves);
    let indices = indices.into_iter().cloned().map(GeneralizedIndex).collect::<Vec<_>>();
    let proof_indices = get_helper_indices(&indices);
    let mut proof = Vec::new();

    for GeneralizedIndex(index) in proof_indices {
        proof.push(virtual_tree[index].clone())
    }

    if matches!(
        type_class,
        SszTypeClass::Bits(ElementsType::Vector) | SszTypeClass::Elements(ElementsType::Vector)
    ) {
        // todo: mix in the length.
    }

    Ok(proof)
}

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
    let mut all_helper_indices = BTreeSet::new();
    let mut all_path_indices = BTreeSet::new();

    for index in indices {
        all_helper_indices.extend(get_branch_indices(index).iter());
        all_path_indices.extend(get_path_indices(index).iter());
    }

    let mut all_branch_indices =
        all_helper_indices.difference(&all_path_indices).cloned().collect::<Vec<_>>();
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

    let mut objects = BTreeMap::new();
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

            let parent = objects.entry(parent_index).or_insert_with(|| Node::default());
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

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_node_from_hex(hex: &str) -> Node {
        Node::from_bytes(hex::decode(hex).expect("is hex").try_into().expect("is right size"))
    }

    #[test]
    fn test_basic_proof() {
        let leaf = decode_node_from_hex(
            "94159da973dfa9e40ed02535ee57023ba2d06bad1017e451055470967eb71cd5",
        );
        let branch = [
            "8f594dbb4f4219ad4967f86b9cccdb26e37e44995a291582a431eef36ecba45c",
            "f8c2ed25e9c31399d4149dcaa48c51f394043a6a1297e65780a5979e3d7bb77c",
            "382ba9638ce263e802593b387538faefbaed106e9f51ce793d405f161b105ee6",
            "c78009fdf07fc56a11f122370658a353aaa542ed63e44c4bc15ff4cd105ab33c",
        ]
        .into_iter()
        .map(decode_node_from_hex)
        .collect::<Vec<_>>();
        let depth = 3;
        let index = 2;
        let root = decode_node_from_hex(
            "27097c728aade54ff1376d5954681f6d45c282a81596ef19183148441b754abb",
        );

        assert!(is_valid_merkle_branch(&leaf, branch.iter(), depth, index, &root))
    }
}
