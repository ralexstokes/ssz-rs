use crate::error::{InstanceError, TypeError};
use crate::ser::BYTES_PER_LENGTH_OFFSET;
use crate::SimpleSerialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("expected at least {expected} bytes when decoding but provided only {provided} bytes")]
    ExpectedFurtherInput { provided: usize, expected: usize },
    #[error("{provided} bytes given but only expected {expected} bytes")]
    AdditionalInput { provided: usize, expected: usize },
    #[error("invalid byte {0:x} when decoding data of the expected type")]
    InvalidByte(u8),
    #[error("invalid instance: {0}")]
    InvalidInstance(#[from] InstanceError),
    #[error("invalid type: {0}")]
    InvalidType(#[from] TypeError),
}

pub trait Deserialize {
    fn deserialize(encoding: &[u8]) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

fn deserialize_fixed_homogeneous_composite<T>(encoding: &[u8]) -> Result<Vec<T>, DeserializeError>
where
    T: SimpleSerialize,
{
    let remainder = encoding.len() % T::size_hint();
    if remainder != 0 {
        return Err(DeserializeError::AdditionalInput {
            provided: encoding.len(),
            expected: encoding.len() - remainder,
        });
    }

    let mut elements = vec![];
    for chunk in encoding.chunks_exact(T::size_hint()) {
        let element = T::deserialize(chunk)?;
        elements.push(element);
    }
    Ok(elements)
}

fn deserialize_variable_homogeneous_composite<T>(
    encoding: &[u8],
) -> Result<Vec<T>, DeserializeError>
where
    T: SimpleSerialize,
{
    if encoding.is_empty() {
        return Ok(vec![]);
    }

    let data_pointer = u32::deserialize(&encoding[..BYTES_PER_LENGTH_OFFSET])?;
    let data_pointer = data_pointer as usize;
    if encoding.len() < data_pointer {
        return Err(DeserializeError::ExpectedFurtherInput {
            provided: encoding.len(),
            expected: data_pointer,
        });
    }

    let offsets = &mut encoding[..data_pointer]
        .chunks_exact(BYTES_PER_LENGTH_OFFSET)
        .map(|chunk| u32::deserialize(chunk).map(|offset| offset as usize))
        .collect::<Result<Vec<usize>, DeserializeError>>()?;
    offsets.push(encoding.len());

    let element_count = data_pointer / BYTES_PER_LENGTH_OFFSET;
    let mut result = Vec::with_capacity(element_count);
    for span in offsets.windows(2) {
        let start = span[0];
        let end = span[1];
        let element = T::deserialize(&encoding[start..end])?;
        result.push(element);
    }
    Ok(result)
}

pub fn deserialize_homogeneous_composite<T>(encoding: &[u8]) -> Result<Vec<T>, DeserializeError>
where
    T: SimpleSerialize,
{
    if T::is_variable_size() {
        deserialize_variable_homogeneous_composite(encoding)
    } else {
        deserialize_fixed_homogeneous_composite(encoding)
    }
}
