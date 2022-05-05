mod test_utils;

use ssz_rs::prelude::*;
use test_utils::{
    deserialize, hash_tree_root, read_ssz_snappy_from_test_data, root_from_hex, serialize,
};

#[test]
fn test_uints_uint_256_zero_2() {
    let mut value = U256::from_bytes_le([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_zero_3() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_zero_4() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_random_2() {
    let mut value = 1966913376797472348559631900882537126;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("a68a04f1c6f71282ca13121251d07a0100000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_max_0() {
    let mut value = 4294967295;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffff00000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_random_1() {
    let mut value = 12900;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("6432000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_zero_3() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_random_0() {
    let mut value = 225;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("e100000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_max_0() {
    let mut value = 65535;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffff000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_zero_4() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_zero_4() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_zero_3() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_random_0() {
    let mut value = 11001;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("f92a000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_max_1() {
    let mut value = 4294967295;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffff00000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_zero_4() {
    let mut value = U256::from_bytes_le([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_random_3() {
    let mut value = 223686144064414504608552983434269426145;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("e101ce24c16ec3b57c2f0b79616248a800000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_random_4() {
    let mut value = 199925590919705556758473559487562637786;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("dae1c72a086dde0deb118413aa44689600000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_zero_3() {
    let mut value = U256::from_bytes_le([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_zero_2() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_zero_2() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_max_1() {
    let mut value = 65535;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffff000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_random_1() {
    let mut value = 59;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("3b00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_zero_2() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_zero_0() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_max_2() {
    let mut value = 340282366920938463463374607431768211455;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffff00000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_max_1() {
    let mut value = 255;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ff00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_max_3() {
    let mut value = 340282366920938463463374607431768211455;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffff00000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_last_byte_empty() {
    let mut value = U256::from_bytes_le([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_last_byte_empty/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_max_4() {
    let mut value = 340282366920938463463374607431768211455;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffff00000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_zero_1() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_max_0() {
    let mut value = 255;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ff00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_max_4() {
    let mut value = U256::from_bytes_le([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_zero_0() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_random_2() {
    let mut value = U256::from_bytes_le([
        145, 36, 54, 124, 134, 65, 119, 96, 224, 3, 87, 209, 164, 118, 23, 209, 5, 72, 9, 168, 251,
        195, 102, 65, 122, 101, 27, 164, 66, 115, 0, 49,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("9124367c86417760e00357d1a47617d1054809a8fbc366417a651ba442730031");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_random_3() {
    let mut value = 11891402719218752485;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("e5db2510c5bf06a5000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_max_3() {
    let mut value = U256::from_bytes_le([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_random_4() {
    let mut value = 15683022699148686111;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("1f33257b0d4aa5d9000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_max_2() {
    let mut value = 18446744073709551615;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffff000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_random_0() {
    let mut value = 3387753032;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("4802edc900000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_max_2() {
    let mut value = U256::from_bytes_le([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_random_4() {
    let mut value = U256::from_bytes_le([
        236, 44, 123, 92, 134, 169, 87, 238, 98, 219, 210, 219, 26, 37, 128, 52, 156, 71, 217, 131,
        206, 187, 193, 227, 34, 128, 209, 179, 17, 9, 210, 107,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ec2c7b5c86a957ee62dbd2db1a2580349c47d983cebbc1e32280d1b31109d26b");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_zero_1() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_random_3() {
    let mut value = U256::from_bytes_le([
        9, 220, 230, 65, 45, 6, 68, 219, 208, 26, 176, 18, 183, 94, 87, 176, 157, 70, 34, 109, 52,
        201, 18, 243, 217, 129, 175, 51, 196, 80, 238, 25,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("09dce6412d0644dbd01ab012b75e57b09d46226d34c912f3d981af33c450ee19");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_random_2() {
    let mut value = 10680714365983390887;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("a7fcd98320853994000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_random_1() {
    let mut value = 2676973563;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("fb5f8f9f00000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_max_4() {
    let mut value = 18446744073709551615;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffff000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_max_3() {
    let mut value = 18446744073709551615;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffff000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_last_byte_empty() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_last_byte_empty/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_random_4() {
    let mut value = 17;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("1100000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_max_4() {
    let mut value = 65535;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffff000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_zero_0() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_zero_0() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_random_3() {
    let mut value = 46;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("2e00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_max_3() {
    let mut value = 65535;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffff000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_random_1() {
    let mut value = 226427817519480008631815531407103573168;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("b03c1174ebe365e018a5b887516958aa00000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_zero_1() {
    let mut value = U256::from_bytes_le([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_zero_0() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_random_2() {
    let mut value = 46482;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("92b5000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_max_4() {
    let mut value = 4294967295;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffff00000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_max_3() {
    let mut value = 4294967295;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffff00000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_max_2() {
    let mut value = 65535;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffff000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_random_2() {
    let mut value = 3;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0300000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_zero_1() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_zero_1() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_random_4() {
    let mut value = 2284;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ec08000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_max_2() {
    let mut value = 4294967295;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffff00000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_random_3() {
    let mut value = 31039;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("3f79000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_zero_0() {
    let mut value = U256::from_bytes_le([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_zero_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_zero_1() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_zero_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_random_0() {
    let mut value = 317658863013703600909281237913711302754;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("62583644e66ec83fc2a6cda723dffaee00000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_max_2() {
    let mut value = 255;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_max_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ff00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_zero_4() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_zero_3() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_max_1() {
    let mut value = 340282366920938463463374607431768211455;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffff00000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_max_3() {
    let mut value = 255;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_max_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ff00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_max_4() {
    let mut value = 255;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_max_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ff00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_max_0() {
    let mut value = 340282366920938463463374607431768211455;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffff00000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_zero_2() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_random_3() {
    let mut value = 638037343;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_random_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("5fad072600000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_random_4() {
    let mut value = 4144220671;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_random_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffc903f700000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_max_1() {
    let mut value = 18446744073709551615;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffff000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_last_byte_empty() {
    let mut value = 16777215;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_last_byte_empty/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffff0000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_128_last_byte_empty() {
    let mut value = 1329227995784915872903807060280344575;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_128_last_byte_empty/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u128 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffff0000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_16_last_byte_empty() {
    let mut value = 255;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_16_last_byte_empty/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u16 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ff00000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_zero_4() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_zero_4/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_max_0() {
    let mut value = U256::from_bytes_le([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_zero_3() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_zero_3/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_random_1() {
    let mut value = U256::from_bytes_le([
        160, 200, 243, 199, 115, 30, 235, 132, 127, 224, 146, 208, 192, 97, 24, 112, 2, 157, 177,
        75, 95, 22, 105, 70, 180, 97, 182, 31, 39, 79, 21, 199,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("a0c8f3c7731eeb847fe092d0c0611870029db14b5f166946b461b61f274f15c7");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_random_0() {
    let mut value = 8594311575614880821;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("357c8de9d7204577000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_max_0() {
    let mut value = 18446744073709551615;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_max_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffff000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_32_random_2() {
    let mut value = 2644908285;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_32_random_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u32 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("fd18a69d00000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_8_zero_2() {
    let mut value = 0;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_8_zero_2/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u8 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_random_0() {
    let mut value = U256::from_bytes_le([
        58, 55, 99, 28, 168, 145, 249, 244, 255, 81, 153, 135, 170, 128, 39, 36, 202, 1, 166, 171,
        97, 55, 46, 78, 36, 161, 66, 116, 168, 139, 34, 10,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_random_0/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("3a37631ca891f9f4ff519987aa802724ca01a6ab61372e4e24a14274a88b220a");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_random_1() {
    let mut value = 12453893770581738044;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_random_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("3c82f999661ed5ac000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_256_max_1() {
    let mut value = U256::from_bytes_le([
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    ]);
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_256_max_1/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: U256 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    assert_eq!(root, expected_root);
}

#[test]
fn test_uints_uint_64_last_byte_empty() {
    let mut value = 72057594037927935;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/valid/uint_64_last_byte_empty/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: u64 = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&mut value);
    let expected_root =
        root_from_hex("ffffffffffffff00000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
#[should_panic]
fn test_uints_uint_128_one_too_high() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_128_one_too_high/serialized.ssz_snappy",
    );

    deserialize::<u128>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_8_one_byte_longer() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_8_one_byte_longer/serialized.ssz_snappy",
    );

    deserialize::<u8>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_8_one_too_high() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_8_one_too_high/serialized.ssz_snappy",
    );

    deserialize::<u8>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_16_one_byte_shorter() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_16_one_byte_shorter/serialized.ssz_snappy",
    );

    deserialize::<u16>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_32_one_byte_shorter() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_32_one_byte_shorter/serialized.ssz_snappy",
    );

    deserialize::<u32>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_64_one_too_high() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_64_one_too_high/serialized.ssz_snappy",
    );

    deserialize::<u64>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_256_one_byte_longer() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_256_one_byte_longer/serialized.ssz_snappy",
    );

    deserialize::<U256>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_32_one_byte_longer() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_32_one_byte_longer/serialized.ssz_snappy",
    );

    deserialize::<u32>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_128_one_byte_longer() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_128_one_byte_longer/serialized.ssz_snappy",
    );

    deserialize::<u128>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_16_one_byte_longer() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_16_one_byte_longer/serialized.ssz_snappy",
    );

    deserialize::<u16>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_32_one_too_high() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_32_one_too_high/serialized.ssz_snappy",
    );

    deserialize::<u32>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_64_one_byte_longer() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_64_one_byte_longer/serialized.ssz_snappy",
    );

    deserialize::<u64>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_16_one_too_high() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_16_one_too_high/serialized.ssz_snappy",
    );

    deserialize::<u16>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_256_one_too_high() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_256_one_too_high/serialized.ssz_snappy",
    );

    deserialize::<U256>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_128_one_byte_shorter() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_128_one_byte_shorter/serialized.ssz_snappy",
    );

    deserialize::<u128>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_256_one_byte_shorter() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_256_one_byte_shorter/serialized.ssz_snappy",
    );

    deserialize::<U256>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_64_one_byte_shorter() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_64_one_byte_shorter/serialized.ssz_snappy",
    );

    deserialize::<u64>(&encoding);
}

#[test]
#[should_panic]
fn test_uints_uint_8_one_byte_shorter() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz_rs/tests/data/uints/invalid/uint_8_one_byte_shorter/serialized.ssz_snappy",
    );

    deserialize::<u8>(&encoding);
}
