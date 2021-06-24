use crate::ssz::SSZ;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("expected further data when decoding")]
    InputTooShort,
    #[error("invalid data for expected type")]
    InvalidInput,
}

pub trait Deserialize {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

fn deserialize_fixed_homogeneous_composite<T>(
    encoding: &[u8],
    chunk_size: usize,
) -> Result<Vec<T>, DeserializeError>
where
    T: SSZ,
{
    encoding
        .chunks_exact(chunk_size)
        .map(|chunk| T::deserialize(chunk))
        .collect()
}

fn deserialize_variable_homogeneous_composite<T>(
    encoding: &[u8],
) -> Result<Vec<T>, DeserializeError>
where
    T: SSZ,
{
    unimplemented!()
}

pub fn deserialize_homogeneous_composite<T>(
    encoding: &[u8],
    chunk_size: usize,
) -> Result<Vec<T>, DeserializeError>
where
    T: SSZ,
{
    if T::is_variable_size() {
        deserialize_variable_homogeneous_composite(encoding)
    } else {
        deserialize_fixed_homogeneous_composite(encoding, chunk_size)
    }
}
