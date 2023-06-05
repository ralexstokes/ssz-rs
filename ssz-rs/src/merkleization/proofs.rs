use sha2::{Digest, Sha256};

use crate::merkleization::Node;

/// `is_valid_merkle_branch` verifies the Merkle proof
/// against the `root` given the other metadata.
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
            hasher.update(next_node.as_ref());
            hasher.update(value.as_ref());
        } else {
            hasher.update(value.as_ref());
            hasher.update(next_node.as_ref());
        }
        value.as_mut().copy_from_slice(&hasher.finalize_reset());
    }
    value == *root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_node_from_hex(hex: &str) -> Node {
        let bytes = hex::decode(hex).expect("is hex");
        Node::try_from(bytes.as_ref()).expect("is right size")
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
