mod test_utils;

use ssz_rs::prelude::*;
use test_utils::{
    deserialize, hash_tree_root, read_ssz_snappy_from_test_data, root_from_hex, serialize,
};

#[test]
fn test_boolean_true() {
    let mut value = true;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/boolean/valid/true/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: bool = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0100000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_boolean_false() {
    let mut value = false;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/boolean/valid/false/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: bool = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
#[should_panic]
fn test_boolean_byte_0x80() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/boolean/invalid/byte_0x80/serialized.ssz_snappy",
    );

    deserialize::<bool>(&encoding);
}

#[test]
#[should_panic]
fn test_boolean_byte_2() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/boolean/invalid/byte_2/serialized.ssz_snappy",
    );

    deserialize::<bool>(&encoding);
}

#[test]
#[should_panic]
fn test_boolean_byte_full() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/boolean/invalid/byte_full/serialized.ssz_snappy",
    );

    deserialize::<bool>(&encoding);
}

#[test]
#[should_panic]
fn test_boolean_byte_rev_nibble() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/boolean/invalid/byte_rev_nibble/serialized.ssz_snappy",
    );

    deserialize::<bool>(&encoding);
}
