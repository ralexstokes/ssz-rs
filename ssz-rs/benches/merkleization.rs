use criterion::{criterion_group, criterion_main, Criterion};
use ssz_rs::{PathElement, Prove};

fn generate_id(name: &str) -> String {
    let tag = if cfg!(feature = "hashtree") {
        "hashtree"
    } else if cfg!(feature = "sha2-asm") {
        "sha256-asm"
    } else {
        "sha256"
    };

    format!("{}_{}", name, tag)
}

fn bench_merkleization(c: &mut Criterion) {
    use ssz_rs::{HashTreeRoot, List};

    let inner: Vec<List<u8, 1073741824>> = vec![
        vec![0u8, 1u8, 2u8].try_into().unwrap(),
        vec![3u8, 4u8, 5u8].try_into().unwrap(),
        vec![6u8, 7u8, 8u8].try_into().unwrap(),
        vec![9u8, 10u8, 11u8].try_into().unwrap(),
    ];

    // Emulate a transactions tree
    let outer: List<List<u8, 1073741824>, 1048576> = List::try_from(inner).unwrap();

    c.bench_function(&generate_id("hash_tree_root"), |b| {
        b.iter(|| {
            let _ = outer.hash_tree_root().unwrap();
        })
    });

    // let root = outer.hash_tree_root().unwrap();
    let index = PathElement::from(1);
    c.bench_function(&generate_id("generate_proof"), |b| {
        b.iter(|| {
            let (_proof, _witness) = outer.prove(&[index.clone()]).unwrap();
        })
    });
}

criterion_group!(benches, bench_merkleization,);

criterion_main!(benches);
