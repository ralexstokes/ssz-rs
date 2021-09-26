use crate::ser::{Serialize, SerializeError};
use sha2::{Digest, Sha256};
use std::{cmp::Ordering, convert::TryInto, ops::Index};
use thiserror::Error;

pub(crate) const BYTES_PER_CHUNK: usize = 32;

pub(crate) const ZERO_CHUNK: &[u8] = &[0; BYTES_PER_CHUNK];

pub type Root = [u8; 32];

pub trait Merkleized {
    fn hash_tree_root(&self, context: &Context) -> Result<Root, MerkleizationError>;
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

fn hash_nodes(hasher: &mut Sha256, a: &[u8], b: &[u8], out: &mut [u8]) {
    hasher.update(a);
    hasher.update(b);
    out.copy_from_slice(&hasher.finalize_reset());
}

const MAX_MERKLE_TREE_DEPTH: usize = 64;

fn compute_zero_hashes() -> [u8; MAX_MERKLE_TREE_DEPTH * BYTES_PER_CHUNK] {
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; MAX_MERKLE_TREE_DEPTH * BYTES_PER_CHUNK];
    for i in 0..MAX_MERKLE_TREE_DEPTH - 1 {
        let focus_range = i * BYTES_PER_CHUNK..(i + 2) * BYTES_PER_CHUNK;
        let focus = &mut buffer[focus_range];
        let (source, target) = focus.split_at_mut(BYTES_PER_CHUNK);
        hash_nodes(&mut hasher, source, source, target);
    }
    buffer
}

pub struct Context {
    zero_hashes: [u8; MAX_MERKLE_TREE_DEPTH * BYTES_PER_CHUNK],
}

