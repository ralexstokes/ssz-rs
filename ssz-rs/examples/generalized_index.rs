use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
struct Bar {
    f: Foo,
    a: List<u8, 25>,
}

enum BarPath {
    F(FooPath),
    A(ListPath<Done>),
}

impl Indexed for Bar {
    type Path = BarPath;

    fn chunk_count() -> usize {
        2
    }

    fn generalized_index(root: GeneralizedIndex, path: &Self::Path) -> GeneralizedIndex {
        match path {
            BarPath::F(path) => {
                let chunk_position = 0;
                let base_index = 1;
                let root =
                    root * base_index + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                Foo::generalized_index(root, path)
            }
            BarPath::A(path) => {
                let chunk_position = 1;
                let base_index = 1;
                let root =
                    root * base_index + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                List::<u8, 25>::generalized_index(root, path)
            }
        }
    }
}

#[derive(Default, Debug, SimpleSerialize)]
struct Foo {
    x: Vector<u8, 32>,
    y: List<Qux, 256>,
}

enum FooPath {
    X(IndexPath<Done>),
    Y(ListPath<QuxPath>),
}

impl Indexed for Foo {
    type Path = FooPath;

    fn chunk_count() -> usize {
        2
    }

    fn generalized_index(root: GeneralizedIndex, path: &Self::Path) -> GeneralizedIndex {
        match path {
            FooPath::X(path) => {
                let chunk_position = 0;
                let base_index = 1;
                let root =
                    root * base_index + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                Vector::<u8, 32>::generalized_index(root, path)
            }
            FooPath::Y(path) => {
                let chunk_position = 1;
                let base_index = 1;
                let root =
                    root * base_index + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                List::<Qux, 256>::generalized_index(root, path)
            }
        }
    }
}

#[derive(Default, Debug, SimpleSerialize)]
struct Qux {
    a: Vector<u16, 8>,
}

enum QuxPath {
    A(IndexPath<Done>),
}

impl Indexed for Qux {
    type Path = QuxPath;

    fn chunk_count() -> usize {
        1
    }

    fn generalized_index(root: GeneralizedIndex, path: &Self::Path) -> GeneralizedIndex {
        match path {
            QuxPath::A(path) => {
                let chunk_position = 0;
                let base_index = 1;
                let root =
                    root * base_index + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                Vector::<u16, 8>::generalized_index(root, path)
            }
        }
    }
}

fn main() {
    let path = BarPath::F(FooPath::Y(ListPath::Index((2, QuxPath::A((3, Done))))));
    let root = 1;
    let index = Bar::generalized_index(root, &path);
    dbg!(index);
}
