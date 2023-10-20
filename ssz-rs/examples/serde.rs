#![cfg(feature = "serde")]
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
        b: List::<u16, 128>::try_from(vec![48645]).unwrap(),
        c: 46,
        d: List::<u8, 256>::try_from(vec![105]).unwrap(),
        e: VarTestStruct { a: 1558, b: List::<u16, 1024>::try_from(vec![39947]).unwrap(), c: 65 },
        f: Vector::<FixedTestStruct, 4>::try_from(vec![
            FixedTestStruct { a: 70, b: 905948488145107787, c: 2675781419 },
            FixedTestStruct { a: 3, b: 12539792087931462647, c: 4719259 },
            FixedTestStruct { a: 73, b: 13544872847030609257, c: 2819826618 },
            FixedTestStruct { a: 159, b: 16328658841145598323, c: 2375225558 },
        ])
        .unwrap(),
        g: Vector::<VarTestStruct, 2>::try_from(vec![
            VarTestStruct {
                a: 30336,
                b: List::<u16, 1024>::try_from(vec![30909]).unwrap(),
                c: 240,
            },
            VarTestStruct {
                a: 64263,
                b: List::<u16, 1024>::try_from(vec![38121]).unwrap(),
                c: 100,
            },
        ])
        .unwrap(),
        h: Bitvector::try_from(
            [true, false, false, true, false, false, false, true, true].as_ref(),
        )
        .unwrap(),
        i: Bitlist::try_from([true, false, true, true].as_ref()).unwrap(),
        j: U256::from_le_bytes([12u8; 32]),
    };
    let json_repr = serde_json::to_value(&value).unwrap();
    dbg!(&json_repr);
    let roundtrip_value: ComplexTestStruct = serde_json::from_value(json_repr).unwrap();
    assert_eq!(value, roundtrip_value);
    dbg!(roundtrip_value);
}
