use crate::{
    lib::*,
    merkleization::{multiproofs::GeneralizedIndex, BYTES_PER_CHUNK},
};

#[derive(Debug, Clone)]
pub enum PathElement {
    Index(usize),
    Field(&'static str),
    Length,
}

#[derive(Debug)]
pub enum PathError {
    Type(PathElement),
    EmptyPath,
}

impl Display for PathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type(elem) => write!(f, "invalid path element {elem:?} when walking type"),
            Self::EmptyPath => write!(f, ""),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PathError {}

pub trait Indexed {
    fn item_length() -> usize {
        BYTES_PER_CHUNK
    }

    fn chunk_count() -> usize {
        1
    }

    // Compute the generalized index starting from the `root` index and following `path`.
    fn generalized_index(
        root: GeneralizedIndex,
        path: &[PathElement],
    ) -> Result<GeneralizedIndex, PathError>;
}
