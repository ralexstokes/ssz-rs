use crate::merkleization::{multiproofs::GeneralizedIndex, BYTES_PER_CHUNK};

pub trait IndexedPath {}

pub struct Done;

impl IndexedPath for Done {}

pub trait Indexed {
    type Path: IndexedPath;

    fn item_length() -> usize {
        BYTES_PER_CHUNK
    }

    fn chunk_count() -> usize {
        1
    }

    // Compute the generalized index starting from the `root` index and following `path`.
    fn generalized_index(root: GeneralizedIndex, path: &Self::Path) -> GeneralizedIndex;
}
