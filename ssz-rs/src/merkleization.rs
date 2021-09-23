use sha2::{Digest, Sha256};
use std::convert::TryInto;

use crate::ser::{Serialize, SerializeError};
use thiserror::Error;

pub(crate) const BYTES_PER_CHUNK: usize = 32;

pub(crate) const ZERO_CHUNK: &[u8] = &[0; BYTES_PER_CHUNK];

pub type Root = [u8; 32];

pub trait Merkleized {
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

pub(crate) fn pack_bytes(buffer: &mut Vec<u8>) {
    let data_len = buffer.len();
    if data_len % BYTES_PER_CHUNK != 0 {
        let bytes_to_pad = BYTES_PER_CHUNK - data_len % BYTES_PER_CHUNK;
        let pad = vec![0u8; bytes_to_pad];
        buffer.extend_from_slice(&pad);
    }
}

// Packs serializations of `values` into the return buffer with the
// guarantee that `buffer.len() % BYTES_PER_CHUNK == 0`
pub(crate) fn pack<T>(values: &[T]) -> Result<Vec<u8>, MerkleizationError>
where
    T: Serialize,
{
    let mut buffer = vec![];
    for value in values {
        value.serialize(&mut buffer)?;
    }
    pack_bytes(&mut buffer);
    Ok(buffer)
}

fn hash_nodes(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(a);
    hasher.update(b);
    hasher.finalize().to_vec()
}

// Return the root of the Merklization of a binary tree
// formed from `chunks` when `chunks.len() / BYTES_PER_CHUNK > 1`.
// Invariant: `chunks.len() % BYTES_PER_CHUNK == 0`
// Invariant: `leaf_count.next_power_of_two() == leaf_count`
// NOTE: naive implementation, can make much more efficient
fn merkleize_chunks(chunks: &[u8], leaf_count: usize) -> Result<Root, MerkleizationError> {
    debug_assert!(leaf_count.next_power_of_two() == leaf_count);

    let node_count = 2 * leaf_count - 1;
    let interior_count = node_count - leaf_count;
    let leaf_start = interior_count * BYTES_PER_CHUNK;

    let mut buffer = vec![0u8; node_count * BYTES_PER_CHUNK];
    debug_assert!(node_count * BYTES_PER_CHUNK == buffer.len());
    for (i, chunk) in chunks.chunks_exact(BYTES_PER_CHUNK).enumerate() {
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

// Return the root of the Merklization of a binary tree
// formed from `chunks`.
// Invariant: `chunks.len() % BYTES_PER_CHUNK == 0`
pub fn merkleize(chunks: &[u8], limit: Option<usize>) -> Result<Root, MerkleizationError> {
    debug_assert!(chunks.len() % BYTES_PER_CHUNK == 0);
    if chunks.is_empty() {
        return Ok(Root::default());
    }
    let chunk_count = chunks.len() / BYTES_PER_CHUNK;
    let mut leaf_count = chunk_count.next_power_of_two();
    if let Some(limit) = limit {
        if limit < chunk_count {
            return Err(MerkleizationError::InputExceedsLimit(limit));
        }
        leaf_count = limit.next_power_of_two();
    }
    merkleize_chunks(chunks, leaf_count)
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
    use crate as ssz_rs;
    use crate::prelude::*;
    use hex_literal::hex;
    use ssz_rs_derive::SimpleSerialize;
    use std::iter::FromIterator;

    #[test]
    fn test_packing_basic_types_simple() {
        let b = true;
        let mut expected = vec![0u8; BYTES_PER_CHUNK];
        expected[0] = 1u8;
        let input = &[b];
        let result = pack(input).expect("can pack values");
        assert!(result.len() == BYTES_PER_CHUNK);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_packing_basic_types_extended() {
        let b = true;
        let input = &[b, !b, !b, b];
        let result = pack(input).expect("can pack values");

        let mut expected = vec![0u8; BYTES_PER_CHUNK];
        expected[0] = 1u8;
        expected[3] = 1u8;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_packing_basic_types_multiple() {
        let data = U256([1u8; 32]);
        let input = &[data.clone(), data.clone(), data.clone()];
        let result = pack(input).expect("can pack values");

        let expected = vec![1u8; 3 * 32];
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
        let chunks = vec![0u8; 2 * BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 2).expect("can merkleize");
        assert_eq!(
            root,
            hex!("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b")
        );

        let chunks = vec![1u8; 2 * BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 2).expect("can merkleize");
        assert_eq!(
            root,
            hex!("7c8975e1e60a5c8337f28edf8c33c3b180360b7279644a9bc1af3c51e6220bf5")
        );

        let chunks = vec![0u8; BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 4).expect("can merkleize");
        assert_eq!(
            root,
            hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71")
        );

        let chunks = vec![1u8; BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 4).expect("can merkleize");
        assert_eq!(
            root,
            hex!("29797eded0e83376b70f2bf034cc0811ae7f1414653b1d720dfd18f74cf13309")
        );

        let chunks = vec![2u8; BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 8).expect("can merkleize");
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
            g: List<u16, 7>,
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
            g: List::from_iter([1, 2]),
        };

        let root = foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root,
            hex!("7078155bf8f0dc42d8afccec8d9b5aeb54f0a2e8e58fcef3e723f6a867232ce7")
        );

        let original_foo = foo.clone();

        foo.b[2] = 44u32;
        foo.d.pop();
        foo.e = Bar::A(33);

        let root = original_foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root,
            hex!("7078155bf8f0dc42d8afccec8d9b5aeb54f0a2e8e58fcef3e723f6a867232ce7")
        );

        let root = foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root,
            hex!("0063bfcfabbca567483a2ee859fcfafb958329489eb328ac7f07790c7df1b231")
        );

        let encoding = serialize(&original_foo).expect("can serialize");

        let mut restored_foo = Foo::deserialize(&encoding).expect("can deserialize");

        let root = restored_foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root,
            hex!("7078155bf8f0dc42d8afccec8d9b5aeb54f0a2e8e58fcef3e723f6a867232ce7")
        );

        restored_foo.b[2] = 44u32;
        restored_foo.d.pop();
        restored_foo.e = Bar::A(33);

        let root = foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root,
            hex!("0063bfcfabbca567483a2ee859fcfafb958329489eb328ac7f07790c7df1b231")
        );
    }
}
