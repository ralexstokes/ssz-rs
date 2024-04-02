use ssz_rs::{prelude::*, proofs::ProofAndWitness};
use ssz_rs_derive::SimpleSerialize;
use std::fmt;

#[derive(Debug, Clone, SimpleSerialize, PartialEq, Eq)]
struct Foo {
    a: u8,
    b: u32,
    c: List<usize, 45>,
    d: U256,
}

#[derive(Debug, PartialEq, Eq, Serializable, HashTreeRoot)]
#[ssz(transparent)]
enum Bar {
    A(u8),
    B(Foo),
}

fn generalized_index_for_bar(
    object: &Bar,
    path: Path,
) -> Result<GeneralizedIndex, MerkleizationError> {
    match object {
        Bar::A(_) => u8::generalized_index(path),
        Bar::B(_) => Foo::generalized_index(path),
    }
}

fn prove_for_bar(object: &Bar, path: Path) -> Result<ProofAndWitness, MerkleizationError> {
    match object {
        Bar::A(value) => value.prove(path),
        Bar::B(value) => value.prove(path),
    }
}

#[derive(Debug, PartialEq, Eq, SimpleSerialize)]
struct Wrapper(Foo);

#[derive(Debug, PartialEq, Eq, SimpleSerialize)]
struct WrappedList(List<u8, 23>);

fn can_serde<T: Serializable + Eq + fmt::Debug>(data: &T) {
    let mut buf = vec![];
    let _ = data.serialize(&mut buf).unwrap();
    let recovered = T::deserialize(&buf).unwrap();
    assert_eq!(data, &recovered);
}

