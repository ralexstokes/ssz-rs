mod test_utils;

use ssz_rs::prelude::*;
use std::iter::FromIterator;
use test_utils::{
    deserialize, hash_tree_root, read_ssz_snappy_from_test_data, root_from_hex, serialize,
};

#[test]
fn test_bitvector_bitvec_5_max() {
    let value = Bitvector::<5>::from_iter([true, true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_5_max/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("1f00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_1_random() {
    let value = Bitvector::<1>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_1_random/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_512_random() {
    let value = Bitvector::<512>::from_iter([
        false, true, false, true, false, false, false, false, true, false, false, true, true,
        false, false, false, true, true, false, true, false, false, false, true, true, false, true,
        true, false, false, true, false, false, false, false, true, true, true, false, true, true,
        false, true, true, false, true, false, true, false, true, true, false, true, false, true,
        false, true, true, false, false, false, true, false, true, true, true, true, false, true,
        true, true, true, true, false, false, false, true, true, true, true, false, true, true,
        false, false, false, false, false, true, false, false, true, false, false, false, false,
        true, false, true, false, false, true, false, false, false, false, false, false, true,
        true, false, false, false, false, false, true, false, false, true, false, true, true,
        false, true, false, false, true, true, false, false, false, false, true, true, false,
        false, false, false, true, true, false, true, false, false, false, false, true, true, true,
        false, false, false, false, true, true, true, true, false, true, true, false, true, false,
        false, false, true, false, false, true, false, false, true, true, false, true, true, false,
        true, false, false, true, true, true, true, false, true, true, false, false, true, false,
        true, false, true, true, false, false, false, true, true, true, true, false, true, false,
        false, false, true, true, true, true, true, true, false, false, false, false, false, false,
        false, true, false, true, true, false, false, true, false, true, false, false, false,
        false, false, false, true, false, true, true, true, true, true, false, true, true, false,
        true, false, false, false, false, false, false, true, false, false, false, false, true,
        true, false, true, false, false, false, false, false, true, true, false, true, true, true,
        false, false, false, false, true, false, false, false, true, false, true, true, true, true,
        false, true, true, false, true, true, false, true, false, true, false, false, false, false,
        false, false, true, true, true, false, false, false, true, false, false, false, false,
        false, false, false, false, false, true, false, true, true, false, true, false, false,
        false, true, false, false, true, false, true, true, true, false, false, false, true, true,
        true, true, false, true, true, false, true, false, false, false, true, true, false, true,
        true, false, false, true, true, false, false, false, false, false, true, false, false,
        false, true, true, false, true, false, true, true, true, true, true, true, false, true,
        false, true, true, true, true, true, false, true, false, false, false, false, false, false,
        true, true, false, false, false, true, true, false, false, false, true, false, false, true,
        true, false, false, true, false, true, true, false, true, true, true, false, true, false,
        false, true, false, true, true, false, true, true, false, true, false, false, true, true,
        false, true, false, false, true, false, true, false, true, true, false, false, true, false,
        false, true, false, true, true, false, true, false, false, true, true, true, true, false,
        false, true, true, false, false, false, true, true, false, true, false, true, false, false,
        false, false, false, true, true, false, true, false, true, true, true, false, true, false,
        true, true, false, true, false, false, false, true, true, false, true, true, false, true,
        true, true, true, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_512_random/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("fbdb71e991457c4fd956e16be1ae1dc959bceaf00f692fec9431de3f0175655a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_4_max() {
    let value = Bitvector::<4>::from_iter([true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_4_max/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0f00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_5_zero() {
    let value = Bitvector::<5>::from_iter([false, false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_5_zero/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_3_max() {
    let value = Bitvector::<3>::from_iter([true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_3_max/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0700000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_4_zero() {
    let value = Bitvector::<4>::from_iter([false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_4_zero/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_31_zero() {
    let value = Bitvector::<31>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_31_zero/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_2_max() {
    let value = Bitvector::<2>::from_iter([true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_2_max/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0300000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_3_random() {
    let value = Bitvector::<3>::from_iter([true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_3_random/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0700000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_8_zero() {
    let value = Bitvector::<8>::from_iter([false, false, false, false, false, false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_8_zero/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_31_max() {
    let value = Bitvector::<31>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_31_max/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ffffff7f00000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_16_random() {
    let value = Bitvector::<16>::from_iter([
        false, false, true, false, true, true, true, false, true, true, true, false, true, true,
        false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_16_random/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("2eec000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_1_max() {
    let value = Bitvector::<1>::from_iter([true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_1_max/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0100000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_3_zero() {
    let value = Bitvector::<3>::from_iter([false, false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_3_zero/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<3> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_16_zero() {
    let value = Bitvector::<16>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_16_zero/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_512_max() {
    let value = Bitvector::<512>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_512_max/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("8667e718294e9e0df1d30600ba3eeb201f764aad2dad72748643e4a285e1d1f7");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_5_random() {
    let value = Bitvector::<5>::from_iter([false, false, false, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_5_random/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<5> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0300000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_513_max() {
    let value = Bitvector::<513>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_513_max/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("222dd9eebc6467de9788eb1c05ce9c2da8ecc89abdd38810925ce061d91236ef");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_2_zero() {
    let value = Bitvector::<2>::from_iter([false, false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_2_zero/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_1_zero() {
    let value = Bitvector::<1>::from_iter([false]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_1_zero/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<1> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_512_zero() {
    let value = Bitvector::<512>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_512_zero/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<512> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_513_random() {
    let value = Bitvector::<513>::from_iter([
        false, false, true, false, false, false, true, true, false, false, false, false, false,
        true, true, true, true, true, false, false, true, true, true, true, false, true, false,
        false, true, true, false, true, false, true, true, true, false, false, true, false, false,
        false, false, false, true, false, false, true, true, false, false, true, false, true,
        false, false, false, false, false, true, false, true, true, false, false, true, false,
        true, true, true, false, true, true, false, false, false, false, false, false, false,
        false, false, false, true, false, true, false, false, false, true, false, false, false,
        false, false, false, true, true, false, false, true, true, true, false, false, false, true,
        false, false, false, false, false, false, false, false, true, false, true, true, false,
        true, true, false, false, true, true, false, false, true, true, true, false, true, false,
        true, false, true, true, false, false, true, false, false, false, true, true, true, true,
        true, true, false, false, true, true, true, true, false, false, true, true, false, false,
        false, false, false, false, false, true, false, false, false, true, true, false, true,
        true, true, true, false, true, true, false, false, false, false, true, false, true, false,
        false, false, false, true, false, false, false, true, false, true, false, false, true,
        true, true, true, false, true, false, true, true, true, true, true, true, false, false,
        false, true, true, false, false, true, false, false, false, false, false, true, false,
        false, false, false, false, true, false, true, false, true, true, true, true, false, false,
        true, true, false, false, true, true, false, true, true, true, true, false, true, false,
        true, true, true, true, false, false, true, false, true, true, false, true, true, true,
        true, false, false, true, true, false, false, true, false, false, false, false, false,
        false, false, true, false, true, true, false, false, false, false, false, true, false,
        false, false, false, false, false, true, true, true, true, true, false, false, false, true,
        true, false, false, false, false, true, true, true, true, false, false, false, false, true,
        true, false, true, false, true, true, true, false, true, false, true, true, true, false,
        false, true, false, false, true, false, false, true, false, true, false, false, true,
        false, true, true, false, true, false, false, false, false, true, true, false, false,
        false, false, true, false, false, true, true, true, true, false, true, true, true, true,
        true, true, true, false, true, false, false, true, true, true, false, false, true, true,
        false, false, true, true, true, false, false, true, false, false, false, false, true, true,
        false, true, false, false, false, false, true, true, true, true, true, false, false, false,
        false, true, false, false, false, false, true, true, true, false, false, false, true, true,
        false, false, true, false, false, false, false, true, true, true, true, true, false, true,
        false, true, true, true, true, false, true, false, false, true, false, false, true, false,
        false, true, true, false, true, false, true, false, true, true, true, true, false, false,
        true, true, true, false, false, false, false, true, true, true, true, true, true, true,
        false, false, false, false, true, false, false, false, true, true, true, true, true, true,
        true, false, false, false, true, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_513_random/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("84f06e5024cc71b8162c3a96f4b743505481722da5a281a6aaa69791b9f79283");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_31_random() {
    let value = Bitvector::<31>::from_iter([
        false, true, true, true, false, false, true, false, true, true, false, true, true, true,
        true, true, false, true, true, false, false, true, false, false, false, false, true, false,
        true, false, true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_31_random/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<31> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("72df641500000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_2_random() {
    let value = Bitvector::<2>::from_iter([true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_2_random/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<2> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0300000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_513_zero() {
    let value = Bitvector::<513>::from_iter([
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_513_zero/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<513> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_16_max() {
    let value = Bitvector::<16>::from_iter([
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_16_max/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<16> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ffff000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_4_random() {
    let value = Bitvector::<4>::from_iter([true, true, false, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_4_random/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<4> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0d00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_8_random() {
    let value = Bitvector::<8>::from_iter([true, true, false, true, true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_8_random/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("df00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_bitvector_bitvec_8_max() {
    let value = Bitvector::<8>::from_iter([true, true, true, true, true, true, true, true]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/valid/bitvec_8_max/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: Bitvector<8> = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("ff00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_9_max_8() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_9_max_8/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<9>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_5_random_6() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_5_random_6/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<5>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_2_zero_3() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_2_zero_3/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<2>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_2_max_3() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_2_max_3/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<2>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_16_zero_8() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_16_zero_8/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<16>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_4_max_5() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_4_max_5/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<4>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_512_zero_513() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_512_zero_513/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<512>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_512_max_513() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_512_max_513/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<512>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_3_max_4() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_3_max_4/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<3>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_8_max_9() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_8_max_9/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<8>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_32_max_33() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_32_max_33/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<32>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_512_random_513() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_512_random_513/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<512>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_32_zero_33() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_32_zero_33/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<32>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_1_max_2() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_1_max_2/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<1>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_5_zero_6() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_5_zero_6/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<5>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_9_zero_8() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_9_zero_8/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<9>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_1_random_2() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_1_random_2/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<1>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_4_zero_5() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_4_zero_5/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<4>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_8_random_9() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_8_random_9/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<8>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_9_random_8() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_9_random_8/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<9>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_4_random_5() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_4_random_5/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<4>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_3_zero_4() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_3_zero_4/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<3>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_16_max_8() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_16_max_8/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<16>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_32_random_33() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_32_random_33/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<32>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_1_zero_2() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_1_zero_2/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<1>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_8_zero_9() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_8_zero_9/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<8>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_0() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_0/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<0>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_2_random_3() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_2_random_3/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<2>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_3_random_4() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_3_random_4/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<3>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_5_max_6() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_5_max_6/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<5>>(&encoding);
}

#[test]
#[should_panic]
fn test_bitvector_bitvec_16_random_8() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/bitvector/invalid/bitvec_16_random_8/serialized.ssz_snappy",
    );

    deserialize::<Bitvector<16>>(&encoding);
}
