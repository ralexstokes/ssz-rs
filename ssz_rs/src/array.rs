//! This module provides `SimpleSerialize` implementations for arrays of size 0..=32.
//! These sizes are hard-coded as `SimpleSerialize` requires a `Default` implementation
//! and Rust already defines `Default` for these special array sizes.
//! If/when this restriction is lifted in favor of const generics, the macro here
//! can likely be simplified to a definition over `const N: usize`.
use crate::de::{Deserialize, DeserializeError};
use crate::merkleization::{Context, MerkleizationError, Merkleized, Node};
use crate::ser::{Serialize, SerializeError};
use crate::{SimpleSerialize, Sized};

macro_rules! define_ssz_for_array_of_size {
    ($n: literal) => {
        impl Sized for [u8; $n] {
            fn is_variable_size() -> bool {
                false
            }

            fn size_hint() -> usize {
                $n
            }
        }

        impl Serialize for [u8; $n] {
            fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
                buffer.extend_from_slice(self.as_ref());
                Ok(Self::size_hint())
            }
        }

        impl Deserialize for [u8; $n] {
            fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
                let byte_size = Self::size_hint();
                if encoding.len() < byte_size {
                    return Err(DeserializeError::InputTooShort);
                }
                if encoding.len() > byte_size {
                    return Err(DeserializeError::ExtraInput);
                }

                let root = encoding[..byte_size]
                    .try_into()
                    .expect("slice has right length");
                Ok(root)
            }
        }

        impl Merkleized for [u8; $n] {
            fn hash_tree_root(&self, _context: &Context) -> Result<Node, MerkleizationError> {
                let mut root = [0u8; 32];
                root.copy_from_slice(self);
                Ok(Node::from_bytes(root))
            }
        }

        impl SimpleSerialize for [u8; $n] {
            fn is_composite_type() -> bool {
                false
            }
        }
    };
}

// NOTE
define_ssz_for_array_of_size!(0);
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
