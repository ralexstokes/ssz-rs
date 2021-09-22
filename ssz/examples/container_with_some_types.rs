use hex;
use ssz::prelude::*;
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Debug, SimpleSerialize)]
enum Bar {
    A(u32),
    B(List<bool, 32>),
}

#[derive(PartialEq, Eq, Debug, Default, SimpleSerialize)]
struct Foo {
    a: u32,
    b: Vector<u32, 4>,
    c: bool,
    d: Bitlist<27>,
    e: Bar,
    f: Bitvector<4>,
}

fn main() {
    let mut foo = Foo {
        a: 16u32,
        b: Vector::from_iter([3u32, 2u32, 1u32, 10u32]),
        c: true,
        d: Bitlist::from_iter([
            true, false, false, true, true, false, true, false, true, true, false, false, true,
            true, false, true, false, true, true, false, false, true, true, false, true, false,
            true,
        ]),
        e: Bar::B(List::from_iter([true, true, false, false, false, true])),
        f: Bitvector::from_iter([false, true, false, true]),
    };

    let root = foo.hash_tree_root().expect("can make root");
    println!("{}", hex::encode(root));

    foo.b[2] = 44u32;
    foo.d.pop();
    match &mut foo.e {
        Bar::B(inner) => {
            inner.pop();
        }
        _ => {}
    }

    let encoding = match serialize(&foo) {
        Ok(encoding) => encoding,
        Err(e) => {
            eprintln!("some error encoding: {}", e);
            return;
        }
    };

    let restored_foo = match Foo::deserialize(&encoding) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("some error decoding: {}", e);
            return;
        }
    };

    println!("{:#?}", restored_foo);
    let root = restored_foo.hash_tree_root().expect("can make root");
    println!("{}", hex::encode(root));
}
