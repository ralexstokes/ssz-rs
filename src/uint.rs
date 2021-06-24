use crate::de::{Deserialize, DeserializeError};
use crate::ser::{Serialize, SerializeError};
use crate::ssz::SSZ;
use std::convert::TryInto;
use std::default::Default;

impl SSZ for u8 {
    fn is_variable_size(&self) -> bool {
        false
    }

    fn size_hint() -> usize {
        1
    }
}

impl Serialize for u8 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.push(*self);
        Ok(1)
    }
}

impl Deserialize for u8 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 1 {
            return Err(DeserializeError::InputTooShort);
        }

        Ok(encoding[0])
    }
}

impl SSZ for u16 {
    fn is_variable_size(&self) -> bool {
        false
    }

    fn size_hint() -> usize {
        2
    }
}

impl Serialize for u16 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(&self.to_le_bytes());
        Ok(2)
    }
}

impl Deserialize for u16 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 2 {
            return Err(DeserializeError::InputTooShort);
        }

        let bytes = encoding[..2].try_into().expect("slice has right length");
        Ok(u16::from_le_bytes(bytes))
    }
}

impl SSZ for u32 {
    fn is_variable_size(&self) -> bool {
        false
    }

    fn size_hint() -> usize {
        4
    }
}

impl Serialize for u32 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(&self.to_le_bytes());
        Ok(4)
    }
}

impl Deserialize for u32 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 4 {
            return Err(DeserializeError::InputTooShort);
        }

        let bytes = encoding[..4].try_into().expect("slice has right length");
        Ok(u32::from_le_bytes(bytes))
    }
}

impl SSZ for u64 {
    fn is_variable_size(&self) -> bool {
        false
    }

    fn size_hint() -> usize {
        8
    }
}

impl Serialize for u64 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(&self.to_le_bytes());
        Ok(8)
    }
}

impl Deserialize for u64 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 8 {
            return Err(DeserializeError::InputTooShort);
        }

        let bytes = encoding[..8].try_into().expect("slice has right length");
        Ok(u64::from_le_bytes(bytes))
    }
}

impl SSZ for u128 {
    fn is_variable_size(&self) -> bool {
        false
    }

    fn size_hint() -> usize {
        16
    }
}

impl Serialize for u128 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(&self.to_le_bytes());
        Ok(16)
    }
}

impl Deserialize for u128 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 16 {
            return Err(DeserializeError::InputTooShort);
        }

        let bytes = encoding[..16].try_into().expect("slice has right length");
        Ok(u128::from_le_bytes(bytes))
    }
}

#[repr(transparent)]
#[derive(Default, Debug, PartialEq, Eq)]
// inner slice is little-endian
pub struct Uint256([u8; 32]);

impl SSZ for Uint256 {
    fn is_variable_size(&self) -> bool {
        false
    }

    fn size_hint() -> usize {
        32
    }
}

impl Serialize for Uint256 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(&self.0);
        Ok(32)
    }
}

impl Deserialize for Uint256 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 32 {
            return Err(DeserializeError::InputTooShort);
        }

        let bytes = encoding[..32].try_into().expect("slice has right length");
        Ok(Self(bytes))
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
        let tests = vec![
            (2u16, [2u8, 0u8]),
            (1337u16, [57u8, 5u8]),
            (u16::MAX, [u8::MAX, u8::MAX]),
        ];
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
                [
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
            ),
            (
                1337u128,
                [
                    57u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
            ),
            (u128::MAX, [u8::MAX; 16]),
        ];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (Uint256([2u8; 32]), [2u8; 32]),
            (Uint256([u8::MAX; 32]), [u8::MAX; 32]),
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
        let tests = vec![
            (2u16, [2u8, 0u8]),
            (1337u16, [57u8, 5u8]),
            (u16::MAX, [u8::MAX, u8::MAX]),
        ];
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
                [
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
            ),
            (
                1337u128,
                [
                    57u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
            ),
            (u128::MAX, [u8::MAX; 16]),
        ];
        for (expected, bytes) in tests {
            let result = u128::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (Uint256([2u8; 32]), [2u8; 32]),
            (Uint256([u8::MAX; 32]), [u8::MAX; 32]),
        ];
        for (expected, bytes) in tests {
            let result = Uint256::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
    }
}
