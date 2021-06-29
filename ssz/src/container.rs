#[cfg(test)]
mod tests {
    // needed for derives internal to crate
    use crate as ssz;
    use crate::List;
    use crate::Serialize;
    use crate::SSZ;
    use std::iter::FromIterator;

    #[test]
    fn encode_container() {
        #[derive(Default, Serialize)]
        struct SomeContainer {
            a: u32,
            b: bool,
            c: List<bool, 32>,
        }

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
}
