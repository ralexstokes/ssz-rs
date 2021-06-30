#[cfg(test)]
mod tests {
    // needed for derives internal to crate
    use crate as ssz;
    use crate::de::Deserialize;
    use crate::ser::Serialize;
    use crate::List;
    use crate::Vector;
    use crate::SSZ;
    use ssz_derive::Serialize;
    use std::iter::FromIterator;

    #[derive(Debug, Default, PartialEq, Eq, Serialize)]
    struct Inner {
        data: List<u8, 8>,
    }

    #[derive(Debug, PartialEq, Eq, Serialize)]
    enum Foo {
        A(u32),
        B(u8),
    }

    #[derive(Debug, PartialEq, Eq, Serialize)]
    enum Bar {
        A(u32),
        B(Vector<u8, 4>),
    }

    #[derive(Debug, PartialEq, Eq, Serialize)]
    enum Baz {
        A(u32),
        B(Inner),
        C(List<u8, 12>),
    }

    #[derive(Debug, PartialEq, Eq, Serialize)]
    enum Boo {
        A(u32),
        B(Inner),
        C(List<u8, 12>),
        D(Vector<u8, 2>),
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
        let value = Boo::A(12u32);
        let mut buffer = vec![];
        let _ = value.serialize(&mut buffer).expect("can serialize");
        let recovered = Boo::deserialize(&buffer).expect("can decode");
        assert_eq!(value, recovered);

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
