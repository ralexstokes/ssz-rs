use ssz_rs::prelude::*;
use std::{fs::File, io::Read, path::PathBuf};

pub fn root_from_hex(hex_str: &str) -> Node {
    let hex_str = hex_str.trim_start_matches("0x");
    hex::decode(hex_str).expect("can read hex").as_slice().try_into().expect("can extract root")
}

pub fn serialize<T: SimpleSerialize>(value: &T) -> Vec<u8> {
    ssz_rs::serialize(value).expect("can serialize")
}

pub fn deserialize<T: SimpleSerialize>(encoding: &[u8]) -> T {
    ssz_rs::deserialize(encoding).expect("can deserialize")
}

pub fn hash_tree_root<T: SimpleSerialize>(value: &T) -> Node {
    value.hash_tree_root().expect("can compute root")
}

// Return SSZ-encoded bytes from test file at `target_path`
pub fn read_ssz_snappy_from_test_data(target_path: &str) -> Vec<u8> {
    let project_root = project_root::get_project_root().unwrap();
    let target_path = PathBuf::from(target_path);
    let data_path = project_root.join(target_path);
    let mut file = File::open(data_path).expect("can read file");
    let mut data = vec![];
    let _ = file.read_to_end(&mut data).expect("can read file data");
    let mut decoder = snap::raw::Decoder::new();
    decoder.decompress_vec(&data).expect("can decompress snappy")
}
