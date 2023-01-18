use crate::de::DeserializeError;
use crate::list::Error as ListError;
use crate::merkleization::MerkleizationError;
use crate::ser::SerializeError;
use crate::vector::Error as VectorError;
use thiserror::Error;

// Top-level error to wrap all child errors in crate
#[derive(Debug, Error)]
#[error("{0}")]
pub enum Error {
    Serialize(#[from] SerializeError),
    Deserialize(#[from] DeserializeError),
    Merkleization(#[from] MerkleizationError),
    List(#[from] ListError),
    Vector(#[from] VectorError),
}
