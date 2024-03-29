//! Support for constructing and verifying Merkle proofs.
use crate::{
    lib::*,
    merkleization::{
        default_generalized_index, generalized_index::log_2, GeneralizedIndex,
        GeneralizedIndexable, HashTreeRoot, MerkleizationError as Error, Node, Path,
    },
};
use sha2::{Digest, Sha256};

pub type ProofAndWitness = (Proof, Node);

pub fn get_subtree_index(i: GeneralizedIndex) -> Result<usize, Error> {
    let i_log2 = log_2(i).ok_or(Error::InvalidGeneralizedIndex)?;
    Ok(i % 2usize.pow(i_log2))
}

/// Types that can produce Merkle proofs against themselves given a `GeneralizedIndex`.
pub trait Prove {
    /// Provide a Merkle proof of the node in this type's merkle tree corresponding to the `index`.
    fn prove(&mut self, index: GeneralizedIndex) -> Result<ProofAndWitness, Error>;
}

/// Produce a Merkle proof (and corresponding witness) for the type `T` at the given `path` relative
/// to `T`.
pub fn prove<T: GeneralizedIndexable + Prove>(
    data: &mut T,
    path: Path,
) -> Result<ProofAndWitness, Error> {
    let index = T::generalized_index(path)?;
    data.prove(index)
}

/// Contains data necessary to verify `leaf` was included under some witness "root" node
/// at the generalized position `index`.
#[derive(Debug)]
pub struct Proof {
    pub leaf: Node,
    pub branch: Vec<Node>,
    pub index: GeneralizedIndex,
}

impl Proof {
    /// Verify `self` against the provided `root` witness node.
    /// This `root` is the hash tree root of the SSZ object that produced the proof.
    /// See `Prover` for further information.
    pub fn verify(&self, root: Node) -> Result<(), Error> {
        is_valid_merkle_branch_for_generalized_index(self.leaf, &self.branch, self.index, root)
    }
}

pub fn prove_primitive<T: HashTreeRoot + ?Sized>(
    data: &mut T,
    index: GeneralizedIndex,
) -> Result<ProofAndWitness, Error> {
    if index != default_generalized_index() {
        return Err(Error::InvalidGeneralizedIndex)
    }

    let root = data.hash_tree_root()?;
    let proof = Proof { leaf: root, branch: vec![], index };
    Ok((proof, root))
}

pub fn is_valid_merkle_branch_for_generalized_index<T: AsRef<[u8]>>(
    leaf: Node,
    branch: &[T],
    generalized_index: GeneralizedIndex,
    root: Node,
) -> Result<(), Error> {
    let depth = log_2(generalized_index).ok_or(Error::InvalidGeneralizedIndex)? as usize;
    let index = get_subtree_index(generalized_index)?;
    is_valid_merkle_branch(leaf, branch, depth, index, root)
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
    use crate::U256;

    use super::*;

    fn decode_node_from_hex(hex: &str) -> Node {
        let bytes = hex::decode(hex).expect("is hex");
        Node::try_from(bytes.as_ref()).expect("is right size")
    }

    #[test]
    fn test_is_valid_merkle_branch() {
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

    #[test]
    fn test_simple_proof() {
        let leaf = decode_node_from_hex(
            "94159da973dfa9e40ed02535ee57023ba2d06bad1017e451055470967eb71cd5",
        );
        let branch = [
            "8f594dbb4f4219ad4967f86b9cccdb26e37e44995a291582a431eef36ecba45c",
            "f8c2ed25e9c31399d4149dcaa48c51f394043a6a1297e65780a5979e3d7bb77c",
            "382ba9638ce263e802593b387538faefbaed106e9f51ce793d405f161b105ee6",
        ]
        .into_iter()
        .map(decode_node_from_hex)
        .collect::<Vec<_>>();
        let depth = 3;
        let index = 2;
        let proof = Proof { leaf, branch, index: 2usize.pow(depth) + index };
        let root = decode_node_from_hex(
            "27097c728aade54ff1376d5954681f6d45c282a81596ef19183148441b754abb",
        );
        let result = proof.verify(root);
        assert!(result.is_ok());
    }

    #[test]
    fn test_proving_primitives() {
        let mut data = 8u8;
        let (proof, witness) = prove(&mut data, &[]).unwrap();
        assert_eq!(witness, data.hash_tree_root().unwrap());
        let result = proof.verify(witness);
        assert!(result.is_ok());

        let mut data = 234238u64;
        let (proof, witness) = prove(&mut data, &[]).unwrap();
        assert_eq!(witness, data.hash_tree_root().unwrap());
        let result = proof.verify(witness);
        assert!(result.is_ok());

        let mut data = U256::from_str_radix(
            "f8c2ed25e9c31399d4149dcaa48c51f394043a6a1297e65780a5979e3d7bb77c",
            16,
        )
        .unwrap();
        let (proof, witness) = prove(&mut data, &[]).unwrap();
        assert_eq!(witness, data.hash_tree_root().unwrap());
        let result = proof.verify(witness);
        assert!(result.is_ok());

        let mut data = true;
        let (proof, witness) = prove(&mut data, &[]).unwrap();
        assert_eq!(witness, data.hash_tree_root().unwrap());
        let result = proof.verify(witness);
        assert!(result.is_ok())
    }
}
