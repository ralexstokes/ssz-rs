use crate::{
    error::{InstanceError, TypeError},
    lib::*,
    ser::BYTES_PER_LENGTH_OFFSET,
    SimpleSerialize,
};

/// Deserialization errors.
#[derive(Debug)]
pub enum DeserializeError {
    /// More data was expected to be in the buffer.
    ExpectedFurtherInput {
        provided: usize,
        expected: usize,
    },
    /// The buffer contained more data than expected.
    AdditionalInput {
        provided: usize,
        expected: usize,
    },
    InvalidByte(u8),
    /// An invalid instance was encountered.
    InvalidInstance(InstanceError),
    /// An invalid type was encountered.
    InvalidType(TypeError),
}

impl From<InstanceError> for DeserializeError {
    fn from(err: InstanceError) -> Self {
        Self::InvalidInstance(err)
    }
}

impl From<TypeError> for DeserializeError {
    fn from(err: TypeError) -> Self {
        Self::InvalidType(err)
    }
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DeserializeError::ExpectedFurtherInput { provided, expected } => write!(f, "expected at least {expected} bytes when decoding but provided only {provided} bytes"),
            DeserializeError::AdditionalInput { provided, expected } => write!(f, "{provided} bytes given but only expected {expected} bytes"),
            DeserializeError::InvalidByte(b) => write!(
                f,
                "invalid byte {b:x} when decoding data of the expected type"
            ),
            DeserializeError::InvalidInstance(err) => write!(f, "invalid instance: {err}"),
            DeserializeError::InvalidType(err) => write!(f, "invalid type: {err}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DeserializeError {}

/// A data structure that can be deserialized using SSZ.
pub trait Deserialize {
    /// Deserialize this value from the given SSZ-encoded buffer.
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
            // SAFETY: checked subtraction is unnecessary, as encoding.len() > remainder; qed
            expected: encoding.len() - remainder,
        })
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
        return Ok(vec![])
    }

    let data_pointer = u32::deserialize(&encoding[..BYTES_PER_LENGTH_OFFSET])?;
    let data_pointer = data_pointer as usize;
    if encoding.len() < data_pointer {
        return Err(DeserializeError::ExpectedFurtherInput {
            provided: encoding.len(),
            expected: data_pointer,
        })
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
