use ssz_rs::prelude::*;
use ssz_rs_derive::SimpleSerialize;

#[derive(Debug, SimpleSerialize)]
struct Foo {
    a: u8,
    b: u32,
}

#[derive(Debug, SimpleSerialize)]
#[ssz(transparent)]
enum Bar {
    A(u8),
    B(Foo),
}

#[derive(Debug, SimpleSerialize)]
struct Wrapper(Foo);

#[test]
fn test_transparent_helper() {
    let mut f = Foo { a: 23, b: 445 };
    let f_root = f.hash_tree_root().unwrap();
    let mut bar = Bar::B(f);
    let bar_root = bar.hash_tree_root().unwrap();
    assert_eq!(f_root, bar_root);
}
