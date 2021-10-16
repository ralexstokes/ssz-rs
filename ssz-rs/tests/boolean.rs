use hex;
use project_root;
use snap;
use ssz_rs::prelude::*;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn root_from_hex(hex_str: &str) -> Root {
    hex::decode(hex_str)
        .expect("can read hex")
        .try_into()
        .expect("can extract root")
}

fn serialize<T: SimpleSerialize>(value: &T) -> Vec<u8> {
    ssz_rs::serialize(value).expect("can serialize")
}

fn deserialize<T: SimpleSerialize>(encoding: &[u8]) -> T {
    ssz_rs::deserialize(encoding).expect("can deserialize")
}

fn hash_tree_root<T: SimpleSerialize>(value: &T) -> Root {
    let context = MerkleizationContext::new();
    value.hash_tree_root(&context).expect("can compute root")
}

// Return SSZ-encoded bytes from test file at `target_path`
fn read_ssz_snappy_from_test_data(target_path: &str) -> Vec<u8> {
    let project_root = project_root::get_project_root().unwrap();
    let target_path = PathBuf::from(target_path);
    let data_path = project_root.join(&target_path);
    let mut file = File::open(&data_path).expect("can read file");
    let mut data = vec![];
    let _ = file.read_to_end(&mut data).expect("can read file data");
    let mut decoder = snap::raw::Decoder::new();
    decoder
        .decompress_vec(&data)
        .expect("can decompress snappy")
}

#[test]
fn test_boolean_true() {
    let value = true;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/boolean/valid/true/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: bool = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0100000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
fn test_boolean_false() {
    let value = false;
    let encoding = serialize(&value);
    let expected_encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/boolean/valid/false/serialized.ssz_snappy",
    );
    assert_eq!(encoding, expected_encoding);

    let recovered_value: bool = deserialize(&expected_encoding);
    assert_eq!(recovered_value, value);

    let root = hash_tree_root(&value);
    let expected_root =
        root_from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(root, expected_root);
}

#[test]
#[should_panic]
fn test_boolean_byte_0x80() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/boolean/invalid/byte_0x80/serialized.ssz_snappy",
    );

    deserialize::<bool>(&encoding);
}

#[test]
#[should_panic]
fn test_boolean_byte_2() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/boolean/invalid/byte_2/serialized.ssz_snappy",
    );

    deserialize::<bool>(&encoding);
}

#[test]
#[should_panic]
fn test_boolean_byte_full() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/boolean/invalid/byte_full/serialized.ssz_snappy",
    );

    deserialize::<bool>(&encoding);
}

#[test]
#[should_panic]
fn test_boolean_byte_rev_nibble() {
    let encoding = read_ssz_snappy_from_test_data(
        "ssz-rs/tests/data/boolean/invalid/byte_rev_nibble/serialized.ssz_snappy",
    );

    deserialize::<bool>(&encoding);
}
