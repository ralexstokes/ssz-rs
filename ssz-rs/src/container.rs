#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::iter::FromIterator;

    #[derive(Default, Debug, PartialEq, Eq, SimpleSerialize)]
    struct Foo {
        a: u32,
    }

    #[derive(Default, Debug, PartialEq, Eq, SimpleSerialize)]
    struct Bar {
        a: List<u32, 128>,
    }

    #[derive(Default, Debug, PartialEq, Eq, SimpleSerialize)]
    struct BasicContainer {
        a: u32,
        d: bool,
    }

    #[derive(Default, Debug, PartialEq, Eq, SimpleSerialize)]
    struct SomeContainer {
        a: u32,
        b: bool,
        c: List<bool, 32>,
    }

    #[derive(Default, Debug, PartialEq, Eq, SimpleSerialize)]
    struct AnotherContainer {
        a: u32,
        b: bool,
        c: List<bool, 32>,
        d: Vector<bool, 4>,
        e: u8,
    }

    #[derive(Default, Debug, PartialEq, Eq, SimpleSerialize)]
    struct YetAnotherContainer {
        a: u32,
        b: bool,
        c: List<bool, 32>,
        d: Vector<bool, 4>,
        e: u8,
        f: List<u32, 32>,
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
            e: 12u8,
        };

        let mut buffer = vec![];
        let result = value.serialize(&mut buffer).expect("can serialize");
        assert_eq!(result, 16);
        let expected = [
            5u8, 0u8, 0u8, 0u8, 1u8, 14u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 12u8, 1u8, 0u8,
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

    #[test]
    fn roundtrip_container() {
        let value = AnotherContainer {
            a: 5u32,
            b: true,
            c: List::from_iter([true, false, false, false, true, true]),
            d: Vector::from_iter([true, false, false, true]),
            e: 24u8,
        };
        let mut buffer = vec![];
        let _ = value.serialize(&mut buffer).expect("can serialize");
        let recovered = AnotherContainer::deserialize(&buffer).expect("can decode");
        assert_eq!(value, recovered);

        let value = YetAnotherContainer {
            a: 5u32,
            b: true,
            c: List::from_iter([true, false, false, false, true, true]),
            d: Vector::from_iter([true, false, false, true]),
            e: 24u8,
            f: List::from_iter([234u32, 567u32]),
        };
        let mut buffer = vec![];
        let _ = value.serialize(&mut buffer).expect("can serialize");
        let recovered = YetAnotherContainer::deserialize(&buffer).expect("can decode");
        assert_eq!(value, recovered);
    }
}
