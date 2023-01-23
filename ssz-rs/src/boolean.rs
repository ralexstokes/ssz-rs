use crate::{
    de::{Deserialize, DeserializeError},
    lib::*,
    merkleization::{MerkleizationError, Merkleized, Node, SszReflect},
    ser::{Serialize, SerializeError},
    SimpleSerialize, Sized, SszTypeClass,
};

impl Sized for bool {
    fn is_variable_size() -> bool {
        false
    }

    fn size_hint() -> usize {
        1
    }
}

impl Serialize for bool {
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        let value = u8::from(*self);
        buffer.push(value);
        Ok(1)
    }
}

impl Deserialize for bool {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        match encoding.len() {
            0 => Err(DeserializeError::ExpectedFurtherInput { provided: 0, expected: 1 }),
            1 => match encoding[0] {
                0u8 => Ok(false),
                1u8 => Ok(true),
                b => Err(DeserializeError::InvalidByte(b)),
            },
            n => Err(DeserializeError::AdditionalInput { provided: n, expected: 1 }),
        }
    }
}

impl Merkleized for bool {
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        if *self {
            let mut root = Node::default();
            root[0] = 1u8;
            Ok(root)
        } else {
            Ok(Node::default())
        }
    }
}

impl SimpleSerialize for bool {
    fn is_composite_type() -> bool {
        false
    }
}

impl SszReflect for bool {
    fn ssz_type_class(&self) -> SszTypeClass {
        SszTypeClass::Basic
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{deserialize, serialize};

    #[test]
    fn encode_boolean() {
        let tests = vec![(Default::default(), [0u8]), (true, [1u8])];
        for (value, expected) in tests {
            let result = serialize(&value).expect("can encode");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn decode_boolean() {
        let tests = vec![([0u8], false), ([1u8], true)];
        for (bytes, expected) in tests {
            let result = bool::deserialize(&bytes).expect("can encode");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn roundtrip_boolean() {
        for value in [true, false].iter() {
            let encoding = serialize(value).expect("can encode");
            let recovered_value: bool = deserialize(&encoding).expect("can decode");
            assert_eq!(*value, recovered_value);
        }
    }
}
