//! This module provides `SimpleSerialize` implementations for arrays of size 1..=32.
//! These sizes are hard-coded as `SimpleSerialize` requires a `Default` implementation
//! and Rust already defines `Default` for these special array sizes.
//! If/when this restriction is lifted in favor of const generics, the macro here
//! can likely be simplified to a definition over `const N: usize`.
use crate::{
    de::{deserialize_homogeneous_composite, Deserialize, DeserializeError},
    error::{InstanceError, TypeError},
    lib::*,
    merkleization::{elements_to_chunks, merkleize, pack, MerkleizationError, Merkleized, Node},
    ser::{Serialize, SerializeError, Serializer},
    SimpleSerialize, Sized,
};

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
                    return Err(TypeError::InvalidBound($n).into())
                }
                let mut serializer = Serializer::default();
                for element in self {
                    serializer.with_element(element)?;
                }
                serializer.serialize(buffer)
            }
        }

        impl<T> Deserialize for [T; $n]
        where
            T: SimpleSerialize,
        {
            fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
                if $n == 0 {
                    return Err(TypeError::InvalidBound($n).into())
                }

                if !T::is_variable_size() {
                    let expected_length = $n * T::size_hint();
                    if encoding.len() < expected_length {
                        return Err(DeserializeError::ExpectedFurtherInput {
                            provided: encoding.len(),
                            expected: expected_length,
                        })
                    }
                    if encoding.len() > expected_length {
                        return Err(DeserializeError::AdditionalInput {
                            provided: encoding.len(),
                            expected: expected_length,
                        })
                    }
                }
                let elements = deserialize_homogeneous_composite(encoding)?;
                elements.try_into().map_err(|elements: Vec<T>| {
                    InstanceError::Exact { required: $n, provided: elements.len() }.into()
                })
            }
        }

        impl<T> Merkleized for [T; $n]
        where
            T: SimpleSerialize,
        {
            fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
                if T::is_composite_type() {
                    let count = self.len();
                    let chunks = elements_to_chunks(self.iter_mut().enumerate(), count)?;
                    merkleize(&chunks, None)
                } else {
                    let chunks = pack(self)?;
                    merkleize(&chunks, None)
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
