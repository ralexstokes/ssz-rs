#![cfg(feature = "serde")]
use serde_json;
use ssz_rs::prelude::*;

#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
struct FixedTestStruct {
    a: u8,
    b: u64,
    c: u32,
}

#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
struct VarTestStruct {
    a: u16,
    b: List<u16, 1024>,
    c: u8,
}

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize, serde::Serialize, serde::Deserialize)]
struct ComplexTestStruct {
    a: u16,
    b: List<u16, 128>,
    c: u8,
    d: List<u8, 256>,
    e: VarTestStruct,
    f: Vector<FixedTestStruct, 4>,
    g: Vector<VarTestStruct, 2>,
    h: Bitvector<9>,
    i: Bitlist<32>,
    j: U256,
}

fn main() {
    let value = ComplexTestStruct {
        a: 51972,
        b: List::<u16, 128>::from_iter([48645]),
        c: 46,
        d: List::<u8, 256>::from_iter([105]),
        e: VarTestStruct { a: 1558, b: List::<u16, 1024>::from_iter([39947]), c: 65 },
        f: Vector::<FixedTestStruct, 4>::from_iter([
            FixedTestStruct { a: 70, b: 905948488145107787, c: 2675781419 },
            FixedTestStruct { a: 3, b: 12539792087931462647, c: 4719259 },
            FixedTestStruct { a: 73, b: 13544872847030609257, c: 2819826618 },
            FixedTestStruct { a: 159, b: 16328658841145598323, c: 2375225558 },
        ]),
        g: Vector::<VarTestStruct, 2>::from_iter([
            VarTestStruct { a: 30336, b: List::<u16, 1024>::from_iter([30909]), c: 240 },
            VarTestStruct { a: 64263, b: List::<u16, 1024>::from_iter([38121]), c: 100 },
        ]),
        h: Bitvector::from_iter([true, false, false, true, false, false, false, true, true]),
        i: Bitlist::from_iter([true, false, true, true]),
        j: U256::from_bytes_le([12u8; 32]),
    };
    let json_repr = serde_json::to_value(&value).unwrap();
    dbg!(&json_repr);
    let roundtrip_value: ComplexTestStruct = serde_json::from_value(json_repr).unwrap();
    assert_eq!(value, roundtrip_value);
    dbg!(roundtrip_value);
}
