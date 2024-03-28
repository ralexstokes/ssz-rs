use crate::merkleization::MerkleizationError as Error;

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
