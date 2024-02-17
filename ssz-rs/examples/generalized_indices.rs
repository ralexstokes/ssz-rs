use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize, Indexed)]
struct Bar {
    c: u8,
    f: Foo,
    a: List<u8, 25>,
}

#[derive(Default, Debug, SimpleSerialize, Indexed)]
struct Foo {
    x: Vector<u8, 32>,
    y: List<Qux, 256>,
}

#[derive(Default, Debug, SimpleSerialize, Indexed)]
struct Qux {
    a: Vector<u16, 8>,
}

fn main() {
    let path = &[2.into()];
    let index = Vector::<u8, 16>::generalized_index(path).unwrap();
    dbg!(index);

    let path = &[2.into()];
    let index = get_generalized_index::<List<u8, 256>>(path).unwrap();
    dbg!(index);
    let path = &[PathElement::Length];
    let index = List::<u8, 256>::generalized_index(path).unwrap();
    dbg!(index);

    // containers
    let path = &["c".into()];
    let index = Bar::generalized_index(path).unwrap();
    dbg!(index);

    // nested access
    let path = &["a".into(), 2.into()];
    let index = Bar::generalized_index(path).unwrap();
    dbg!(index);

    let path = &["f".into(), "y".into(), 2.into(), "a".into(), 3.into()];
    let index = Bar::generalized_index(path).unwrap();
    dbg!(index);

    let path = &["f".into(), "y".into(), PathElement::Length];
    let index = Bar::generalized_index(path).unwrap();
    dbg!(index);
}