impl Context {
    pub fn new() -> Self {
        Self {
            zero_hashes: compute_zero_hashes(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for Context {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.zero_hashes[index * BYTES_PER_CHUNK..(index + 1) * BYTES_PER_CHUNK]
    }
}

// Return the root of the Merklization of a binary tree formed from `chunks`.
// `chunks` forms the bottom layer of a binary tree that is Merkleized.
// This implementation is memory efficient by relying on pre-computed subtrees of all
// "zero" leaves stored in the `context`. SSZ specifies that `chunks` is padded to the next power
// of two and this can be quite large for some types. "Zero" subtrees are virtualized to avoid the
// memory and computation cost of large trees with partially empty leaves.
// Invariant: `chunks.len() % BYTES_PER_CHUNK == 0`
// Invariant: `leaf_count.next_power_of_two() == leaf_count`
fn merkleize_chunks_with_virtual_padding(
    chunks: &[u8],
    leaf_count: usize,
    context: &Context,
) -> Result<Root, MerkleizationError> {
    let chunk_count = chunks.len() / BYTES_PER_CHUNK;

    let mut hasher = Sha256::new();
    debug_assert!(chunks.len() % BYTES_PER_CHUNK == 0);
    debug_assert!(leaf_count.next_power_of_two() == leaf_count);

    let height = leaf_count.trailing_zeros() + 1;
    let mut layer = chunks.to_vec();
    let mut last_index = chunk_count - 1;
    for k in (1..height).rev() {
        for i in (0..2usize.pow(k)).step_by(2) {
            let parent_index = i / 2;
            match i.cmp(&last_index) {
                Ordering::Less => {
                    let focus =
                        &mut layer[parent_index * BYTES_PER_CHUNK..(i + 2) * BYTES_PER_CHUNK];
                    let children_index = focus.len() - 2 * BYTES_PER_CHUNK;
                    let (parent, children) = focus.split_at_mut(children_index);
                    let (left, right) = children.split_at_mut(BYTES_PER_CHUNK);
                    if parent.is_empty() {
                        // NOTE: have to specially handle the situation where the children nodes and parent node share memory
                        hasher.update(&left);
                        hasher.update(right);
                        left.copy_from_slice(&hasher.finalize_reset());
                    } else {
                        hash_nodes(&mut hasher, left, right, &mut parent[..BYTES_PER_CHUNK]);
                    }
                }
                Ordering::Equal => {
                    let focus =
                        &mut layer[parent_index * BYTES_PER_CHUNK..(i + 1) * BYTES_PER_CHUNK];
                    let children_index = focus.len() - BYTES_PER_CHUNK;
                    let (parent, children) = focus.split_at_mut(children_index);
                    let (left, _) = children.split_at_mut(BYTES_PER_CHUNK);
                    let depth = height - k - 1;
                    let right = &context[depth as usize];
                    if parent.is_empty() {
                        // NOTE: have to specially handle the situation where the children nodes and parent node share memory
                        hasher.update(&left);
                        hasher.update(right);
                        left.copy_from_slice(&hasher.finalize_reset());
                    } else {
                        hash_nodes(&mut hasher, left, right, &mut parent[..BYTES_PER_CHUNK]);
                    }
                }
                _ => break,
            };
        }
        last_index /= 2;
    }

    Ok(layer[..BYTES_PER_CHUNK]
        .try_into()
        .expect("can produce a single root chunk"))
}

// Return the root of the Merklization of a binary tree
// formed from `chunks`.
// Invariant: `chunks.len() % BYTES_PER_CHUNK == 0`
pub fn merkleize(
    chunks: &[u8],
    limit: Option<usize>,
    context: &Context,
) -> Result<Root, MerkleizationError> {
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
    merkleize_chunks_with_virtual_padding(chunks, leaf_count, context)
}

fn mix_in_decoration(root: &Root, decoration: usize, context: &Context) -> Root {
    let decoration_data = decoration
        .hash_tree_root(context)
        .expect("can merkleize usize");

    let mut hasher = Sha256::new();
    let mut output = vec![0u8; BYTES_PER_CHUNK];
    hash_nodes(&mut hasher, root, &decoration_data, &mut output);
    output.try_into().expect("can extract root")
}

pub(crate) fn mix_in_length(root: &Root, length: usize, context: &Context) -> Root {
    mix_in_decoration(root, length, context)
}

pub fn mix_in_selector(root: &Root, selector: usize, context: &Context) -> Root {
    mix_in_decoration(root, selector, context)
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
    fn test_merkleize_basic() {
        let context = Context::new();

        let input = &[];
        let result = merkleize(input, None, &context).expect("can merkle");
        assert_eq!(result, Root::default());

        let b = true;
        let input = &[b];
        let input = pack(input).expect("can pack");
        let result = merkleize(&input, None, &context).expect("can merkle");
        let mut expected = Root::default();
        expected[0] = 1u8;
        assert_eq!(result, expected);
    }

    // Return the root of the Merklization of a binary tree formed from `chunks`.
    // Invariant: `chunks.len() % BYTES_PER_CHUNK == 0`
    // Invariant: `leaf_count.next_power_of_two() == leaf_count`
    // NOTE: naive implementation, can make much more efficient
    fn merkleize_chunks(chunks: &[u8], leaf_count: usize) -> Result<Root, MerkleizationError> {
        debug_assert!(chunks.len() % BYTES_PER_CHUNK == 0);
        debug_assert!(leaf_count.next_power_of_two() == leaf_count);

        let node_count = 2 * leaf_count - 1;
        let interior_count = node_count - leaf_count;
        let leaf_start = interior_count * BYTES_PER_CHUNK;

        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; node_count * BYTES_PER_CHUNK];
        buffer[leaf_start..leaf_start + chunks.len()].copy_from_slice(chunks);
        for i in chunks.len()..leaf_count {
            let start = leaf_start + (i * BYTES_PER_CHUNK);
            let end = leaf_start + (i + 1) * BYTES_PER_CHUNK;
            buffer[start..end].copy_from_slice(ZERO_CHUNK);
        }

        for i in (1..node_count).rev().step_by(2) {
            let parent_index = (i - 1) / 2;
            let focus = &mut buffer[parent_index * BYTES_PER_CHUNK..(i + 1) * BYTES_PER_CHUNK];
            let children_index = focus.len() - 2 * BYTES_PER_CHUNK;
            let (parent, children) = focus.split_at_mut(children_index);
            let left = &children[0..BYTES_PER_CHUNK];
            let right = &children[BYTES_PER_CHUNK..2 * BYTES_PER_CHUNK];
            hash_nodes(&mut hasher, left, right, &mut parent[..BYTES_PER_CHUNK]);
        }
        Ok(buffer[0..BYTES_PER_CHUNK]
            .try_into()
            .expect("can produce a single root chunk"))
    }

    #[test]
    fn test_naive_merkleize_chunks() {
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

        let chunks = vec![1u8; 5 * BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 8).expect("can merkleize");
        assert_eq!(
            root,
            hex!("0ae67e34cba4ad2bbfea5dc39e6679b444021522d861fab00f05063c54341289")
        );
    }

    #[test]
    fn test_merkleize_chunks() {
        let context = Context::new();

        let chunks = vec![1u8; 3 * BYTES_PER_CHUNK];
        let root =
            merkleize_chunks_with_virtual_padding(&chunks, 4, &context).expect("can merkleize");
        assert_eq!(
            root,
            hex!("65aa94f2b59e517abd400cab655f42821374e433e41b8fe599f6bb15484adcec")
        );

        let chunks = vec![1u8; 5 * BYTES_PER_CHUNK];
        let root =
            merkleize_chunks_with_virtual_padding(&chunks, 8, &context).expect("can merkleize");
        assert_eq!(
            root,
            hex!("0ae67e34cba4ad2bbfea5dc39e6679b444021522d861fab00f05063c54341289")
        );

        let chunks = vec![1u8; 6 * BYTES_PER_CHUNK];
        let root =
            merkleize_chunks_with_virtual_padding(&chunks, 8, &context).expect("can merkleize");
        assert_eq!(
            root,
            hex!("0ef7df63c204ef203d76145627b8083c49aa7c55ebdee2967556f55a4f65a238")
        );
    }

    #[test]
    fn test_merkleize_chunks_with_many_virtual_nodes() {
        let context = Context::new();

        let chunks = vec![1u8; 5 * BYTES_PER_CHUNK];
        let root = merkleize_chunks_with_virtual_padding(&chunks, 2usize.pow(10), &context)
            .expect("can merkleize");
        assert_eq!(
            root,
            hex!("2647cb9e26bd83eeb0982814b2ac4d6cc4a65d0d98637f1a73a4c06d3db0e6ce")
        );

        let chunks = vec![1u8; 70 * BYTES_PER_CHUNK];
        let root = merkleize_chunks_with_virtual_padding(&chunks, 2usize.pow(63), &context)
            .expect("can merkleize");
        assert_eq!(
            root,
            hex!("9317695d95b5a3b46e976b5a9cbfcfccb600accaddeda9ac867cc9669b862979")
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

        let context = Context::new();

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

        let root = foo.hash_tree_root(&context).expect("can make root");
        assert_eq!(
            root,
            hex!("7078155bf8f0dc42d8afccec8d9b5aeb54f0a2e8e58fcef3e723f6a867232ce7")
        );

        let original_foo = foo.clone();

        foo.b[2] = 44u32;
        foo.d.pop();
        foo.e = Bar::A(33);

        let root = original_foo
            .hash_tree_root(&context)
            .expect("can make root");
        assert_eq!(
            root,
            hex!("7078155bf8f0dc42d8afccec8d9b5aeb54f0a2e8e58fcef3e723f6a867232ce7")
        );

        let root = foo.hash_tree_root(&context).expect("can make root");
        assert_eq!(
            root,
            hex!("0063bfcfabbca567483a2ee859fcfafb958329489eb328ac7f07790c7df1b231")
        );

        let encoding = serialize(&original_foo).expect("can serialize");

        let mut restored_foo = Foo::deserialize(&encoding).expect("can deserialize");

        let root = restored_foo
            .hash_tree_root(&context)
            .expect("can make root");
        assert_eq!(
            root,
            hex!("7078155bf8f0dc42d8afccec8d9b5aeb54f0a2e8e58fcef3e723f6a867232ce7")
        );

        restored_foo.b[2] = 44u32;
        restored_foo.d.pop();
        restored_foo.e = Bar::A(33);

        let root = foo.hash_tree_root(&context).expect("can make root");
        assert_eq!(
            root,
            hex!("0063bfcfabbca567483a2ee859fcfafb958329489eb328ac7f07790c7df1b231")
        );
    }
}
