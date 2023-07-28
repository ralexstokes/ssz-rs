use crate::{
    de::{Deserialize, DeserializeError},
    lib::*,
    merkleization::{pack_bytes, MerkleizationError, Merkleized, Node},
    ser::{Serialize, SerializeError},
    Serializable, SimpleSerialize, BITS_PER_BYTE,
};
use num_bigint::BigUint;

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

define_uint!(u8);
define_uint!(u16);
define_uint!(u32);
define_uint!(u64);
define_uint!(u128);
define_uint!(usize);

/// An unsigned integer represented by 256 bits
#[derive(Default, Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct U256(BigUint);

impl U256 {
    pub fn new() -> Self {
        Self(BigUint::default())
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn try_from_bytes_le(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Self::deserialize(bytes)
    }

    pub fn from_bytes_le(bytes: [u8; 32]) -> Self {
        Self::deserialize(&bytes).unwrap()
    }

    pub fn to_bytes_le(&self) -> Vec<u8> {
        let mut bytes = self.0.to_bytes_le();
        bytes.resize(Self::size_hint(), 0u8);
        bytes
    }

    pub fn from_hex(data: &str) -> Option<Self> {
        let data = data.strip_prefix("0x").unwrap_or(data);
        BigUint::parse_bytes(data.as_bytes(), 16).map(Self)
    }
}

impl From<u64> for U256 {
    fn from(x: u64) -> Self {
        Self(x.into())
    }
}

impl Serializable for U256 {
    fn is_variable_size() -> bool {
        false
    }

    fn size_hint() -> usize {
        32
    }
}

impl Serialize for U256 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(&self.to_bytes_le());
        Ok(Self::size_hint())
    }
}

impl Deserialize for U256 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        let byte_size = Self::size_hint();
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

        // SAFETY: index is safe because encoding.len() == byte_size; qed
        let value = BigUint::from_bytes_le(&encoding[..byte_size]);
        Ok(Self(value))
    }
}

impl Merkleized for U256 {
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        let data = self.to_bytes_le();
        let node = Node::try_from(data.as_ref()).expect("is right size");
        Ok(node)
    }

    fn is_composite_type() -> bool {
        false
    }
}

impl SimpleSerialize for U256 {}

#[cfg(feature = "serde")]
impl serde::Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let output = format!("{}", self.0);
        serializer.collect_str(&output)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for U256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        let value = s.parse::<BigUint>().map_err(serde::de::Error::custom)?;
        Ok(Self(value))
    }
}

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
            (U256::try_from_bytes_le(&[2u8; 32]).unwrap(), [2u8; 32]),
            (U256::try_from_bytes_le(&[u8::MAX; 32]).unwrap(), [u8::MAX; 32]),
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
            (U256::try_from_bytes_le(&[2u8; 32]).unwrap(), [2u8; 32]),
            (U256::try_from_bytes_le(&[u8::MAX; 32]).unwrap(), [u8::MAX; 32]),
        ];
        for (expected, bytes) in tests {
            let result = U256::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
    }
}
