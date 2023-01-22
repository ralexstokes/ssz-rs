use crate::de::DeserializeError;
use crate::merkleization::MerkleizationError;
use crate::ser::SerializeError;
use thiserror::Error;

// Top-level error to wrap all child errors in crate
#[derive(Debug, Error)]
pub enum Error {
    #[error("could not serialize: {0}")]
    Serialize(#[from] SerializeError),
    #[error("could not deserialize: {0}")]
    Deserialize(#[from] DeserializeError),
    #[error("merkleization error: {0}")]
    Merkleization(#[from] MerkleizationError),
    #[error("{0}")]
    Bounds(#[from] BoundsError),
}

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("the type for this value has a bound of {bound} but the value has {len} elements")]
    BoundsViolated { bound: usize, len: usize },
    #[error("the type for this value is not valid SSZ with bound {0}")]
    Invalid(usize),
}

#[derive(Error, Debug)]
pub enum BoundsError {
    #[error(
        "{provided} elements provided that exceed the expected bound {expected} for this type"
    )]
    ExcessElements { expected: usize, provided: usize },
}
