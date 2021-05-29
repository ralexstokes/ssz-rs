use crate::de::{Deserialize, DeserializeError};
use crate::ser::{Serialize, SerializeError};
use std::convert::TryInto;
use std::default::Default;

#[repr(transparent)]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Uint8(u8);

impl Serialize for Uint8 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.push(self.0);
        Ok(1)
    }
}

impl Deserialize for Uint8 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 1 {
            return Err(DeserializeError::InputTooShort);
        }

        Ok(Self(encoding[0]))
    }
}

#[repr(transparent)]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Uint16(u16);

impl Serialize for Uint16 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(&self.0.to_le_bytes());
        Ok(2)
    }
}

impl Deserialize for Uint16 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 2 {
            return Err(DeserializeError::InputTooShort);
        }

        let bytes = encoding[..2].try_into().expect("slice has right length");
        Ok(Self(u16::from_le_bytes(bytes)))
    }
}

#[repr(transparent)]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Uint32(u32);

impl Serialize for Uint32 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(&self.0.to_le_bytes());
        Ok(4)
    }
}

impl Deserialize for Uint32 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 4 {
            return Err(DeserializeError::InputTooShort);
        }

        let bytes = encoding[..4].try_into().expect("slice has right length");
        Ok(Self(u32::from_le_bytes(bytes)))
    }
}

#[repr(transparent)]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Uint64(u64);

impl Serialize for Uint64 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(&self.0.to_le_bytes());
        Ok(8)
    }
}

impl Deserialize for Uint64 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 8 {
            return Err(DeserializeError::InputTooShort);
        }

        let bytes = encoding[..8].try_into().expect("slice has right length");
        Ok(Self(u64::from_le_bytes(bytes)))
    }
}

#[repr(transparent)]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Uint128(u128);

impl Serialize for Uint128 {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.extend_from_slice(&self.0.to_le_bytes());
        Ok(16)
    }
}

impl Deserialize for Uint128 {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 16 {
            return Err(DeserializeError::InputTooShort);
        }

        let bytes = encoding[..16].try_into().expect("slice has right length");
        Ok(Self(u128::from_le_bytes(bytes)))
    }
}

#[repr(transparent)]
#[derive(Default, Debug, PartialEq, Eq)]
// inner slice is little-endian
pub struct Uint256([u8; 32]);

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
    use crate::ser::serialize;

    #[test]
    fn encode_uints() {
        let tests = vec![
            (Uint8::default(), [0u8]),
            (Uint8(2u8), [2u8]),
            (Uint8(u8::MAX), [u8::MAX]),
        ];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (Uint16(2u16), [2u8, 0u8]),
            (Uint16(1337u16), [57u8, 5u8]),
            (Uint16(u16::MAX), [u8::MAX, u8::MAX]),
        ];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (Uint32(2u32), [2u8, 0u8, 0u8, 0u8]),
            (Uint32(1337u32), [57u8, 5u8, 0u8, 0u8]),
            (Uint32(u32::MAX), [u8::MAX, u8::MAX, u8::MAX, u8::MAX]),
        ];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (Uint64(2u64), [2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            (Uint64(1337u64), [57u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            (Uint64(u64::MAX), [u8::MAX; 8]),
        ];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (
                Uint128(2u128),
                [
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
            ),
            (
                Uint128(1337u128),
                [
                    57u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
            ),
            (Uint128(u128::MAX), [u8::MAX; 16]),
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
        let tests = vec![
            (Uint8::default(), [0u8]),
            (Uint8(2u8), [2u8]),
            (Uint8(u8::MAX), [u8::MAX]),
        ];
        for (expected, bytes) in tests {
            let result = Uint8::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (Uint16(2u16), [2u8, 0u8]),
            (Uint16(1337u16), [57u8, 5u8]),
            (Uint16(u16::MAX), [u8::MAX, u8::MAX]),
        ];
        for (expected, bytes) in tests {
            let result = Uint16::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (Uint32(2u32), [2u8, 0u8, 0u8, 0u8]),
            (Uint32(1337u32), [57u8, 5u8, 0u8, 0u8]),
            (Uint32(u32::MAX), [u8::MAX, u8::MAX, u8::MAX, u8::MAX]),
        ];
        for (expected, bytes) in tests {
            let result = Uint32::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (Uint64(2u64), [2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            (Uint64(1337u64), [57u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            (Uint64(u64::MAX), [u8::MAX; 8]),
        ];
        for (expected, bytes) in tests {
            let result = Uint64::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
        let tests = vec![
            (
                Uint128(2u128),
                [
                    2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
            ),
            (
                Uint128(1337u128),
                [
                    57u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                ],
            ),
            (Uint128(u128::MAX), [u8::MAX; 16]),
        ];
        for (expected, bytes) in tests {
            let result = Uint128::deserialize(&bytes).expect("can encode");
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
