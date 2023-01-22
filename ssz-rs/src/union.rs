use crate::de::{Deserialize, DeserializeError};
use crate::merkleization::{mix_in_selector, MerkleizationError, Merkleized, Node};
use crate::ser::{Serialize, SerializeError};
use crate::{SimpleSerialize, Sized};

/// `SimpleSerialize` is implemented for `Option` as a convenience
/// when the schema is equivalent to one described by:
/// enum Option<T: SimpleSerialize> {
///     None,
///     Some(T),
/// }
impl<T: SimpleSerialize> Sized for Option<T> {
    fn is_variable_size() -> bool {
        true
    }

    fn size_hint() -> usize {
        0
    }
}

impl<T> Serialize for Option<T>
where
    T: SimpleSerialize,
{
    fn serialize(&self, buffer: &mut Vec<u8>) -> Result<usize, SerializeError> {
        match self {
            Some(data) => {
                let selector_bytes = 1u8.serialize(buffer)?;
                let value_bytes = data.serialize(buffer)?;
                Ok(selector_bytes + value_bytes)
            }
            None => 0u8.serialize(buffer),
        }
    }
}

impl<T> Deserialize for Option<T>
where
    T: SimpleSerialize,
{
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError> {
        if encoding.is_empty() {
            return Err(DeserializeError::ExpectedFurtherInput {
                provided: 0,
                expected: 1,
            });
        }

        match encoding[0] {
            0 => Ok(None),
            1 => {
                let inner = T::deserialize(&encoding[1..])?;
                Ok(Some(inner))
            }
            b => Err(DeserializeError::InvalidByte(b)),
        }
    }
}

impl<T> Merkleized for Option<T>
where
    T: SimpleSerialize,
{
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError> {
        match self {
            Some(value) => Ok(mix_in_selector(&value.hash_tree_root()?, 1)),
            None => Ok(mix_in_selector(&Node::default(), 0)),
        }
    }
}

