use crate::{de::DeserializeError, lib::*, merkleization::MerkleizationError, ser::SerializeError};

// Top-level error to wrap all child errors in crate
#[derive(Debug)]
pub enum Error {
    Serialize(SerializeError),
    Deserialize(DeserializeError),
    Merkleization(MerkleizationError),
}

impl From<SerializeError> for Error {
    fn from(err: SerializeError) -> Self {
        Self::Serialize(err)
    }
}

impl From<DeserializeError> for Error {
    fn from(err: DeserializeError) -> Self {
        Self::Deserialize(err)
    }
}

impl From<MerkleizationError> for Error {
    fn from(err: MerkleizationError) -> Self {
        Self::Merkleization(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialize(err) => write!(f, "could not serialize: {err}"),
            Self::Deserialize(err) => write!(f, "could not deserialize: {err}"),
            Self::Merkleization(err) => write!(f, "merkleization error: {err}"),
        }
    }
}

#[derive(Debug)]
pub enum TypeError {
    InvalidBound(usize),
}

impl Display for TypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBound(size) => {
                write!(f, "the type for this value is invalid with bound {size}")
            }
        }
    }
}

#[derive(Debug)]
pub enum InstanceError {
    Exact { required: usize, provided: usize },
    Bounded { bound: usize, provided: usize },
}

impl Display for InstanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact { required, provided } => write!(
                f,
                "required {required} elements for this type but {provided} elements given"
            ),
            Self::Bounded { bound, provided } => write!(
                f,
                "{provided} elements given for a type with (inclusive) upper bound {bound}"
            ),
        }
    }
}
