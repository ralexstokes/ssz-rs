use crate::ser::BYTES_PER_LENGTH_OFFSET;
use crate::SimpleSerialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("expected further data when decoding")]
    InputTooShort,
    #[error("invalid data for expected type")]
    InvalidInput,
    #[error("{0}")]
    IOError(#[from] std::io::Error),
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
    encoding
        .chunks_exact(T::size_hint())
        .map(|chunk| T::deserialize(chunk))
        .collect()
}

fn deserialize_variable_homogeneous_composite<T>(
    encoding: &[u8],
) -> Result<Vec<T>, DeserializeError>
where
    T: SimpleSerialize,
{
    let data_pointer = u32::deserialize(&encoding[..BYTES_PER_LENGTH_OFFSET])? as usize;
    if encoding.len() < data_pointer {
        return Err(DeserializeError::InputTooShort);
    }

    let offsets = &mut encoding[..data_pointer]
        .chunks_exact(BYTES_PER_LENGTH_OFFSET)
        .map(|chunk| u32::deserialize(chunk).map(|offset| offset as usize))
        .collect::<Result<Vec<usize>, DeserializeError>>()?;
    offsets.push(encoding.len());

    let element_count = data_pointer as usize / BYTES_PER_LENGTH_OFFSET;
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
