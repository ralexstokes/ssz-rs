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
    let path = &[PathElement::Index(2)];
    let index = get_generalized_index::<Vector<u8, 16>>(path).unwrap();
    dbg!(index);

    let path = &[PathElement::Index(2)];
    let index = get_generalized_index::<List<u8, 256>>(path).unwrap();
    dbg!(index);
    let path = &[PathElement::Length];
    let index = List::<u8, 256>::generalized_index(path).unwrap();
    dbg!(index);

    // containers
    let path = &[PathElement::Field("c".into())];
    let index = Bar::generalized_index(path).unwrap();
    dbg!(index);

    // nested access
    let path = &[PathElement::Field("a".into()), PathElement::Index(2)];
    let index = Bar::generalized_index(path).unwrap();
    dbg!(index);

    let path = &[
        PathElement::Field("f".into()),
        PathElement::Field("y".into()),
        PathElement::Index(2),
        PathElement::Field("a".into()),
        PathElement::Index(3),
    ];
    let index = Bar::generalized_index(path).unwrap();
    dbg!(index);

    let path =
        &[PathElement::Field("f".into()), PathElement::Field("y".into()), PathElement::Length];
    let index = Bar::generalized_index(path).unwrap();
    dbg!(index);
}
