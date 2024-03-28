use crate::{
    lib::*,
    merkleization::{MerkleizationError as Error, BYTES_PER_CHUNK},
};

#[derive(Debug, Clone)]
pub enum PathElement {
    Index(usize),
    Field(String),
    Length,
}

impl From<&str> for PathElement {
    fn from(value: &str) -> Self {
        PathElement::Field(value.to_string())
    }
}

impl From<usize> for PathElement {
    fn from(value: usize) -> Self {
        PathElement::Index(value)
    }
}

pub type Path<'a> = &'a [PathElement];

pub trait Indexed {
    fn item_length() -> usize {
        BYTES_PER_CHUNK
    }

    /// Return the chunk count when merkleizing this type.
    /// Default implementation for "basic" types that fit in one chunk.
    fn chunk_count() -> usize {
        1
    }

    /// Compute the generalized index starting from `parent` and following `path` through the
    /// implementing type.
    /// Default implementation for "basic" types with no further children in the Merkle tree.
    fn compute_generalized_index(
        parent: GeneralizedIndex,
        path: Path,
    ) -> Result<GeneralizedIndex, Error> {
        if path.is_empty() {
            Ok(parent)
        } else {
            Err(Error::InvalidPath(path.to_vec()))
        }
    }

    fn generalized_index(path: Path) -> Result<GeneralizedIndex, Error>
    where
        Self: Sized,
    {
        get_generalized_index::<Self>(path)
    }
}

pub fn get_generalized_index<T: Indexed>(path: Path) -> Result<GeneralizedIndex, Error> {
    let root = default_generalized_index();
    T::compute_generalized_index(root, path)
}

// Return base 2 logarithm of `x`.
// `None` is returned if `x` is `0` as this logarithm is undefined.
pub fn log_2(x: usize) -> Option<u32> {
    x.checked_ilog2()
}

pub fn get_power_of_two_ceil(x: usize) -> usize {
    match x {
        x if x <= 1 => 1,
        2 => 2,
        x => 2 * get_power_of_two_ceil((x + 1) / 2),
    }
}

/// Represents a "generalized index" from the SSZ spec.
/// Note: the default `GeneralizedIndex` is _not_ what `Default::default()`
/// provides. See the function `default_generalized_index` when working with
/// these values.
pub type GeneralizedIndex = usize;

pub const fn default_generalized_index() -> GeneralizedIndex {
    1
}

pub fn get_path_length(index: GeneralizedIndex) -> Result<usize, Error> {
    let length = log_2(index).ok_or(Error::InvalidGeneralizedIndex)?;
    Ok(length as usize)
}

pub const fn get_bit(index: GeneralizedIndex, position: usize) -> bool {
    index & (1 << position) > 0
}

pub const fn sibling(index: GeneralizedIndex) -> GeneralizedIndex {
    index ^ 1
}

pub const fn child_left(index: GeneralizedIndex) -> GeneralizedIndex {
    index * 2
}

pub const fn child_right(index: GeneralizedIndex) -> GeneralizedIndex {
    index * 2 + 1
}

pub const fn parent(index: GeneralizedIndex) -> GeneralizedIndex {
    index / 2
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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

    #[test]
    fn test_basic_generalized_index_computation() {
        let mut indices = vec![];

        let path = &[2.into()];
        let index = Vector::<u8, 16>::generalized_index(path).unwrap();
        indices.push(index);

        let path = &[2.into()];
        let index = get_generalized_index::<List<u8, 256>>(path).unwrap();
        indices.push(index);

        let path = &[PathElement::Length];
        let index = List::<u8, 256>::generalized_index(path).unwrap();
        indices.push(index);

        // containers
        let path = &["c".into()];
        let index = Bar::generalized_index(path).unwrap();
        indices.push(index);

        // nested access
        let path = &["a".into(), 2.into()];
        let index = Bar::generalized_index(path).unwrap();
        indices.push(index);

        let path = &["f".into(), "y".into(), 2.into(), "a".into(), 3.into()];
        let index = Bar::generalized_index(path).unwrap();
        indices.push(index);

        let path = &["f".into(), "y".into(), PathElement::Length];
        let index = Bar::generalized_index(path).unwrap();
        indices.push(index);

        assert_eq!(indices, [1, 16, 3, 4, 12, 5634, 23])
    }
}
