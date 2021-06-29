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

    #[derive(Default, Debug, PartialEq, Eq, Serialize)]
    struct Foo {
        a: u32,
    }

    #[derive(Default, Debug, PartialEq, Eq, Serialize)]
    struct Bar {
        a: List<u32, 128>,
    }

    #[derive(Default, Debug, PartialEq, Eq, Serialize)]
    struct BasicContainer {
        a: u32,
        d: bool,
    }

    #[derive(Default, Debug, PartialEq, Eq, Serialize)]
    struct SomeContainer {
        a: u32,
        b: bool,
        c: List<bool, 32>,
    }

    #[derive(Default, Debug, PartialEq, Eq, Serialize)]
    struct AnotherContainer {
        a: u32,
        b: bool,
        c: List<bool, 32>,
        d: Vector<bool, 4>,
    }

    #[test]
    fn encode_container() {
        let value = Foo { a: 5u32 };

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 4);
        let expected = [5u8, 0u8, 0u8, 0u8];
        assert_eq!(buffer, expected);

        let value = Bar {
            a: Default::default(),
        };

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 4);
        let expected = [4u8, 0u8, 0u8, 0u8];
        assert_eq!(buffer, expected);

        let value = BasicContainer { a: 5u32, d: true };

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 5);
        let expected = [5u8, 0u8, 0u8, 0u8, 1u8];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn encode_container2() {
        let value = SomeContainer {
            a: 5u32,
            b: true,
            c: List::from_iter([true, false]),
        };

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 11);
        let expected = [5u8, 0u8, 0u8, 0u8, 1u8, 9u8, 0u8, 0u8, 0u8, 1u8, 0u8];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn encode_container3() {
        let value = AnotherContainer {
            a: 5u32,
            b: true,
            c: List::from_iter([true, false]),
            d: Default::default(),
        };

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 15);
        let expected = [
            5u8, 0u8, 0u8, 0u8, 1u8, 13u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8,
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn decode_container() {
        let data = vec![5u8, 0u8, 0u8, 0u8, 1u8, 9u8, 0u8, 0u8, 0u8, 1u8, 0u8];
        let result = SomeContainer::deserialize(&data).expect("can deserialize");
        let value = SomeContainer {
            a: 5u32,
            b: true,
            c: List::from_iter([true, false]),
        };
        assert_eq!(result, value);
    }
}
