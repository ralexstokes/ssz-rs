use crate::merkleization::{
    generalized_index::log_2, GeneralizedIndex, MerkleizationError as Error, Node,
};
use sha2::{Digest, Sha256};

pub fn get_subtree_index(i: GeneralizedIndex) -> Result<usize, Error> {
    let i_log2 = log_2(i).ok_or(Error::InvalidGeneralizedIndex)?;
    Ok(i % 2usize.pow(i_log2))
}

/// `is_valid_merkle_branch` verifies the Merkle proof
/// against the `root` given the other metadata.
pub fn is_valid_merkle_branch<T: AsRef<[u8]>>(
    leaf: Node,
    branch: &[T],
    depth: usize,
    index: usize,
    root: Node,
) -> Result<(), Error> {
    if branch.len() != depth {
        return Err(Error::InvalidProof)
    }

    let mut derived_root = leaf;
    let mut hasher = Sha256::new();

    for (i, node) in branch.iter().enumerate() {
        let node = Node::try_from(node.as_ref()).map_err(|_| Error::InvalidProof)?;

        if (index / 2usize.pow(i as u32)) % 2 != 0 {
            hasher.update(node.as_ref());
            hasher.update(derived_root.as_ref());
        } else {
            hasher.update(derived_root.as_ref());
            hasher.update(node.as_ref());
        }
        derived_root.copy_from_slice(&hasher.finalize_reset());
    }

    if derived_root == root {
        Ok(())
    } else {
        Err(Error::InvalidProof)
    }
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
        ]
        .into_iter()
        .map(|str| hex::decode(str).expect("is valid"))
        .collect::<Vec<_>>();
        let depth = 3;
        let index = 2;
        let root = decode_node_from_hex(
            "27097c728aade54ff1376d5954681f6d45c282a81596ef19183148441b754abb",
        );

        assert!(is_valid_merkle_branch(leaf, &branch, depth, index, root).is_ok());
    }
}