impl<T> SimpleSerialize for Option<T> where T: SimpleSerialize {}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[derive(Debug, PartialEq, Eq, SimpleSerialize)]
    enum AnotherOption {
        None,
        A(u8),
        B(u8),
    }

    impl Default for AnotherOption {
        fn default() -> Self {
            Self::None
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, SimpleSerialize)]
    struct Inner {
        data: List<u8, 8>,
    }

    #[derive(Debug, PartialEq, Eq, SimpleSerialize)]
    enum Foo {
        A(u32),
        B(u8),
    }

    impl Default for Foo {
        fn default() -> Self {
            Self::A(Default::default())
        }
    }

    #[derive(Debug, PartialEq, Eq, SimpleSerialize)]
    enum Bar {
        A(u32),
        B(Vector<u8, 4>),
    }

    impl Default for Bar {
        fn default() -> Self {
            Self::A(Default::default())
        }
    }

    #[derive(Debug, PartialEq, Eq, SimpleSerialize)]
    enum Baz {
        A(u32),
        B(Inner),
        C(List<u8, 12>),
    }

    impl Default for Baz {
        fn default() -> Self {
            Self::A(Default::default())
        }
    }

    #[derive(Debug, PartialEq, Eq, SimpleSerialize)]
    enum Boo {
        A(u32),
        B(Inner),
        C(List<u8, 12>),
        D(Vector<u8, 2>),
    }

    impl Default for Boo {
        fn default() -> Self {
            Self::A(Default::default())
        }
    }

    #[test]
    fn test_option() {
        let mut x = Some(12u8);
        let mut buffer = vec![];
        let result = x.serialize(&mut buffer).expect("can encode");
        assert_eq!(result, 2);
        let expected = [1u8, 12u8];
        assert_eq!(buffer, expected);

        x = None;
        let mut buffer = vec![];
        let result = x.serialize(&mut buffer).expect("can encode");
        assert_eq!(result, 1);
        let expected = [0u8];
        assert_eq!(buffer, expected);

        x = Some(34u8);
        let mut buffer = vec![];
        let _ = x.serialize(&mut buffer).expect("can serialize");
        let recovered = Option::<u8>::deserialize(&buffer).expect("can decode");
        assert_eq!(x, recovered);
    }

    #[test]
    fn test_another_option() {
        let mut x = AnotherOption::A(12u8);
        let mut buffer = vec![];
        let result = x.serialize(&mut buffer).expect("can encode");
        assert_eq!(result, 2);
        let expected = [1u8, 12u8];
        assert_eq!(buffer, expected);

        x = AnotherOption::None;
        let mut buffer = vec![];
        let result = x.serialize(&mut buffer).expect("can encode");
        assert_eq!(result, 1);
        let expected = [0u8];
        assert_eq!(buffer, expected);

        x = AnotherOption::B(32u8);
        let mut buffer = vec![];
        let _ = x.serialize(&mut buffer).expect("can serialize");
        let recovered = AnotherOption::deserialize(&buffer).expect("can decode");
        assert_eq!(x, recovered);
    }

    #[test]
    fn encode_union() {
        let value = Foo::A(12u32);

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 5);
        let expected = [0u8, 12u8, 0u8, 0u8, 0u8];
        assert_eq!(buffer, expected);

        let value = Foo::B(6u8);

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 2);
        let expected = [1u8, 6u8];
        assert_eq!(buffer, expected);

        let value = Bar::A(12u32);

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 5);
        let expected = [0u8, 12u8, 0u8, 0u8, 0u8];
        assert_eq!(buffer, expected);

        let value = Bar::B(Vector::from_iter([3u8, 2u8, 1u8, 10u8]));

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 5);
        let expected = [1u8, 3u8, 2u8, 1u8, 10u8];
        assert_eq!(buffer, expected);

        let value = Baz::A(12u32);

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 5);
        let expected = [0u8, 12u8, 0u8, 0u8, 0u8];
        assert_eq!(buffer, expected);

        let value = Baz::B(Inner {
            data: List::from_iter([123u8]),
        });

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 6);
        let expected = [1u8, 4u8, 0u8, 0u8, 0u8, 123u8];
        assert_eq!(buffer, expected);

        let value = Baz::C(List::from_iter([123u8, 253u8]));

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 3);
        let expected = [2u8, 123u8, 253u8];
        assert_eq!(buffer, expected);

        let value = Boo::A(12u32);

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 5);
        let expected = [0u8, 12u8, 0u8, 0u8, 0u8];
        assert_eq!(buffer, expected);

        let value = Boo::B(Inner {
            data: List::from_iter([123u8]),
        });

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 6);
        let expected = [1u8, 4u8, 0u8, 0u8, 0u8, 123u8];
        assert_eq!(buffer, expected);

        let value = Boo::C(List::from_iter([123u8, 253u8]));

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 3);
        let expected = [2u8, 123u8, 253u8];
        assert_eq!(buffer, expected);

        let value = Boo::D(Vector::from_iter([123u8, 253u8]));

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 3);
        let expected = [3u8, 123u8, 253u8];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn decode_union() {
        let data = [0u8, 12u8, 0u8, 0u8, 0u8];
        let result = Boo::deserialize(&data).expect("can decode");
        let value = Boo::A(12u32);
        assert_eq!(result, value);

        let data = [1u8, 4u8, 0u8, 0u8, 0u8, 123u8];
        let result = Boo::deserialize(&data).expect("can decode");
        let value = Boo::B(Inner {
            data: List::from_iter([123u8]),
        });
        assert_eq!(result, value);

        let data = [2u8, 123u8, 253u8];
        let result = Boo::deserialize(&data).expect("can decode");
        let value = Boo::C(List::from_iter([123u8, 253u8]));
        assert_eq!(result, value);

        let data = [3u8, 123u8, 253u8];
        let result = Boo::deserialize(&data).expect("can decode");
        let value = Boo::D(Vector::from_iter([123u8, 253u8]));
        assert_eq!(result, value);
    }

    #[test]
    fn roundtrip_union() {
        let value = Boo::default();
        let mut buffer = vec![];
        let _ = value.serialize(&mut buffer).expect("can serialize");
        let recovered = Boo::deserialize(&buffer).expect("can decode");
        assert_eq!(value, recovered);
        assert_eq!(value, Boo::A(u32::default()));

        let value = Boo::B(Inner {
            data: List::from_iter([123u8]),
        });
        let mut buffer = vec![];
        let _ = value.serialize(&mut buffer).expect("can serialize");
        let recovered = Boo::deserialize(&buffer).expect("can decode");
        assert_eq!(value, recovered);

        let value = Boo::C(List::from_iter([123u8, 253u8]));
        let mut buffer = vec![];
        let _ = value.serialize(&mut buffer).expect("can serialize");
        let recovered = Boo::deserialize(&buffer).expect("can decode");
        assert_eq!(value, recovered);

        let value = Boo::D(Vector::from_iter([123u8, 253u8]));
        let mut buffer = vec![];
        let _ = value.serialize(&mut buffer).expect("can serialize");
        let recovered = Boo::deserialize(&buffer).expect("can decode");
        assert_eq!(value, recovered);
    }
}
