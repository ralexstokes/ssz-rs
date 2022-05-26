use hex;
use ssz_rs::prelude::*;
use ssz_rs::std::{FromIterator, vec};

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
struct SingleFieldTestStruct {
    a: u8,
}

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
struct SmallTestStruct {
    a: u16,
    b: u16,
}

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

fn main() {
    let mut value = ComplexTestStruct {
        a: 51972,
        b: List::<u16, 128>::from_iter([48645]),
        c: 46,
        d: List::<u8, 256>::from_iter([105]),
        e: VarTestStruct {
            a: 1558,
            b: List::<u16, 1024>::from_iter([39947]),
            c: 65,
        },
        f: Vector::<FixedTestStruct, 4>::from_iter([
            FixedTestStruct {
                a: 70,
                b: 905948488145107787,
                c: 2675781419,
            },
            FixedTestStruct {
                a: 3,
                b: 12539792087931462647,
                c: 4719259,
            },
            FixedTestStruct {
                a: 73,
                b: 13544872847030609257,
                c: 2819826618,
            },
            FixedTestStruct {
                a: 159,
                b: 16328658841145598323,
                c: 2375225558,
            },
        ]),
        g: Vector::<VarTestStruct, 2>::from_iter([
            VarTestStruct {
                a: 30336,
                b: List::<u16, 1024>::from_iter([30909]),
                c: 240,
            },
            VarTestStruct {
                a: 64263,
                b: List::<u16, 1024>::from_iter([38121]),
                c: 100,
            },
        ]),
    };
    let encoding = serialize(&value).expect("can serialize");
    let expected_encoding = vec![
        4, 203, 71, 0, 0, 0, 46, 73, 0, 0, 0, 74, 0, 0, 0, 70, 75, 251, 176, 156, 89, 147, 146, 12,
        43, 47, 125, 159, 3, 247, 163, 104, 30, 119, 74, 6, 174, 155, 2, 72, 0, 73, 105, 229, 23,
        47, 23, 14, 249, 187, 186, 35, 19, 168, 159, 115, 97, 6, 253, 179, 12, 155, 226, 214, 16,
        147, 141, 83, 0, 0, 0, 5, 190, 105, 22, 6, 7, 0, 0, 0, 65, 11, 156, 8, 0, 0, 0, 17, 0, 0,
        0, 128, 118, 7, 0, 0, 0, 240, 189, 120, 7, 251, 7, 0, 0, 0, 100, 233, 148,
    ];
    assert_eq!(encoding, expected_encoding);

    let recovered_value: ComplexTestStruct =
        deserialize(&expected_encoding).expect("can deserialize");
    assert_eq!(recovered_value, value);

    let root = value.hash_tree_root().expect("can find root");
    let expected_root = "69b0ce69dfbc8abb8ae4fba564dcb813f5cc5b93c76d2b3d0689687c35821036";
    assert_eq!(hex::encode(root), expected_root);
}
