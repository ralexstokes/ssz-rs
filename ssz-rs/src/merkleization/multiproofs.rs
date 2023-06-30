use crate::merkleization::BYTES_PER_CHUNK;

pub type GeneralizedIndex = usize;

pub type IndexPath<Continuation> = (usize, Continuation);

pub struct Done;

pub trait Indexed {
    type Path;

    fn item_length() -> usize {
        BYTES_PER_CHUNK
    }

    fn chunk_count() -> usize {
        1
    }

    // Compute the generalized index starting from the `root` index and following `path`.
    fn generalized_index(root: GeneralizedIndex, path: &Self::Path) -> GeneralizedIndex;
}

#[derive(Default)]
pub struct Proof {
    _leaves: Vec<crate::merkleization::Node>,
}

pub fn get_power_of_two_ceil(x: usize) -> usize {
    match x {
        x if x <= 1 => 1,
        2 => 2,
        x => 2 * get_power_of_two_ceil((x + 1) / 2),
    }
}
