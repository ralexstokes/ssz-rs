//! Support for constructing and verifying Merkle proofs.
use crate::{
    lib::*,
    merkleization::{
        compute_merkle_tree, generalized_index::log_2, GeneralizedIndex, GeneralizedIndexable,
        MerkleizationError as Error, Node, Path,
    },
};
use sha2::{Digest, Sha256};

pub type ProofAndWitness = (Proof, Node);

fn get_depth(i: GeneralizedIndex) -> Result<u32, Error> {
    log_2(i).ok_or(Error::InvalidGeneralizedIndex)
}

fn get_index(i: GeneralizedIndex, depth: u32) -> usize {
    i % 2usize.pow(depth)
}

pub fn get_subtree_index(i: GeneralizedIndex) -> Result<usize, Error> {
    let depth = get_depth(i)?;
    Ok(get_index(i, depth))
}

// Identify the generalized index that is the largest parent of `i` that fits in a perfect binary
// tree with `leaf_count` leaves. Return this index along with its depth in the tree
// and its index in the leaf layer.
pub fn compute_local_merkle_coordinates(
    mut i: GeneralizedIndex,
    leaf_count: usize,
) -> Result<(u32, usize, GeneralizedIndex), Error> {
    let node_count = 2 * leaf_count - 1;
    while i > node_count {
        i /= 2;
    }
    let depth = get_depth(i)?;
    Ok((depth, get_index(i, depth), i))
}

#[derive(Debug)]
pub struct Prover {
    hasher: Sha256,
    proof: Proof,
    witness: Node,
}

impl Prover {
    fn set_leaf(&mut self, leaf: &[u8]) {
        self.proof.leaf = leaf.try_into().expect("is correct size");
    }

    // Adds a node to the Merkle proof's branch.
    // Assumes nodes are provided going from the bottom of the tree to the top.
    fn extend_branch(&mut self, node: &[u8]) {
        self.proof.branch.push(node.try_into().expect("is correct size"))
    }

    fn set_witness(&mut self, witness: &[u8]) {
        self.witness = witness.try_into().expect("is correct size");
    }

    /// Derive a Merkle proof relative to `data` given the parameters in `self`.
    pub fn compute_proof<T: Prove>(&mut self, data: &mut T) -> Result<(), Error> {
        let chunk_count = T::chunk_count();
        let leaf_count = chunk_count.next_power_of_two();
        let parent_index = self.proof.index;
        let (local_depth, local_index, local_generalized_index) =
            compute_local_merkle_coordinates(parent_index, leaf_count)?;

        let mut is_leaf_local = false;
        if local_generalized_index < parent_index {
            // NOTE: need to recurse to children to find ultimate leaf
            let child_index = if parent_index % 2 == 0 {
                parent_index / local_generalized_index
            } else {
                parent_index / local_generalized_index + 1
            };
            self.proof.index = child_index;
            let child = data.inner_element(local_index)?;
            self.compute_proof(child)?;
            self.proof.index = parent_index;
        } else {
            // NOTE: leaf is within the current object, set a flag to grab from merkle tree later
            is_leaf_local = true;
        }
        let chunks = data.chunks()?;
        let tree = compute_merkle_tree(&mut self.hasher, &chunks, leaf_count)?;

        if is_leaf_local {
            self.set_leaf(&tree[parent_index]);
        }

        let mut target = local_generalized_index;
        for _ in 0..local_depth {
            let sibling = if target % 2 != 0 { &tree[target - 1] } else { &tree[target + 1] };
            self.extend_branch(sibling);
            target /= 2;
        }

        self.set_witness(&tree[1]);

        Ok(())
    }
}

impl From<Prover> for ProofAndWitness {
    fn from(value: Prover) -> Self {
        (value.proof, value.witness)
    }
}

impl From<GeneralizedIndex> for Prover {
    fn from(index: GeneralizedIndex) -> Self {
        Self {
            hasher: Sha256::new(),
            proof: Proof { leaf: Default::default(), branch: vec![], index },
            witness: Default::default(),
        }
    }
}

/// Required functionality to support computing Merkle proofs.
pub trait Prove: GeneralizedIndexable {
    type InnerElement: Prove;

    /// Compute the "chunks" of this type as required for the SSZ merkle tree computation.
    /// Default implementation signals an error. Implementing types should override
    /// to provide the correct behavior.
    fn chunks(&mut self) -> Result<Vec<u8>, Error> {
        Err(Error::NotChunkable)
    }

    /// Provide a reference to a member element of a composite type.
    /// Default implementation signals an error. Implementing types should override
    /// to provide the correct behavior.
    fn inner_element(&mut self, _index: usize) -> Result<&mut Self::InnerElement, Error> {
        Err(Error::NoInnerElement)
    }
}

// Implement `GeneralizedIndexable` for `()` for use as a marker type in `Prove`.
impl GeneralizedIndexable for () {
    fn compute_generalized_index(
        _parent: GeneralizedIndex,
        path: Path,
    ) -> Result<GeneralizedIndex, Error> {
        Err(Error::InvalidPath(path.to_vec()))
    }
}

// Implement the default `Prove` functionality for use of `()` as a marker type.
impl Prove for () {
    type InnerElement = ();
}

/// Produce a Merkle proof (and corresponding witness) for the type `T` at the given `path` relative
/// to `T`.
pub fn prove<T: Prove>(data: &mut T, path: Path) -> Result<ProofAndWitness, Error> {
    let index = T::generalized_index(path)?;
    let mut prover = Prover::from(index);
    prover.compute_proof(data)?;
    Ok(prover.into())
}

/// Contains data necessary to verify `leaf` was included under some witness "root" node
/// at the generalized position `index`.
#[derive(Debug, PartialEq, Eq)]
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
    use crate::{PathElement, SimpleSerialize, U256};

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
    fn test_proving_primitives_fails_with_bad_path() {
        let mut data = 8u8;
        let result = prove(&mut data, &[PathElement::Length]);
        assert!(result.is_err());

        let mut data = true;
        let result = prove(&mut data, &[234.into()]);
        assert!(result.is_err());
    }

    fn compute_and_verify_proof_for_path<T: SimpleSerialize + Prove>(data: &mut T, path: Path) {
        let (proof, witness) = prove(data, path).unwrap();
        assert_eq!(witness, data.hash_tree_root().unwrap());
        let result = proof.verify(witness);
        assert!(result.is_ok());
    }

    #[test]
    fn test_prove_primitives() {
        let mut data = 8u8;
        compute_and_verify_proof_for_path(&mut data, &[]);

        let mut data = 0u8;
        compute_and_verify_proof_for_path(&mut data, &[]);

        let mut data = 234238u64;
        compute_and_verify_proof_for_path(&mut data, &[]);

        let mut data = 0u128;
        compute_and_verify_proof_for_path(&mut data, &[]);

        let mut data = u128::MAX;
        compute_and_verify_proof_for_path(&mut data, &[]);

        let mut data = U256::from_str_radix(
            "f8c2ed25e9c31399d4149dcaa48c51f394043a6a1297e65780a5979e3d7bb77c",
            16,
        )
        .unwrap();
        compute_and_verify_proof_for_path(&mut data, &[]);

        let mut data = true;
        compute_and_verify_proof_for_path(&mut data, &[]);

        let mut data = false;
        compute_and_verify_proof_for_path(&mut data, &[]);
    }
}
