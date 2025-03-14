mod test_utils;

use ssz_rs::prelude::*;

#[derive(PartialEq, Eq, Debug, Default, Clone, SimpleSerialize)]
struct FixedTestStruct {
    a: u8,
    b: u64,
    c: u32,
}

#[derive(PartialEq, Eq, Debug, Default, Clone, SimpleSerialize)]
struct VarTestStruct {
    a: u16,
    b: List<u16, 1024>,
    c: u8,
}

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
struct ComplexTestStruct {
    a: u16,
    b: List<u16, 128>,
    c: u8,
    d: List<u8, 256>,
    e: VarTestStruct,
    f: Vector<FixedTestStruct, 4>,
    g: Vector<VarTestStruct, 2>,
}

#[test]
fn test_out_of_range() {
    let bytes = Vec::<u8>::from([
        255, 255, 71, 0, 0, 0, 224, 71, 0, 0, 0, 72, 0, 0, 0, 0, 237, 38, 85, 218, 185, 193, 20,
        75, 188, 112, 128, 90, 255, 9, 3, 93, 80, 107, 2, 94, 246, 150, 222, 8, 235, 145, 255, 255,
        255, 255, 255, 255, 255, 255, 74, 32, 4, 239, 42, 187, 195, 198, 64, 69, 12, 145, 38, 255,
        255, 255, 255, 79, 0, 0, 0, 0, 209, 172, 7, 0, 0, 0, 253, 8, 0, 0, 0, 55, 0, 0, 0, 255,
        255, 7, 0, 0, 0, 253, 0, 0, 255, 255, 7, 0, 0, 0, 0, 47, 69,
    ]);
    let result = ssz_rs::deserialize::<ComplexTestStruct>(&bytes);
    assert!(matches!(
        result.unwrap_err(),
        ssz_rs::DeserializeError::ExpectedFurtherInput { provided: 26, expected: 55 }
    ));
}

#[test]
fn test_expected_further_input() {
    let bytes = Vec::<u8>::from([255, 255, 71, 0, 0, 0, 224, 71, 0, 0, 0]);
    let result = ssz_rs::deserialize::<ComplexTestStruct>(&bytes);
    assert!(matches!(
        result.unwrap_err(),
        ssz_rs::DeserializeError::ExpectedFurtherInput { provided: 11, expected: 71 }
    ));
}
