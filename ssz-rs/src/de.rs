use crate::ser::BYTES_PER_LENGTH_OFFSET;
use crate::SimpleSerialize;
use crate::std::{Vec, vec, Debug, Display, Formatter};

#[derive(Debug)]
pub enum DeserializeError {
    InputTooShort,
    ExtraInput,
    InvalidInput,
    IOError,
    TypeBoundsViolated { bound: usize, len: usize },
    IllegalType { bound: usize },
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            DeserializeError::InputTooShort => write!(f, "expected further data when decoding"),
            DeserializeError::ExtraInput => write!(f, "unexpected additional data provided when decoding"),
            DeserializeError::InvalidInput => write!(f, "invalid data for expected type"),
            DeserializeError::IOError => write!(f, "IOError"),
            DeserializeError::TypeBoundsViolated { bound, len } => write!(f, "the type for this value has a bound of {} but the value has {} elements", bound, len),
            DeserializeError::IllegalType { bound } => write!(f, "the type for this value has an illegal bound of {}", bound),
        }
    }
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
    if encoding.len() % T::size_hint() != 0 {
        return Err(DeserializeError::InvalidInput);
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