#[test]
fn test_transparent_helper() {
    // derive traits for "regular" types
    let container = Foo {
        a: 23,
        b: 445,
        c: List::<usize, 45>::try_from(vec![9, 8, 7, 6, 5, 4]).unwrap(),
        d: U256::from(234234),
    };
    can_serde(&container);

    let container_root = container.hash_tree_root().unwrap();

    let mut container_indices = vec![];
    let container_paths = [
        (vec!["a".into()], 4),
        (vec!["b".into()], 5),
        (vec!["c".into()], 6),
        (vec!["c".into(), 1.into()], 192),
        (vec!["c".into(), 43.into()], 202),
        (vec!["d".into()], 7),
    ];
    for (path, expected) in &container_paths {
        let index = Foo::generalized_index(path).unwrap();
        assert_eq!(index, *expected);
        container_indices.push(index);
    }

    let mut container_proofs = vec![];
    for pair in &container_paths {
        let path = &pair.0;
        let (proof, witness) = container.prove(path).unwrap();
        assert_eq!(witness, container_root);
        assert!(proof.verify(witness).is_ok());
        container_proofs.push((proof, witness));
    }

    // derive traits in "transparent" mode
    let inner = 22;
    let bar = Bar::A(inner);
    can_serde(&bar);

    let inner_root = inner.hash_tree_root().unwrap();
    let bar_root = bar.hash_tree_root().unwrap();
    assert_eq!(inner_root, bar_root);

    // `bar` just wraps a primitive type, so `path` is empty.
    let index = generalized_index_for_bar(&bar, &[]).unwrap();
    assert_eq!(index, 1);
    let result = generalized_index_for_bar(&bar, &["a".into()]);
    assert!(result.is_err());

    let path = &[];
    let (proof, witness) = prove_for_bar(&bar, path).unwrap();
    assert_eq!(witness, inner_root);
    assert_eq!(witness, bar_root);
    assert!(proof.verify(witness).is_ok());

    // repeat transparent with other variant
    let inner = container.clone();
    let inner_root = inner.hash_tree_root().unwrap();
    let bar = Bar::B(inner);
    can_serde(&bar);

    let bar_root = bar.hash_tree_root().unwrap();
    assert_eq!(inner_root, bar_root);

    for (i, (path, _)) in container_paths.iter().enumerate() {
        let index = generalized_index_for_bar(&bar, path).unwrap();
        assert_eq!(index, container_indices[i]);
    }

    for (i, pair) in container_paths.iter().enumerate() {
        let path = &pair.0;
        let (proof, witness) = prove_for_bar(&bar, path).unwrap();
        assert_eq!(witness, container_root);
        assert!(proof.verify(witness).is_ok());
        assert_eq!((proof, witness), container_proofs[i]);
    }

    // derive traits for "new type" pattern
    // for a wrapped type without "decoration"
    let mut buf = vec![];
    let container_serialization = container.serialize(&mut buf).unwrap();
    let wrapped = Wrapper(container);
    can_serde(&wrapped);
    buf.clear();
    let wrapped_serialization = wrapped.serialize(&mut buf).unwrap();
    assert_eq!(container_serialization, wrapped_serialization);

    let wrapped_root = wrapped.hash_tree_root().unwrap();
    assert_eq!(wrapped_root, container_root);

    let wrapped_paths = [
        (vec!["a".into()], 4),
        (vec!["b".into()], 5),
        (vec!["c".into()], 6),
        (vec!["c".into(), 1.into()], 192),
        (vec!["c".into(), 43.into()], 202),
        (vec!["d".into()], 7),
    ];
    for (i, (path, expected)) in container_paths.iter().enumerate() {
        let index = Wrapper::generalized_index(path).unwrap();
        assert_eq!(index, *expected);
        assert_eq!(index, container_indices[i]);
    }

    for (i, pair) in wrapped_paths.iter().enumerate() {
        let path = &pair.0;
        let (proof, witness) = wrapped.prove(path).unwrap();
        assert_eq!(witness, container_root);
        assert!(proof.verify(witness).is_ok());
        assert_eq!((proof, witness), container_proofs[i]);
    }

    // for a wrapped type with "decoration"
    let mut buf = vec![];
    let inner = List::<u8, 23>::try_from(vec![10, 11, 12]).unwrap();
    let inner_serialization = inner.serialize(&mut buf).unwrap();
    let inner_root = inner.hash_tree_root().unwrap();
    let inner_paths =
        [(vec![0.into()], 2), (vec![1.into()], 2), (vec![21.into()], 2), (vec![22.into()], 2)];
    let mut inner_indices = vec![];
    for (path, expected) in &inner_paths {
        let index = List::<u8, 23>::generalized_index(path).unwrap();
        assert_eq!(index, *expected);
        inner_indices.push(index);
    }
    let mut inner_proofs = vec![];
    for (path, _) in &inner_paths {
        let (proof, witness) = inner.prove(path).unwrap();
        assert_eq!(witness, inner_root);
        assert!(proof.verify(witness).is_ok());
        inner_proofs.push((proof, witness));
    }

    let wrapped = WrappedList(inner);
    can_serde(&wrapped);
    buf.clear();
    let wrapped_serialization = wrapped.serialize(&mut buf).unwrap();
    assert_eq!(inner_serialization, wrapped_serialization);

    let wrapped_root = wrapped.hash_tree_root().unwrap();
    assert_eq!(wrapped_root, inner_root);

    let wrapped_paths =
        [(vec![0.into()], 2), (vec![3.into()], 2), (vec![21.into()], 2), (vec![22.into()], 2)];
    for (i, (path, expected)) in wrapped_paths.iter().enumerate() {
        let index = WrappedList::generalized_index(path).unwrap();
        assert_eq!(index, *expected);
        assert_eq!(index, inner_indices[i]);
    }

    for (i, pair) in wrapped_paths.iter().enumerate() {
        let path = &pair.0;
        let (proof, witness) = wrapped.prove(path).unwrap();
        assert_eq!(witness, inner_root);
        assert!(proof.verify(witness).is_ok());
        assert_eq!((proof, witness), inner_proofs[i]);
    }
}
