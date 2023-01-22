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
}

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("the type for this value is invalid with bound {0}")]
    InvalidBound(usize),
}

#[derive(Error, Debug)]
pub enum InstanceError {
    #[error("required {required} elements for this type but {provided} elements given")]
    Exact { required: usize, provided: usize },
    #[error("{provided} elements given for a type with (inclusive) upper bound {bound}")]
    Bounded { bound: usize, provided: usize },
}
