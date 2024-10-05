use ssz_rs::prelude::*;

const VALIDATOR_REGISTRY_LIMIT: usize = 1099511627776;

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
    h: List<u64, VALIDATOR_REGISTRY_LIMIT>,
}

fn compute_and_verify_proof<T: SimpleSerialize>(data: &T, path: Path) {
    let (proof, witness) = data.prove(path).unwrap();
    assert_eq!(witness, data.hash_tree_root().unwrap());
    dbg!(&proof, &witness);
    let result = proof.verify(witness);
    if let Err(err) = result {
        panic!("{err} for {proof:?} with witness {witness}")
    }
}

fn main() {
    let data = 8u8;
    let path = &[];
    compute_and_verify_proof(&data, path);

    let data = ComplexTestStruct {
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
        h: List::<u64, VALIDATOR_REGISTRY_LIMIT>::try_from(vec![10000, 4000]).unwrap(),
    };

    let path = &["a".into()];
    compute_and_verify_proof(&data, path);

    let path = &["b".into(), 0.into()];
    compute_and_verify_proof(&data, path);

    let path = &["e".into(), "a".into()];
    compute_and_verify_proof(&data, path);

    let path = &["e".into(), "b".into(), 0.into()];
    compute_and_verify_proof(&data, path);

    let path = &["e".into(), "b".into(), 33.into()];
    compute_and_verify_proof(&data, path);

    let path = &["g".into(), 1.into(), "b".into(), 0.into()];
    compute_and_verify_proof(&data, path);

    let path = &["h".into(), 1.into()];
    compute_and_verify_proof(&data, path);
}
