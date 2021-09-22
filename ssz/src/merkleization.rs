use sha2::{Digest, Sha256};
use std::convert::TryInto;

use crate::ser::{Serialize, SerializeError};
use thiserror::Error;

pub const BYTES_PER_CHUNK: usize = 32;

pub const ZERO_CHUNK: &[u8] = &[0; BYTES_PER_CHUNK];

pub type Root = [u8; 32];

pub(crate) type Chunk = Vec<u8>;

pub trait Merkleized {
    fn chunk_count(&self) -> usize;

    fn hash_tree_root(&self) -> Result<Root, MerkleizationError>;
}

#[derive(Error, Debug)]
#[error("the value could not be merkleized: {0}")]
pub enum MerkleizationError {
    #[error("failed to serialize value: {0}")]
    SerializationError(#[from] SerializeError),
    #[error("cannot merkleize a partial chunk of length {1} (data: {0:?})")]
    PartialChunk(Vec<u8>, usize),
    #[error("cannot merkleize data that exceeds the declared limit {0}")]
    InputExceedsLimit(usize),
}

pub(crate) fn pack_bytes(mut buffer: Vec<u8>) -> Vec<Chunk> {
    let data_len = buffer.len();
    if data_len % BYTES_PER_CHUNK != 0 {
        let bytes_to_pad = BYTES_PER_CHUNK - data_len % BYTES_PER_CHUNK;
        for _ in 0..bytes_to_pad {
            buffer.push(0u8);
        }
    }
    let mut result = vec![];
    for chunk in buffer.chunks_exact(BYTES_PER_CHUNK) {
        result.push(chunk.to_vec());
    }
    result
}

pub(crate) fn pack<T>(values: &[T]) -> Result<Vec<Chunk>, MerkleizationError>
where
    T: Serialize,
{
    let mut buffer = vec![];
    for value in values {
        value.serialize(&mut buffer)?;
    }
    Ok(pack_bytes(buffer))
}

fn hash_nodes(a: &[u8], b: &[u8]) -> Chunk {
    let mut hasher = Sha256::new();
    hasher.update(a);
    hasher.update(b);
    hasher.finalize().to_vec()
}

fn merkleize_chunks(chunks: &[Chunk], leaf_count: usize) -> Result<Root, MerkleizationError> {
    // NOTE: naive implementation, can make much more efficient
    // NOTE: invariants to this function:
    debug_assert!(leaf_count.next_power_of_two() == leaf_count);
    debug_assert!(leaf_count >= 2);

    let node_count = 2 * leaf_count - 1;
    let interior_count = node_count - leaf_count;
    let leaf_start = interior_count * BYTES_PER_CHUNK;

    let mut buffer = vec![0u8; node_count * BYTES_PER_CHUNK];
    debug_assert!(node_count * BYTES_PER_CHUNK == buffer.len());
    for (i, chunk) in chunks.iter().enumerate() {
        let start = leaf_start + (i * BYTES_PER_CHUNK);
        let end = leaf_start + (i + 1) * BYTES_PER_CHUNK;
        buffer[start..end].copy_from_slice(chunk);
    }
    for i in chunks.len()..leaf_count {
        let start = leaf_start + (i * BYTES_PER_CHUNK);
        let end = leaf_start + (i + 1) * BYTES_PER_CHUNK;
        buffer[start..end].copy_from_slice(ZERO_CHUNK);
    }

    for i in (1..node_count).rev().step_by(2) {
        let left_range = (i - 1) * BYTES_PER_CHUNK..(i) * BYTES_PER_CHUNK;
        let right_range = (i) * BYTES_PER_CHUNK..(i + 1) * BYTES_PER_CHUNK;
        let left = &buffer[left_range];
        let right = &buffer[right_range];
        let parent = hash_nodes(left, right);
        let parent_index = (i - 1) / 2;
        buffer[parent_index * BYTES_PER_CHUNK..(parent_index + 1) * BYTES_PER_CHUNK]
            .copy_from_slice(&parent);
    }
    Ok(buffer[0..BYTES_PER_CHUNK]
        .try_into()
        .expect("can produce a single root chunk"))
}

pub fn merkleize(chunks: &[Chunk], limit: Option<usize>) -> Result<Root, MerkleizationError> {
    let chunk_count = chunks.len();
    let mut leaf_count = chunk_count.next_power_of_two();
    if let Some(limit) = limit {
        if limit < chunk_count {
            return Err(MerkleizationError::InputExceedsLimit(limit));
        }
        leaf_count = limit.next_power_of_two();
    }
    match chunk_count {
        0 => Ok(Default::default()),
        1 => chunks[0]
            .clone()
            .try_into()
            .map_err(|partial_chunk: Vec<u8>| {
                MerkleizationError::PartialChunk(partial_chunk.clone(), partial_chunk.len())
            }),
        _ => merkleize_chunks(chunks, leaf_count),
    }
}

pub(crate) fn mix_in_length(root: &Root, length: usize) -> Root {
    let mut length_bytes = vec![0; BYTES_PER_CHUNK];
    let length_data = &length.to_le_bytes();
    length_bytes[0..length_data.len()].copy_from_slice(length_data);
    hash_nodes(root, &length_bytes)
        .try_into()
        .expect("can convert to root")
}

pub fn mix_in_selector(root: &Root, selector: usize) -> Root {
    let mut selector_bytes = vec![0; BYTES_PER_CHUNK];
    let selector_data = &selector.to_le_bytes();
    selector_bytes[0..selector_data.len()].copy_from_slice(selector_data);
    hash_nodes(root, &selector_bytes)
        .try_into()
        .expect("can convert to root")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as ssz;
    use crate::prelude::*;
    use hex_literal::hex;
    use ssz_derive::SimpleSerialize;
    use std::iter::FromIterator;

    #[test]
    fn test_packing_basic_types_simple() {
        let b = true;
        let mut expected = vec![0u8; BYTES_PER_CHUNK];
        expected[0] = 1u8;
        let input = &[b];
        let result = pack(input).expect("can pack values");
        assert!(result.len() == 1);
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_packing_basic_types_extended() {
        let b = true;
        let input = &[b, !b, !b, b];
        let result = pack(input).expect("can pack values");

        let mut expected = vec![0u8; BYTES_PER_CHUNK];
        expected[0] = 1u8;
        expected[3] = 1u8;
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_packing_basic_types_multiple() {
        let data = U256([1u8; 32]);
        let input = &[data.clone(), data.clone(), data.clone()];
        let result = pack(input).expect("can pack values");

        let expected = vec![[1u8; 32]; 3];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merkleize() {
        let input = &[];
        let result = merkleize(input, None).expect("can merkle");
        assert_eq!(result, Root::default());

        let b = true;
        let input = &[b];
        let input = pack(input).expect("can pack");
        let result = merkleize(&input, None).expect("can merkle");
        let mut expected = Root::default();
        expected[0] = 1u8;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merkleize_chunks() {
        let chunks = &[ZERO_CHUNK.to_vec(), ZERO_CHUNK.to_vec()];
        let root = merkleize_chunks(chunks, chunks.len()).expect("can merkleize");
        assert_eq!(
            root,
            hex!("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b")
        );

        let chunks = &[[1u8; 32].to_vec(), [1u8; 32].to_vec()];
        let root = merkleize_chunks(chunks, chunks.len()).expect("can merkleize");
        assert_eq!(
            root,
            hex!("7c8975e1e60a5c8337f28edf8c33c3b180360b7279644a9bc1af3c51e6220bf5")
        );

        let chunks = &[[0u8; 32].to_vec()];
        let root = merkleize_chunks(chunks, 4).expect("can merkleize");
        assert_eq!(
            root,
            hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71")
        );

        let chunks = &[[1u8; 32].to_vec()];
        let root = merkleize_chunks(chunks, 4).expect("can merkleize");
        assert_eq!(
            root,
            hex!("29797eded0e83376b70f2bf034cc0811ae7f1414653b1d720dfd18f74cf13309")
        );

        let chunks = &[[2u8; 32].to_vec()];
        let root = merkleize_chunks(chunks, 8).expect("can merkleize");
        assert_eq!(
            root,
            hex!("fa4cf775712aa8a2fe5dcb5a517d19b2e9effcf58ff311b9fd8e4a7d308e6d00")
        );
    }

    #[test]
    fn test_hash_tree_root() {
        #[derive(PartialEq, Eq, Debug, SimpleSerialize, Clone)]
        enum Bar {
            A(u32),
            B(List<bool, 32>),
        }

        #[derive(PartialEq, Eq, Debug, Default, SimpleSerialize, Clone)]
        struct Foo {
            a: u32,
            b: Vector<u32, 4>,
            c: bool,
            d: Bitlist<27>,
            e: Bar,
            f: Bitvector<4>,
        }

        let mut foo = Foo {
            a: 16u32,
            b: Vector::from_iter([3u32, 2u32, 1u32, 10u32]),
            c: true,
            d: Bitlist::from_iter([
                true, false, false, true, true, false, true, false, true, true, false, false, true,
                true, false, true, false, true, true, false, false, true, true, false, true, false,
                true,
            ]),
            e: Bar::B(List::from_iter([true, true, false, false, false, true])),
            f: Bitvector::from_iter([false, true, false, true]),
        };

        let root = foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root,
            hex!("c46234d121c855779cff24e67356fc69b94fb165d3a4611ebb64340c12301b99")
        );

        let original_foo = foo.clone();

        foo.b[2] = 44u32;
        foo.d.pop();
        foo.e = Bar::A(33);

        let root = original_foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root,
            hex!("c46234d121c855779cff24e67356fc69b94fb165d3a4611ebb64340c12301b99")
        );

        let root = foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root,
            hex!("9408e9fc300a11c11ee9e4515de14f19d5aee9e72e0a8a89da60b1e591f6b2e5")
        );

        let encoding = match serialize(&original_foo) {
            Ok(encoding) => encoding,
            Err(e) => {
                eprintln!("some error encoding: {}", e);
                return;
            }
        };

        let mut restored_foo = match Foo::deserialize(&encoding) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("some error decoding: {}", e);
                return;
            }
        };

        let root = restored_foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root,
            hex!("c46234d121c855779cff24e67356fc69b94fb165d3a4611ebb64340c12301b99")
        );

        restored_foo.b[2] = 44u32;
        restored_foo.d.pop();
        restored_foo.e = Bar::A(33);

        let root = foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root,
            hex!("9408e9fc300a11c11ee9e4515de14f19d5aee9e72e0a8a89da60b1e591f6b2e5")
        );
    }
}
