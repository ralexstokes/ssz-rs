use crate::{
    de::{Deserialize, DeserializeError},
    lib::*,
    merkleization::{pack_bytes, MerkleizationError, Merkleized, Node},
    ser::{Serialize, SerializeError},
    Serializable, SimpleSerialize, BITS_PER_BYTE,
};
use alloy_primitives::{U128, U16, U256, U32, U64, U8};

#[inline]
fn bits_to_bytes(count: u32) -> usize {
    (count / BITS_PER_BYTE) as usize
}

macro_rules! define_uint {
    ($uint:ty) => {
        impl Serializable for $uint {
            fn is_variable_size() -> bool {
                false
            }

            fn size_hint() -> usize {
                bits_to_bytes(<$uint>::BITS)
            }
        }

        impl Serialize for $uint {
            fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
                buffer.extend_from_slice(&self.to_le_bytes());
                Ok(bits_to_bytes(<$uint>::BITS))
            }
        }

        impl Deserialize for $uint {
            fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
                let byte_size = bits_to_bytes(<$uint>::BITS);
                if encoding.len() < byte_size {
                    return Err(DeserializeError::ExpectedFurtherInput {
                        provided: encoding.len(),
                        expected: byte_size,
                    })
                }
                if encoding.len() > byte_size {
                    return Err(DeserializeError::AdditionalInput {
                        provided: encoding.len(),
                        expected: byte_size,
                    })
                }

                // SAFETY: index is safe because encoding.len() has been checked above; qed
                let bytes = encoding[..byte_size].try_into().expect("slice has right length");
                Ok(<$uint>::from_le_bytes(bytes))
            }
        }

        impl Merkleized for $uint {
            fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
                let mut root = vec![];
                let _ = self.serialize(&mut root)?;
                pack_bytes(&mut root);
                Ok(root.as_slice().try_into().expect("is valid root"))
            }

            fn is_composite_type() -> bool {
                false
            }
        }

        impl SimpleSerialize for $uint {}
    };
}

macro_rules! define_alloy {
    ($uint:ty) => {
        impl Serializable for $uint {
            fn is_variable_size() -> bool {
                false
            }

            fn size_hint() -> usize {
                <$uint>::BYTES
            }
        }

        impl Serialize for $uint {
            fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
                const BYTE_SIZE: usize = <$uint>::BYTES;
                buffer.extend_from_slice(&self.to_le_bytes::<BYTE_SIZE>());
                Ok(<$uint>::BYTES)
            }
        }

        impl Deserialize for $uint {
            fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
                const BYTE_SIZE: usize = <$uint>::BYTES;
                if encoding.len() < BYTE_SIZE {
                    return Err(DeserializeError::ExpectedFurtherInput {
                        provided: encoding.len(),
                        expected: BYTE_SIZE,
                    })
                }
                if encoding.len() > BYTE_SIZE {
                    return Err(DeserializeError::AdditionalInput {
                        provided: encoding.len(),
                        expected: BYTE_SIZE,
                    })
                }

                // SAFETY: index is safe because encoding.len() has been checked above; qed
                let bytes = encoding[..BYTE_SIZE].try_into().expect("slice has right length");

                Ok(<$uint>::from_le_bytes::<BYTE_SIZE>(bytes))
            }
        }

        impl Merkleized for $uint {
            fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
                let mut root = vec![];
                let _ = self.serialize(&mut root)?;
                pack_bytes(&mut root);
                Ok(root.as_slice().try_into().expect("is valid root"))
            }

            fn is_composite_type() -> bool {
                false
            }
        }

        impl SimpleSerialize for $uint {}
    };
}

define_uint!(u8);
define_uint!(u16);
define_uint!(u32);
define_uint!(u64);
define_uint!(u128);
define_uint!(usize);

define_alloy!(U8);
define_alloy!(U16);
define_alloy!(U32);
define_alloy!(U64);
define_alloy!(U128);
define_alloy!(U256);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize;

    #[test]
    fn encode_uints() {
        let tests = vec![(u8::default(), [0u8]), (2u8, [2u8]), (u8::MAX, [u8::MAX])];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests =
            vec![(2u16, [2u8, 0u8]), (1337u16, [57u8, 5u8]), (u16::MAX, [u8::MAX, u8::MAX])];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (2u32, [2u8, 0u8, 0u8, 0u8]),
            (1337u32, [57u8, 5u8, 0u8, 0u8]),
            (u32::MAX, [u8::MAX, u8::MAX, u8::MAX, u8::MAX]),
        ];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (2u64, [2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            (1337u64, [57u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            (u64::MAX, [u8::MAX; 8]),
        ];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (
                2u128,
                [2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
            ),
            (
                1337u128,
                [57u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
            ),
            (u128::MAX, [u8::MAX; 16]),
        ];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (U256::from_le_bytes([2u8; 32]), [2u8; 32]),
            (U256::from_le_bytes([u8::MAX; 32]), [u8::MAX; 32]),
        ];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn decode_uints() {
        let tests = vec![(u8::default(), [0u8]), (2u8, [2u8]), (u8::MAX, [u8::MAX])];
        for (expected, bytes) in tests {
            let result = u8::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests =
            vec![(2u16, [2u8, 0u8]), (1337u16, [57u8, 5u8]), (u16::MAX, [u8::MAX, u8::MAX])];
        for (expected, bytes) in tests {
            let result = u16::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (2u32, [2u8, 0u8, 0u8, 0u8]),
            (1337u32, [57u8, 5u8, 0u8, 0u8]),
            (u32::MAX, [u8::MAX, u8::MAX, u8::MAX, u8::MAX]),
        ];
        for (expected, bytes) in tests {
            let result = u32::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (2u64, [2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            (1337u64, [57u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            (u64::MAX, [u8::MAX; 8]),
        ];
        for (expected, bytes) in tests {
            let result = u64::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (
                2u128,
                [2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
            ),
            (
                1337u128,
                [57u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8],
            ),
            (u128::MAX, [u8::MAX; 16]),
        ];
        for (expected, bytes) in tests {
            let result = u128::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (U256::from_le_bytes([2u8; 32]), [2u8; 32]),
            (U256::from_le_bytes([u8::MAX; 32]), [u8::MAX; 32]),
        ];
        for (expected, bytes) in tests {
            let result = U256::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
    }
}
