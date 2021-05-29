use crate::de::{Deserialize, DeserializeError};
use crate::ser::{Serialize, SerializeError};

#[repr(transparent)]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Boolean(bool);

impl Serialize for Boolean {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        buffer.push(self.0 as u8);
        Ok(1)
    }
}

impl Deserialize for Boolean {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.len() < 1 {
            return Err(DeserializeError::InputTooShort);
        }

        match encoding[0] {
            0u8 => Ok(Self(false)),
            1u8 => Ok(Self(true)),
            _ => Err(DeserializeError::InvalidInput),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{deserialize, serialize};

    #[test]
    fn encode_boolean() {
        let tests = vec![(Default::default(), [0u8]), (Boolean(true), [1u8])];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn decode_boolean() {
        let tests = vec![([0u8], Boolean(false)), ([1u8], Boolean(true))];
        for (bytes, expected) in tests {
            let result = Boolean::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn roundtrip_boolean() {
        let value = Boolean(false);
        let encoding = serialize(&value).expect("can encode");
        let recovered_value = deserialize(&encoding).expect("can decode");
        assert_eq!(value, recovered_value);
    }
}
