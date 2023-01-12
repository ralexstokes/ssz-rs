#![no_main]

use libfuzzer_sys::{arbitrary, fuzz_target};
use ssz_rs::prelude::*;

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize, arbitrary::Arbitrary)]
struct SingleFieldTestStruct {
    a: u8,
}

fuzz_target!(|foo: SingleFieldTestStruct| {
    // fuzzed code goes here
    serialize(&foo).unwrap();
});
