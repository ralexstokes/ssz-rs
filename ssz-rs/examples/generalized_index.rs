use ssz_rs::prelude::*;

#[derive(Default, Debug, SimpleSerialize)]
struct Bar {
    c: u8,
    f: Foo,
    a: List<u8, 25>,
}

impl Bar {
    fn __ssz_rs_generalized_index_by_field(
        root: GeneralizedIndex,
        path: &[PathElement],
        field: &str,
    ) -> Result<GeneralizedIndex, PathError> {
        match field {
            "c" => {
                let chunk_position = 0;
                let root = root + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                u8::generalized_index(root, path)
            }
            "f" => {
                let chunk_position = 1;
                let root = root + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                Foo::generalized_index(root, path)
            }
            "a" => {
                let chunk_position = 2;
                let root = root + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                List::<u8, 25>::generalized_index(root, path)
            }
            s => Err(PathError::Type(PathElement::Field(s))),
        }
    }
}

impl Indexed for Bar {
    fn chunk_count() -> usize {
        3
    }

    fn generalized_index(
        root: GeneralizedIndex,
        path: &[PathElement],
    ) -> Result<GeneralizedIndex, PathError> {
        if let Some((next, rest)) = path.split_first() {
            match next {
                PathElement::Field(field) => {
                    Self::__ssz_rs_generalized_index_by_field(root, rest, *field)
                }
                elem => Err(PathError::Type(elem.clone())),
            }
        } else {
            Ok(root)
        }
    }
}

#[derive(Default, Debug, SimpleSerialize)]
struct Foo {
    x: Vector<u8, 32>,
    y: List<Qux, 256>,
}

impl Foo {
    fn __ssz_rs_generalized_index_by_field(
        root: GeneralizedIndex,
        path: &[PathElement],
        field: &str,
    ) -> Result<GeneralizedIndex, PathError> {
        match field {
            "x" => {
                let chunk_position = 0;
                let root = root + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                Vector::<u8, 32>::generalized_index(root, path)
            }
            "y" => {
                let chunk_position = 1;
                let root = root + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                List<Qux, 256>::generalized_index(root, path)
            }
            s => Err(PathError::Type(PathElement::Field(s))),
        }
    }
}

impl Indexed for Foo {
    fn chunk_count() -> usize {
        2
    }

    fn generalized_index(
        root: GeneralizedIndex,
        path: &[PathElement],
    ) -> Result<GeneralizedIndex, PathError> {
        if let Some((next, rest)) = path.split_first() {
            match next {
                PathElement::Field(field) => {
                    Self::__ssz_rs_generalized_index_by_field(root, rest, *field)
                }
                elem => Err(PathError::Type(elem.clone())),
            }
        } else {
            Ok(root)
        }
    }
}

#[derive(Default, Debug, SimpleSerialize)]
struct Qux {
    a: Vector<u16, 8>,
}

impl Qux{
    fn __ssz_rs_generalized_index_by_field(
        root: GeneralizedIndex,
        path: &[PathElement],
        field: &str,
    ) -> Result<GeneralizedIndex, PathError> {
        match field {
            "a" => {
                let chunk_position = 0;
                let root = root + get_power_of_two_ceil(Self::chunk_count()) + chunk_position;
                Vector::<u16, 8>::generalized_index(root, path)
            }
            s => Err(PathError::Type(PathElement::Field(s))),
        }
    }
}

impl Indexed for Qux {
    fn chunk_count() -> usize {
        1
    }

    fn generalized_index(
        root: GeneralizedIndex,
        path: &[PathElement],
    ) -> Result<GeneralizedIndex, PathError> {
        if let Some((next, rest)) = path.split_first() {
            match next {
                PathElement::Field(field) => {
                    Self::__ssz_rs_generalized_index_by_field(root, rest, *field)
                }
                elem => Err(PathError::Type(elem.clone())),
            }
        } else {
            Ok(root)
        }
    }
}

fn main() {
    let root = 1;

    // {bit,}vector
    let path = [Path::Index(2)];
    let index = Vector::<u8, 16>::generalized_index(root, &path);

    // {bit,}list
    let path = [Path::Index(2)];
    let index = List::<u8, 256>::generalized_index(root, &path);
    let path = [Path::Length];
    let index = List::<u8, 256>::generalized_index(root, &path);

    // containers
    let path = [Path::Field("c")];
    let index = Bar::generalized_index(root, &path);

    // nested access
    let path = [Path::Field("a"), Path::Index(2)];
    let index = Bar::generalized_index(root, &path);

    let path =
        [Path::Field("f"), Path::Field("y"), Path::Index(2), Path::Field("a"), Path::Index(3)];
    let root = 1;
    let index = Bar::generalized_index(root, &path);
    dbg!(index);
}
