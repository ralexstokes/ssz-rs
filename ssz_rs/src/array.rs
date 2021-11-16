//! This module provides `SimpleSerialize` implementations for arrays of size 0..=32.
//! These sizes are hard-coded as `SimpleSerialize` requires a `Default` implementation
//! and Rust already defines `Default` for these special array sizes.
//! If/when this restriction is lifted in favor of const generics, the macro here
//! can likely be simplified to a definition over `const N: usize`.
use crate::de::{deserialize_homogeneous_composite, Deserialize, DeserializeError};
use crate::merkleization::{
    merkleize, pack, Context, MerkleizationError, Merkleized, Node, BYTES_PER_CHUNK,
};
use crate::ser::{serialize_composite, Serialize, SerializeError};
use crate::{SimpleSerialize, Sized};

macro_rules! define_ssz_for_array_of_size {
    ($n: literal) => {
        impl<T> Sized for [T; $n]
        where
            T: SimpleSerialize,
        {
            fn is_variable_size() -> bool {
                T::is_variable_size()
            }

            fn size_hint() -> usize {
                T::size_hint() * $n
            }
        }

        impl<T> Serialize for [T; $n]
        where
            T: SimpleSerialize,
        {
            fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
                if $n == 0 {
                    return Err(SerializeError::IllegalType { bound: $n });
                }
                serialize_composite(self, buffer)
            }
        }

        impl<T> Deserialize for [T; $n]
        where
            T: SimpleSerialize,
        {
            fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
                if $n == 0 {
                    return Err(DeserializeError::IllegalType { bound: $n });
                }

                if !T::is_variable_size() {
                    let expected_length = $n * T::size_hint();
                    if encoding.len() < expected_length {
                        return Err(DeserializeError::InputTooShort);
                    }
                    if encoding.len() > expected_length {
                        return Err(DeserializeError::ExtraInput);
                    }
                }
                let elements = deserialize_homogeneous_composite(encoding)?;
                elements
                    .try_into()
                    .map_err(|_| DeserializeError::InputTooShort)
            }
        }

        impl<T> Merkleized for [T; $n]
        where
            T: SimpleSerialize,
        {
            fn hash_tree_root(&self, context: &Context) -> Result<Node, MerkleizationError> {
                if T::is_composite_type() {
                    let mut chunks = vec![0u8; self.len() * BYTES_PER_CHUNK];
                    for (i, elem) in self.iter().enumerate() {
                        let chunk = elem.hash_tree_root(context)?;
                        let range = i * BYTES_PER_CHUNK..(i + 1) * BYTES_PER_CHUNK;
                        chunks[range].copy_from_slice(chunk.as_ref());
                    }
                    merkleize(&chunks, None, context)
                } else {
                    let chunks = pack(self)?;
                    merkleize(&chunks, None, context)
                }
            }
        }

        impl<T> SimpleSerialize for [T; $n]
        where
            T: SimpleSerialize,
        {
            fn is_composite_type() -> bool {
                T::is_composite_type()
            }
        }
    };
}

// NOTE: version of this that uses const generics
// requires `T: Default` which conflicts with an
// existing definition in the core lib for `[T; N]`
// for N in 1..=32. Revisit when this conflict is resolved.
define_ssz_for_array_of_size!(1);
define_ssz_for_array_of_size!(2);
define_ssz_for_array_of_size!(3);
define_ssz_for_array_of_size!(4);
define_ssz_for_array_of_size!(5);
define_ssz_for_array_of_size!(6);
define_ssz_for_array_of_size!(7);
define_ssz_for_array_of_size!(8);
define_ssz_for_array_of_size!(9);
define_ssz_for_array_of_size!(10);
define_ssz_for_array_of_size!(11);
define_ssz_for_array_of_size!(12);
define_ssz_for_array_of_size!(13);
define_ssz_for_array_of_size!(14);
define_ssz_for_array_of_size!(15);
define_ssz_for_array_of_size!(16);
define_ssz_for_array_of_size!(17);
define_ssz_for_array_of_size!(18);
define_ssz_for_array_of_size!(19);
define_ssz_for_array_of_size!(20);
define_ssz_for_array_of_size!(21);
define_ssz_for_array_of_size!(22);
define_ssz_for_array_of_size!(23);
define_ssz_for_array_of_size!(24);
define_ssz_for_array_of_size!(25);
define_ssz_for_array_of_size!(26);
define_ssz_for_array_of_size!(27);
define_ssz_for_array_of_size!(28);
define_ssz_for_array_of_size!(29);
define_ssz_for_array_of_size!(30);
define_ssz_for_array_of_size!(31);
define_ssz_for_array_of_size!(32);
